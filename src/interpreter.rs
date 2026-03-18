use crate::ast::{BinaryOpKind, Expr, Pattern, Program, Stmt, StmtKind, Type, UnaryOpKind};
use std::cell::RefCell;
use std::collections::HashMap;
use std::io::{self, BufRead};
use std::rc::Rc;

thread_local! {
    static OUTPUT_BUFFER: RefCell<Option<String>> = const { RefCell::new(None) };
}

pub fn capture_start() {
    OUTPUT_BUFFER.with(|b| {
        *b.borrow_mut() = Some(String::new());
    });
}

pub fn capture_flush() -> String {
    OUTPUT_BUFFER.with(|b| b.borrow_mut().take().unwrap_or_default())
}


pub struct Interpreter {
    pub env: Environment,
}

impl Default for Interpreter {
    fn default() -> Self {
        Self::new()
    }
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            env: Environment::new(),
        }
    }

    pub fn interpret(&mut self, program: &Program) -> Result<(), RuntimeError> {
        for stmt in &program.stmts {
            let signal = eval_stmt(stmt, &mut self.env)?;
            if let Some(Signal::Return(_)) = signal {
                break;
            }
        }
        Ok(())
    }

    pub fn eval_program(&mut self, program: &Program) -> Result<(), RuntimeError> {
        for stmt in &program.stmts {
            let signal = eval_stmt(stmt, &mut self.env)?;
            if let Some(Signal::Return(_)) = signal {
                break;
            }
        }
        Ok(())
    }
}

fn output_line(s: &str) {
    OUTPUT_BUFFER.with(|b| {
        if let Some(buf) = b.borrow_mut().as_mut() {
            buf.push_str(s);
            buf.push('\n');
        } else {
            println!("{}", s);
        }
    });
}

#[derive(Debug, Clone)]
pub enum Value {
    Int(i64),
    Float(f64),
    Str(String),
    Bool(bool),
    Void,
    Function {
        params: Vec<(String, Type)>,
        body: Vec<Stmt>,
    },
    Closure {
        params: Vec<(String, Option<Type>)>,
        body: Vec<Stmt>,
        captured: Vec<(String, Value)>,
    },
    Array(Rc<RefCell<Vec<Value>>>),
    Struct {
        name: String,
        fields: Rc<RefCell<HashMap<String, Value>>>,
    },
    Tuple(Vec<Value>),
    Map(Rc<RefCell<Vec<(Value, Value)>>>),
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Value::Int(n) => write!(f, "{}", n),
            Value::Float(n) => write!(f, "{}", n),
            Value::Str(s) => write!(f, "{}", s),
            Value::Bool(b) => write!(f, "{}", if *b { "kin" } else { "ala" }),
            Value::Void => write!(f, "weka"),
            Value::Function { .. } => write!(f, "<pali>"),
            Value::Closure { .. } => write!(f, "<lambda>"),
            Value::Array(arr) => {
                let arr = arr.borrow();
                let items: Vec<String> = arr.iter().map(|v| v.to_string()).collect();
                write!(f, "[{}]", items.join(", "))
            }
            Value::Tuple(vals) => {
                let items: Vec<String> = vals.iter().map(|v| v.to_string()).collect();
                write!(f, "({})", items.join(", "))
            }
            Value::Map(entries) => {
                let entries = entries.borrow();
                let items: Vec<String> = entries
                    .iter()
                    .map(|(k, v)| format!("{}: {}", k, v))
                    .collect();
                write!(f, "{{{}}}", items.join(", "))
            }
            Value::Struct { name, fields } => {
                let fields = fields.borrow();
                let mut pairs: Vec<String> = fields
                    .iter()
                    .map(|(k, v)| format!("{}: {}", k, v))
                    .collect();
                pairs.sort();
                write!(f, "{} {{ {} }}", name, pairs.join(", "))
            }
        }
    }
}

#[derive(Debug)]
pub struct RuntimeError {
    pub message: String,
    pub line: usize,
    pub stack_trace: Vec<String>,
}

impl RuntimeError {
    pub fn new(msg: impl Into<String>, line: usize) -> Self {
        Self {
            message: msg.into(),
            line,
            stack_trace: Vec::new(),
        }
    }

    pub fn with_frame(mut self, frame: String) -> Self {
        self.stack_trace.push(frame);
        self
    }
}

#[derive(Default)]
pub struct Environment {
    store: HashMap<String, Value>,
    outer: Option<Box<Environment>>,
}

impl Environment {
    pub fn new() -> Self {
        Self::default()
    }

    #[cfg(test)]
    pub fn new_enclosed(outer: Environment) -> Self {
        Self {
            store: HashMap::new(),
            outer: Some(Box::new(outer)),
        }
    }

    pub fn get(&self, name: &str) -> Option<Value> {
        match self.store.get(name) {
            Some(v) => Some(v.clone()),
            None => self.outer.as_ref()?.get(name),
        }
    }

    pub fn set(&mut self, name: String, val: Value) {
        self.store.insert(name, val);
    }

    pub fn update(&mut self, name: &str, val: Value) -> bool {
        if self.store.contains_key(name) {
            self.store.insert(name.to_string(), val);
            true
        } else if let Some(outer) = &mut self.outer {
            outer.update(name, val)
        } else {
            false
        }
    }

    pub fn collect_functions(&self) -> Vec<(String, Value)> {
        let mut funcs: Vec<(String, Value)> = self
            .store
            .iter()
            .filter(|(_, v)| matches!(v, Value::Function { .. }))
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect();
        if let Some(outer) = &self.outer {
            for (k, v) in outer.collect_functions() {
                if !funcs.iter().any(|(name, _)| name == &k) {
                    funcs.push((k, v));
                }
            }
        }
        funcs
    }

    pub fn snapshot(&self) -> Vec<(String, Value)> {
        let mut all: Vec<(String, Value)> = self
            .store
            .iter()
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect();
        if let Some(outer) = &self.outer {
            for (k, v) in outer.snapshot() {
                if !all.iter().any(|(name, _)| name == &k) {
                    all.push((k, v));
                }
            }
        }
        all
    }
}

pub enum Signal {
    Return(Value),
    Break,
    Continue,
}

pub fn eval_expr(expr: &Expr, env: &mut Environment, line: usize) -> Result<Value, RuntimeError> {
    match expr {
        Expr::IntLiteral(n) => Ok(Value::Int(*n)),
        Expr::FloatLiteral(f) => Ok(Value::Float(*f)),
        Expr::StringLiteral(s) => Ok(Value::Str(s.clone())),
        Expr::BoolLiteral(b) => Ok(Value::Bool(*b)),
        Expr::NullLiteral => Ok(Value::Void),

        Expr::Identifier(name) => env
            .get(name)
            .ok_or_else(|| RuntimeError::new(format!("Undefined variable: {}", name), line)),

        Expr::Assign { name, value } => {
            let val = eval_expr(value, env, line)?;
            if !env.update(name, val.clone()) {
                env.set(name.clone(), val.clone());
            }
            Ok(val)
        }

        Expr::BinaryOp { op, left, right } => {
            let lv = eval_expr(left, env, line)?;
            let rv = eval_expr(right, env, line)?;
            eval_binary_op(op, lv, rv, line)
        }

        Expr::UnaryOp { op, expr } => {
            let val = eval_expr(expr, env, line)?;
            match op {
                UnaryOpKind::Neg => match val {
                    Value::Int(n) => Ok(Value::Int(-n)),
                    Value::Float(f) => Ok(Value::Float(-f)),
                    _ => Err(RuntimeError::new("Unary negation only for numbers", line)),
                },
                UnaryOpKind::Not => match val {
                    Value::Bool(b) => Ok(Value::Bool(!b)),
                    _ => Err(RuntimeError::new("Logical NOT only for boolean", line)),
                },
            }
        }

        Expr::Call { name, args } => {
            if name == "toki" {
                let mut parts = Vec::new();
                for arg in args {
                    let v = eval_expr(arg, env, line)?;
                    parts.push(v.to_string());
                }
                output_line(&parts.join(" "));
                return Ok(Value::Void);
            }

            if name == "kute" {
                let stdin = io::stdin();
                let mut buf = String::new();
                stdin
                    .lock()
                    .read_line(&mut buf)
                    .map_err(|e| RuntimeError::new(format!("Input error: {}", e), line))?;
                return Ok(Value::Str(buf.trim_end_matches('\n').to_string()));
            }

            if let Some(result) = eval_builtin_math(name, args, env, line)? {
                return Ok(result);
            }

            if let Some(result) = eval_builtin_stdlib(name, args, env, line)? {
                return Ok(result);
            }

            if name == "lipu" {
                let mut pairs = Vec::new();
                let mut i = 0;
                while i + 1 < args.len() {
                    let key = eval_expr(&args[i], env, line)?;
                    let val = eval_expr(&args[i + 1], env, line)?;
                    pairs.push((key, val));
                    i += 2;
                }
                return Ok(Value::Map(Rc::new(RefCell::new(pairs))));
            }

            if let Some(result) = eval_builtin_io(name, args, env, line)? {
                return Ok(result);
            }

            let func_val = env
                .get(name)
                .ok_or_else(|| RuntimeError::new(format!("Undefined function: {}", name), line))?;

            match func_val {
                Value::Function { params, body } => {
                    if args.len() != params.len() {
                        return Err(RuntimeError::new(
                            format!(
                                "Function '{}': Argument count mismatch (expected {}, got {})",
                                name,
                                params.len(),
                                args.len()
                            ),
                            line,
                        ));
                    }

                    let mut arg_vals = Vec::new();
                    for arg in args {
                        arg_vals.push(eval_expr(arg, env, line)?);
                    }

                    let mut func_env = Environment::new();
                    for (k, v) in env.snapshot() {
                        func_env.set(k, v);
                    }
                    for ((param_name, _ty), val) in params.iter().zip(arg_vals) {
                        func_env.set(param_name.clone(), val);
                    }

                    match eval_block(&body, &mut func_env) {
                        Ok(Some(Signal::Return(v))) => Ok(v),
                        Ok(_) => Ok(Value::Void),
                        Err(e) => {
                            Err(e.with_frame(format!("  Function '{}' (line {})", name, line)))
                        }
                    }
                }
                Value::Closure {
                    params,
                    body,
                    captured,
                } => {
                    if args.len() != params.len() {
                        return Err(RuntimeError::new(
                            format!(
                                "Lambda '{}': Argument count mismatch (expected {}, got {})",
                                name,
                                params.len(),
                                args.len()
                            ),
                            line,
                        ));
                    }
                    let mut arg_vals = Vec::new();
                    for arg in args {
                        arg_vals.push(eval_expr(arg, env, line)?);
                    }
                    let mut closure_env = Environment::new();
                    for (k, v) in &captured {
                        closure_env.set(k.clone(), v.clone());
                    }
                    for ((param_name, _), val) in params.iter().zip(arg_vals) {
                        closure_env.set(param_name.clone(), val);
                    }
                    match eval_block(&body, &mut closure_env)? {
                        Some(Signal::Return(v)) => Ok(v),
                        _ => Ok(Value::Void),
                    }
                }
                _ => Err(RuntimeError::new(
                    format!("'{}' is not a function", name),
                    line,
                )),
            }
        }

        Expr::ArrayLiteral(elems) => {
            let mut vals = Vec::new();
            for e in elems {
                vals.push(eval_expr(e, env, line)?);
            }
            Ok(Value::Array(Rc::new(RefCell::new(vals))))
        }

        Expr::Index { object, index } => {
            let obj = eval_expr(object, env, line)?;
            let idx = eval_expr(index, env, line)?;
            match (obj, idx) {
                (Value::Array(arr), Value::Int(i)) => {
                    let arr = arr.borrow();
                    let len = arr.len() as i64;
                    let i = if i < 0 { len + i } else { i };
                    if i < 0 || i >= len {
                        Err(RuntimeError::new(
                            format!("Index out of bounds: {} (suli_ijo {})", i, len),
                            line,
                        ))
                    } else {
                        Ok(arr[i as usize].clone())
                    }
                }
                (Value::Str(s), Value::Int(i)) => {
                    let chars: Vec<char> = s.chars().collect();
                    let len = chars.len() as i64;
                    let i = if i < 0 { len + i } else { i };
                    if i < 0 || i >= len {
                        Err(RuntimeError::new(
                            format!("String Index out of bounds: {}", i),
                            line,
                        ))
                    } else {
                        Ok(Value::Str(chars[i as usize].to_string()))
                    }
                }
                (Value::Map(map), key) => {
                    let map = map.borrow();
                    for (k, v) in map.iter() {
                        if values_equal(k, &key) {
                            return Ok(v.clone());
                        }
                    }
                    Err(RuntimeError::new(
                        format!("Key not found in map: {}", key),
                        line,
                    ))
                }
                _ => Err(RuntimeError::new(
                    "Indexing not supported for this type",
                    line,
                )),
            }
        }

        Expr::IndexAssign {
            object,
            index,
            value,
        } => {
            let obj = eval_expr(object, env, line)?;
            let idx = eval_expr(index, env, line)?;
            let val = eval_expr(value, env, line)?;
            match (obj, idx) {
                (Value::Array(arr), Value::Int(i)) => {
                    let mut arr = arr.borrow_mut();
                    let len = arr.len() as i64;
                    let i = if i < 0 { len + i } else { i };
                    if i < 0 || i >= len {
                        return Err(RuntimeError::new(
                            format!("Index out of bounds: {}", i),
                            line,
                        ));
                    }
                    arr[i as usize] = val.clone();
                    Ok(val)
                }
                (Value::Map(map), key) => {
                    let mut map = map.borrow_mut();
                    for entry in map.iter_mut() {
                        if values_equal(&entry.0, &key) {
                            entry.1 = val.clone();
                            return Ok(val);
                        }
                    }
                    map.push((key, val.clone()));
                    Ok(val)
                }
                _ => Err(RuntimeError::new(
                    "Index assign: Array or Map required",
                    line,
                )),
            }
        }

        Expr::MethodCall {
            object,
            method,
            args,
        } => {
            let obj = eval_expr(object, env, line)?;
            eval_method(obj, method, args, env, line)
        }

        Expr::FieldAccess { object, field } => {
            let obj = eval_expr(object, env, line)?;
            match obj {
                Value::Struct { fields, .. } => {
                    fields.borrow().get(field.as_str()).cloned().ok_or_else(|| {
                        RuntimeError::new(format!("Undefined field: {}", field), line)
                    })
                }
                _ => Err(RuntimeError::new("Field access: Struct required", line)),
            }
        }

        Expr::FieldAssign {
            object,
            field,
            value,
        } => {
            let obj = eval_expr(object, env, line)?;
            let val = eval_expr(value, env, line)?;
            match obj {
                Value::Struct { fields, .. } => {
                    fields.borrow_mut().insert(field.clone(), val.clone());
                    Ok(val)
                }
                _ => Err(RuntimeError::new("Field assign: Struct required", line)),
            }
        }

        Expr::StructLiteral { name, fields } => {
            let mut map = HashMap::new();
            for (fname, fexpr) in fields {
                map.insert(fname.clone(), eval_expr(fexpr, env, line)?);
            }
            Ok(Value::Struct {
                name: name.clone(),
                fields: Rc::new(RefCell::new(map)),
            })
        }

        Expr::Lambda { params, body } => {
            let captured = env.snapshot();
            Ok(Value::Closure {
                params: params.clone(),
                body: body.clone(),
                captured,
            })
        }

        Expr::Range { start, end } => {
            let s = match eval_expr(start, env, line)? {
                Value::Int(n) => n,
                _ => return Err(RuntimeError::new("Range: Integer required", line)),
            };
            let e = match eval_expr(end, env, line)? {
                Value::Int(n) => n,
                _ => return Err(RuntimeError::new("Range: Integer required", line)),
            };
            let vals: Vec<Value> = (s..e).map(Value::Int).collect();
            Ok(Value::Array(Rc::new(RefCell::new(vals))))
        }

        Expr::TupleLiteral(elems) => {
            let mut vals = Vec::new();
            for e in elems {
                vals.push(eval_expr(e, env, line)?);
            }
            Ok(Value::Tuple(vals))
        }

        Expr::TupleIndex { object, index } => {
            let obj = eval_expr(object, env, line)?;
            match obj {
                Value::Tuple(vals) => {
                    if *index >= vals.len() {
                        Err(RuntimeError::new(
                            format!(
                                "Tuple Index out of bounds: {} (suli_ijo {})",
                                index,
                                vals.len()
                            ),
                            line,
                        ))
                    } else {
                        Ok(vals[*index].clone())
                    }
                }
                _ => Err(RuntimeError::new("Tuple indexing: Tuple required", line)),
            }
        }

        Expr::MapLiteral(entries) => {
            let mut pairs = Vec::new();
            for (k, v) in entries {
                let key = eval_expr(k, env, line)?;
                let val = eval_expr(v, env, line)?;
                pairs.push((key, val));
            }
            Ok(Value::Map(Rc::new(RefCell::new(pairs))))
        }
    }
}

fn eval_method(
    obj: Value,
    method: &str,
    args: &[Expr],
    env: &mut Environment,
    line: usize,
) -> Result<Value, RuntimeError> {
    let arg_vals: Vec<Value> = args
        .iter()
        .map(|a| eval_expr(a, env, line))
        .collect::<Result<_, _>>()?;

    match obj {
        Value::Array(arr) => match method {
            "sin_ijo" => {
                let val = arg_vals
                    .into_iter()
                    .next()
                    .ok_or_else(|| RuntimeError::new("Add: Argument(s) required", line))?;
                arr.borrow_mut().push(val);
                Ok(Value::Void)
            }
            "pakala" => {
                let idx = match arg_vals.first() {
                    Some(Value::Int(i)) => *i,
                    _ => return Err(RuntimeError::new("Remove: Integer index required", line)),
                };
                let mut arr = arr.borrow_mut();
                let len = arr.len() as i64;
                let idx = if idx < 0 { len + idx } else { idx };
                if idx < 0 || idx >= len {
                    return Err(RuntimeError::new(
                        format!("Remove: Index out of bounds {}", idx),
                        line,
                    ));
                }
                Ok(arr.remove(idx as usize))
            }
            "suli_ijo" => Ok(Value::Int(arr.borrow().len() as i64)),
            "jo" => {
                let val = arg_vals
                    .first()
                    .ok_or_else(|| RuntimeError::new("Contains: Argument(s) required", line))?;
                let found = arr.borrow().iter().any(|v| values_equal(v, val));
                Ok(Value::Bool(found))
            }
            "monsi" => {
                let mut v = arr.borrow().clone();
                v.reverse();
                Ok(Value::Array(Rc::new(RefCell::new(v))))
            }
            "nasin_ijo" => {
                let mut v = arr.borrow().clone();
                v.sort_by(|a, b| match (a, b) {
                    (Value::Int(x), Value::Int(y)) => x.cmp(y),
                    (Value::Float(x), Value::Float(y)) => {
                        x.partial_cmp(y).unwrap_or(std::cmp::Ordering::Equal)
                    }
                    (Value::Str(x), Value::Str(y)) => x.cmp(y),
                    _ => std::cmp::Ordering::Equal,
                });
                Ok(Value::Array(Rc::new(RefCell::new(v))))
            }
            "wan" => {
                let sep = match arg_vals.first() {
                    Some(Value::Str(s)) => s.clone(),
                    _ => "".to_string(),
                };
                let parts: Vec<String> = arr.borrow().iter().map(|v| v.to_string()).collect();
                Ok(Value::Str(parts.join(&sep)))
            }
            _ => Err(RuntimeError::new(
                format!("Array method not found: {}", method),
                line,
            )),
        },
        Value::Str(s) => match method {
            "suli_ijo" => Ok(Value::Int(s.chars().count() as i64)),
            "tu" => {
                let sep = match arg_vals.first() {
                    Some(Value::Str(sep)) => sep.clone(),
                    _ => " ".to_string(),
                };
                let parts: Vec<Value> = s
                    .split(sep.as_str())
                    .map(|p| Value::Str(p.to_string()))
                    .collect();
                Ok(Value::Array(Rc::new(RefCell::new(parts))))
            }
            "jo" => {
                let needle = match arg_vals.first() {
                    Some(Value::Str(n)) => n.clone(),
                    _ => {
                        return Err(RuntimeError::new(
                            "Contains: String argument required",
                            line,
                        ));
                    }
                };
                Ok(Value::Bool(s.contains(needle.as_str())))
            }
            "ante_ijo" => {
                let from = match arg_vals.first() {
                    Some(Value::Str(f)) => f.clone(),
                    _ => {
                        return Err(RuntimeError::new(
                            "Replace: String Argument(s) required",
                            line,
                        ));
                    }
                };
                let to = match arg_vals.get(1) {
                    Some(Value::Str(t)) => t.clone(),
                    _ => {
                        return Err(RuntimeError::new(
                            "Replace: String Argument(s) required",
                            line,
                        ));
                    }
                };
                Ok(Value::Str(s.replace(from.as_str(), to.as_str())))
            }
            "pona_ijo" => Ok(Value::Str(s.trim().to_string())),
            "suli_sitelen" => Ok(Value::Str(s.to_uppercase())),
            "lili_sitelen" => Ok(Value::Str(s.to_lowercase())),
            "open_sitelen" => {
                let prefix = match arg_vals.first() {
                    Some(Value::Str(p)) => p.clone(),
                    _ => return Err(RuntimeError::new("Start: String argument required", line)),
                };
                Ok(Value::Bool(s.starts_with(prefix.as_str())))
            }
            "pini_sitelen" => {
                let suffix = match arg_vals.first() {
                    Some(Value::Str(p)) => p.clone(),
                    _ => return Err(RuntimeError::new("End: String argument required", line)),
                };
                Ok(Value::Bool(s.ends_with(suffix.as_str())))
            }
            _ => Err(RuntimeError::new(
                format!("String method not found: {}", method),
                line,
            )),
        },
        Value::Map(map) => match method {
            "nimi_ale" => {
                let keys: Vec<Value> = map.borrow().iter().map(|(k, _)| k.clone()).collect();
                Ok(Value::Array(Rc::new(RefCell::new(keys))))
            }
            "ijo_ale" => {
                let vals: Vec<Value> = map.borrow().iter().map(|(_, v)| v.clone()).collect();
                Ok(Value::Array(Rc::new(RefCell::new(vals))))
            }
            "suli_ijo" => Ok(Value::Int(map.borrow().len() as i64)),
            "jo" => {
                let key = arg_vals
                    .first()
                    .ok_or_else(|| RuntimeError::new("Contains: Key argument required", line))?;
                let found = map.borrow().iter().any(|(k, _)| values_equal(k, key));
                Ok(Value::Bool(found))
            }
            "pakala" => {
                let key = arg_vals
                    .first()
                    .ok_or_else(|| RuntimeError::new("Remove: Key argument required", line))?;
                let mut map = map.borrow_mut();
                let pos = map.iter().position(|(k, _)| values_equal(k, key));
                if let Some(i) = pos {
                    let (_, v) = map.remove(i);
                    Ok(v)
                } else {
                    Err(RuntimeError::new("Remove: Key not found", line))
                }
            }
            _ => Err(RuntimeError::new(
                format!("Map method not found: {}", method),
                line,
            )),
        },
        Value::Struct {
            name: struct_name,
            fields,
        } => {
            let method_key = format!("{}::{}", struct_name, method);
            if let Some(func_val) = env.get(&method_key) {
                match func_val {
                    Value::Function { params, body } => {
                        let mut method_env = Environment::new();
                        for (fname, fval) in env.collect_functions() {
                            method_env.set(fname, fval);
                        }
                        method_env.set(
                            "mi".to_string(),
                            Value::Struct {
                                name: struct_name.to_string(),
                                fields: fields.clone(),
                            },
                        );
                        let non_self_params: Vec<_> =
                            params.iter().filter(|(n, _)| n != "mi").collect();
                        for ((pname, _), val) in non_self_params.iter().zip(arg_vals) {
                            method_env.set(pname.clone(), val);
                        }
                        match eval_block(&body, &mut method_env)? {
                            Some(Signal::Return(v)) => Ok(v),
                            _ => Ok(Value::Void),
                        }
                    }
                    _ => Err(RuntimeError::new(
                        format!("'{}' is not a function", method_key),
                        line,
                    )),
                }
            } else {
                Err(RuntimeError::new(
                    format!("Struct '{}' has no method '{}'", struct_name, method),
                    line,
                ))
            }
        }
        _ => Err(RuntimeError::new(
            format!("Method '{}' Not callable for this type", method),
            line,
        )),
    }
}

fn json_to_value(json: &serde_json::Value) -> Value {
    match json {
        serde_json::Value::Null => Value::Void,
        serde_json::Value::Bool(b) => Value::Bool(*b),
        serde_json::Value::Number(n) => {
            if let Some(i) = n.as_i64() {
                Value::Int(i)
            } else {
                Value::Float(n.as_f64().unwrap_or(0.0))
            }
        }
        serde_json::Value::String(s) => Value::Str(s.clone()),
        serde_json::Value::Array(arr) => {
            let vals: Vec<Value> = arr.iter().map(json_to_value).collect();
            Value::Array(Rc::new(RefCell::new(vals)))
        }
        serde_json::Value::Object(map) => {
            let pairs: Vec<(Value, Value)> = map
                .iter()
                .map(|(k, v)| (Value::Str(k.clone()), json_to_value(v)))
                .collect();
            Value::Map(Rc::new(RefCell::new(pairs)))
        }
    }
}

fn value_to_json(val: &Value) -> serde_json::Value {
    match val {
        Value::Int(n) => serde_json::Value::Number((*n).into()),
        Value::Float(f) => serde_json::json!(*f),
        Value::Str(s) => serde_json::Value::String(s.clone()),
        Value::Bool(b) => serde_json::Value::Bool(*b),
        Value::Void => serde_json::Value::Null,
        Value::Array(arr) => {
            let arr = arr.borrow();
            serde_json::Value::Array(arr.iter().map(value_to_json).collect())
        }
        Value::Map(map) => {
            let map = map.borrow();
            let mut obj = serde_json::Map::new();
            for (k, v) in map.iter() {
                obj.insert(k.to_string(), value_to_json(v));
            }
            serde_json::Value::Object(obj)
        }
        _ => serde_json::Value::Null,
    }
}

fn eval_builtin_stdlib(
    name: &str,
    args: &[Expr],
    env: &mut Environment,
    line: usize,
) -> Result<Option<Value>, RuntimeError> {
    match name {
        "kulupu_lukin" => {
            if args.len() != 1 {
                return Err(RuntimeError::new(
                    "json_parse: String argument required",
                    line,
                ));
            }
            let s = match eval_expr(&args[0], env, line)? {
                Value::Str(s) => s,
                _ => return Err(RuntimeError::new("json_parse: String required", line)),
            };
            let json: serde_json::Value = serde_json::from_str(&s)
                .map_err(|e| RuntimeError::new(format!("JSON Parse error: {}", e), line))?;
            Ok(Some(json_to_value(&json)))
        }
        "kulupu_pali" => {
            if args.len() != 1 {
                return Err(RuntimeError::new(
                    "json_stringify: Argument(s) required",
                    line,
                ));
            }
            let val = eval_expr(&args[0], env, line)?;
            let json = value_to_json(&val);
            Ok(Some(Value::Str(json.to_string())))
        }
        "kulupu_pona" => {
            if args.len() != 1 {
                return Err(RuntimeError::new("json_pretty: Argument(s) required", line));
            }
            let val = eval_expr(&args[0], env, line)?;
            let json = value_to_json(&val);
            let pretty = serde_json::to_string_pretty(&json)
                .map_err(|e| RuntimeError::new(format!("JSON conversion error: {}", e), line))?;
            Ok(Some(Value::Str(pretty)))
        }
        "tawa_kama" => {
            #[cfg(feature = "native")]
            {
                if args.len() != 1 {
                    return Err(RuntimeError::new(
                        "HTTP_Contains: URL argument required",
                        line,
                    ));
                }
                let url = match eval_expr(&args[0], env, line)? {
                    Value::Str(s) => s,
                    _ => {
                        return Err(RuntimeError::new(
                            "HTTP_Contains: String URL required",
                            line,
                        ));
                    }
                };
                let body = reqwest::blocking::get(&url)
                    .map_err(|e| RuntimeError::new(format!("HTTP error: {}", e), line))?
                    .text()
                    .map_err(|e| {
                        RuntimeError::new(format!("HTTP response read error: {}", e), line)
                    })?;
                Ok(Some(Value::Str(body)))
            }
            #[cfg(not(feature = "native"))]
            return Err(RuntimeError::new(
                "HTTP_Contains: Not supported in playground",
                line,
            ));
        }
        "tawa_pana" => {
            #[cfg(feature = "native")]
            {
                if args.len() < 2 {
                    return Err(RuntimeError::new(
                        "http_post: URL and body arguments required",
                        line,
                    ));
                }
                let url = match eval_expr(&args[0], env, line)? {
                    Value::Str(s) => s,
                    _ => return Err(RuntimeError::new("http_post: String URL required", line)),
                };
                let body_val = eval_expr(&args[1], env, line)?;
                let body_str = match &body_val {
                    Value::Str(s) => s.clone(),
                    _ => value_to_json(&body_val).to_string(),
                };
                let client = reqwest::blocking::Client::new();
                let resp = client
                    .post(&url)
                    .header("Content-Type", "application/json")
                    .body(body_str)
                    .send()
                    .map_err(|e| RuntimeError::new(format!("HTTP POST error: {}", e), line))?
                    .text()
                    .map_err(|e| {
                        RuntimeError::new(format!("HTTP response read error: {}", e), line)
                    })?;
                Ok(Some(Value::Str(resp)))
            }
            #[cfg(not(feature = "native"))]
            return Err(RuntimeError::new(
                "http_post: Not supported in playground",
                line,
            ));
        }
        "nasin_alasa" => {
            if args.len() != 2 {
                return Err(RuntimeError::new(
                    "regex_find: Pattern and text arguments required",
                    line,
                ));
            }
            let pattern = match eval_expr(&args[0], env, line)? {
                Value::Str(s) => s,
                _ => {
                    return Err(RuntimeError::new(
                        "regex_find: String pattern required",
                        line,
                    ));
                }
            };
            let text = match eval_expr(&args[1], env, line)? {
                Value::Str(s) => s,
                _ => return Err(RuntimeError::new("regex_find: String text required", line)),
            };
            let re = regex::Regex::new(&pattern)
                .map_err(|e| RuntimeError::new(format!("Regex error: {}", e), line))?;
            let matches: Vec<Value> = re
                .find_iter(&text)
                .map(|m| Value::Str(m.as_str().to_string()))
                .collect();
            Ok(Some(Value::Array(Rc::new(RefCell::new(matches)))))
        }
        "nasin_sama" => {
            if args.len() != 2 {
                return Err(RuntimeError::new(
                    "regex_match: Pattern and text arguments required",
                    line,
                ));
            }
            let pattern = match eval_expr(&args[0], env, line)? {
                Value::Str(s) => s,
                _ => {
                    return Err(RuntimeError::new(
                        "regex_match: String pattern required",
                        line,
                    ));
                }
            };
            let text = match eval_expr(&args[1], env, line)? {
                Value::Str(s) => s,
                _ => return Err(RuntimeError::new("regex_match: String text required", line)),
            };
            let re = regex::Regex::new(&pattern)
                .map_err(|e| RuntimeError::new(format!("Regex error: {}", e), line))?;
            Ok(Some(Value::Bool(re.is_match(&text))))
        }
        "nasin_ante" => {
            if args.len() != 3 {
                return Err(RuntimeError::new(
                    "nasin_ante: Pattern, text, replacement arguments required",
                    line,
                ));
            }
            let pattern = match eval_expr(&args[0], env, line)? {
                Value::Str(s) => s,
                _ => {
                    return Err(RuntimeError::new(
                        "nasin_ante: String pattern required",
                        line,
                    ));
                }
            };
            let text = match eval_expr(&args[1], env, line)? {
                Value::Str(s) => s,
                _ => return Err(RuntimeError::new("nasin_ante: String text required", line)),
            };
            let replacement = match eval_expr(&args[2], env, line)? {
                Value::Str(s) => s,
                _ => {
                    return Err(RuntimeError::new(
                        "nasin_ante: String replacement required",
                        line,
                    ));
                }
            };
            let re = regex::Regex::new(&pattern)
                .map_err(|e| RuntimeError::new(format!("Regex error: {}", e), line))?;
            Ok(Some(Value::Str(
                re.replace_all(&text, replacement.as_str()).to_string(),
            )))
        }
        "tenpo_ni" => {
            let now = chrono::Local::now();
            Ok(Some(Value::Str(
                now.format("%Y-%m-%d %H:%M:%S").to_string(),
            )))
        }
        "suno_ni" => {
            let now = chrono::Local::now();
            Ok(Some(Value::Str(now.format("%Y-%m-%d").to_string())))
        }
        "nanpa_tenpo" => {
            let now = chrono::Utc::now();
            Ok(Some(Value::Int(now.timestamp())))
        }
        "toki_ijo" => {
            let args: Vec<Value> = std::env::args().skip(2).map(Value::Str).collect();
            Ok(Some(Value::Array(Rc::new(RefCell::new(args)))))
        }
        "ma_ijo" => {
            if args.len() != 1 {
                return Err(RuntimeError::new(
                    "ma_ijo: Variable name argument required",
                    line,
                ));
            }
            let var_name = match eval_expr(&args[0], env, line)? {
                Value::Str(s) => s,
                _ => return Err(RuntimeError::new("ma_ijo: String required", line)),
            };
            match std::env::var(&var_name) {
                Ok(val) => Ok(Some(Value::Str(val))),
                Err(_) => Ok(Some(Value::Void)),
            }
        }
        "lawa_pali" => {
            #[cfg(feature = "native")]
            {
                if args.len() != 1 {
                    return Err(RuntimeError::new(
                        "lawa_pali: command String required",
                        line,
                    ));
                }
                let cmd = match eval_expr(&args[0], env, line)? {
                    Value::Str(s) => s,
                    _ => return Err(RuntimeError::new("lawa_pali: String required", line)),
                };
                let output = std::process::Command::new("sh")
                    .arg("-c")
                    .arg(&cmd)
                    .output()
                    .map_err(|e| RuntimeError::new(format!("Execution error: {}", e), line))?;
                Ok(Some(Value::Str(
                    String::from_utf8_lossy(&output.stdout).to_string(),
                )))
            }
            #[cfg(not(feature = "native"))]
            return Err(RuntimeError::new(
                "lawa_pali: Not supported in playground",
                line,
            ));
        }
        "lape" => {
            if args.len() != 1 {
                return Err(RuntimeError::new(
                    "lape: Milliseconds argument required",
                    line,
                ));
            }
            let ms = match eval_expr(&args[0], env, line)? {
                Value::Int(n) => n as u64,
                _ => return Err(RuntimeError::new("lape: Integer required", line)),
            };
            std::thread::sleep(std::time::Duration::from_millis(ms));
            Ok(Some(Value::Void))
        }
        "nasin" => {
            if args.len() != 1 {
                return Err(RuntimeError::new("nasin: Argument(s) required", line));
            }
            let val = eval_expr(&args[0], env, line)?;
            let type_name = match val {
                Value::Int(_) => "nanpa_kind",
                Value::Float(_) => "kipisi",
                Value::Str(_) => "sitelen",
                Value::Bool(_) => "lawa",
                Value::Void => "weka",
                Value::Function { .. } => "pali",
                Value::Closure { .. } => "pali_lili",
                Value::Array(_) => "kulupu_kipisi",
                Value::Tuple(_) => "wan",
                Value::Map(_) => "lipu",
                Value::Struct { .. } => "kulupu",
            };

            Ok(Some(Value::Str(type_name.to_string())))
        }
        _ => Ok(None),
    }
}

fn eval_builtin_io(
    name: &str,
    args: &[Expr],
    env: &mut Environment,
    line: usize,
) -> Result<Option<Value>, RuntimeError> {
    match name {
        "lipu_lukin" => {
            if args.len() != 1 {
                return Err(RuntimeError::new(
                    "lipu_lukin: File path argument required",
                    line,
                ));
            }
            let path = match eval_expr(&args[0], env, line)? {
                Value::Str(s) => s,
                _ => return Err(RuntimeError::new("lipu_lukin: String path required", line)),
            };
            let content = std::fs::read_to_string(&path).map_err(|e| {
                RuntimeError::new(format!("lipu_lukin Failed '{}': {}", path, e), line)
            })?;
            Ok(Some(Value::Str(content)))
        }
        "lipu_sitelen" => {
            if args.len() != 2 {
                return Err(RuntimeError::new(
                    "lipu_sitelen: Path and content arguments required",
                    line,
                ));
            }
            let path = match eval_expr(&args[0], env, line)? {
                Value::Str(s) => s,
                _ => {
                    return Err(RuntimeError::new(
                        "lipu_sitelen: String path required",
                        line,
                    ));
                }
            };
            let content = eval_expr(&args[1], env, line)?.to_string();
            std::fs::write(&path, &content).map_err(|e| {
                RuntimeError::new(format!("lipu_sitelen Failed '{}': {}", path, e), line)
            })?;
            Ok(Some(Value::Void))
        }
        "lipu_sin" => {
            if args.len() != 2 {
                return Err(RuntimeError::new(
                    "lipu_sin: Path and content arguments required",
                    line,
                ));
            }
            let path = match eval_expr(&args[0], env, line)? {
                Value::Str(s) => s,
                _ => return Err(RuntimeError::new("lipu_sin: String path required", line)),
            };
            let content = eval_expr(&args[1], env, line)?.to_string();
            use std::io::Write as IoWrite;
            let mut file = std::fs::OpenOptions::new()
                .append(true)
                .create(true)
                .open(&path)
                .map_err(|e| {
                    RuntimeError::new(format!("lipu_sin Failed '{}': {}", path, e), line)
                })?;
            file.write_all(content.as_bytes())
                .map_err(|e| RuntimeError::new(format!("lipu_sin writing Failed: {}", e), line))?;
            Ok(Some(Value::Void))
        }
        "lipu_lon" => {
            if args.len() != 1 {
                return Err(RuntimeError::new(
                    "lipu_lon: File path argument required",
                    line,
                ));
            }
            let path = match eval_expr(&args[0], env, line)? {
                Value::Str(s) => s,
                _ => return Err(RuntimeError::new("lipu_lon: String path required", line)),
            };
            Ok(Some(Value::Bool(std::path::Path::new(&path).exists())))
        }
        "toki_pakala" => {
            let mut parts = Vec::new();
            for arg in args {
                parts.push(eval_expr(arg, env, line)?.to_string());
            }
            eprintln!("{}", parts.join(" "));
            Ok(Some(Value::Void))
        }
        "sitelen_pali" => {
            if args.is_empty() {
                return Err(RuntimeError::new(
                    "sitelen_pali: Format string argument required",
                    line,
                ));
            }
            let template = match eval_expr(&args[0], env, line)? {
                Value::Str(s) => s,
                _ => {
                    return Err(RuntimeError::new(
                        "sitelen_pali: First arg String required",
                        line,
                    ));
                }
            };
            let mut positional = Vec::new();
            for arg in &args[1..] {
                positional.push(eval_expr(arg, env, line)?.to_string());
            }
            let result = if positional.is_empty() {
                let snapshot = env.snapshot();
                let mut out = template.clone();
                for (k, v) in &snapshot {
                    out = out.replace(&format!("{{{}}}", k), &v.to_string());
                }
                out
            } else {
                let mut out = template.clone();
                for (i, val) in positional.iter().enumerate() {
                    out = out.replace(&format!("{{{}}}", i), val);
                }
                out
            };
            Ok(Some(Value::Str(result)))
        }
        _ => Ok(None),
    }
}

fn eval_builtin_math(
    name: &str,
    args: &[Expr],
    env: &mut Environment,
    line: usize,
) -> Result<Option<Value>, RuntimeError> {
    match name {
        "lili_nanpa" => {
            if args.len() != 1 {
                return Err(RuntimeError::new("lili_nanpa: Argument(s) required", line));
            }
            let v = eval_expr(&args[0], env, line)?;
            match v {
                Value::Int(n) => Ok(Some(Value::Float((n as f64).sqrt()))),
                Value::Float(f) => Ok(Some(Value::Float(f.sqrt()))),
                _ => Err(RuntimeError::new("lili_nanpa: Numeric type required", line)),
            }
        }
        "wawa_nanpa" => {
            if args.len() != 1 {
                return Err(RuntimeError::new("wawa_nanpa: Argument(s) required", line));
            }
            let v = eval_expr(&args[0], env, line)?;
            match v {
                Value::Int(n) => Ok(Some(Value::Int(n.abs()))),
                Value::Float(f) => Ok(Some(Value::Float(f.abs()))),
                _ => Err(RuntimeError::new("wawa_nanpa: Numeric type required", line)),
            }
        }
        "suli_nanpa" => {
            if args.len() != 2 {
                return Err(RuntimeError::new(
                    "suli_nanpa: Argument(s) required (base, exponent)",
                    line,
                ));
            }
            let base = eval_expr(&args[0], env, line)?;
            let exp = eval_expr(&args[1], env, line)?;
            match (base, exp) {
                (Value::Int(b), Value::Int(e)) => Ok(Some(Value::Float((b as f64).powf(e as f64)))),
                (Value::Float(b), Value::Float(e)) => Ok(Some(Value::Float(b.powf(e)))),
                (Value::Float(b), Value::Int(e)) => Ok(Some(Value::Float(b.powf(e as f64)))),
                (Value::Int(b), Value::Float(e)) => Ok(Some(Value::Float((b as f64).powf(e)))),
                _ => Err(RuntimeError::new("suli_nanpa: Numeric type required", line)),
            }
        }
        "nanpa_ante" => {
            if args.len() != 1 {
                return Err(RuntimeError::new("nanpa_ante: Argument(s) required", line));
            }
            let v = eval_expr(&args[0], env, line)?;
            match v {
                Value::Int(n) => Ok(Some(Value::Int(n))),
                Value::Float(f) => Ok(Some(Value::Int(f as i64))),
                Value::Str(s) => s
                    .parse::<i64>()
                    .map(|n| Some(Value::Int(n)))
                    .map_err(|_| RuntimeError::new(format!("nanpa_ante Failed: '{}'", s), line)),
                Value::Bool(b) => Ok(Some(Value::Int(if b { 1 } else { 0 }))),
                _ => Err(RuntimeError::new("nanpa_ante: Cannot convert type", line)),
            }
        }
        "kipisi_ante" => {
            if args.len() != 1 {
                return Err(RuntimeError::new("kipisi_ante: Argument(s) required", line));
            }
            let v = eval_expr(&args[0], env, line)?;
            match v {
                Value::Int(n) => Ok(Some(Value::Float(n as f64))),
                Value::Float(f) => Ok(Some(Value::Float(f))),
                Value::Str(s) => s
                    .parse::<f64>()
                    .map(|f| Some(Value::Float(f)))
                    .map_err(|_| RuntimeError::new(format!("kipisi_ante Failed: '{}'", s), line)),
                _ => Err(RuntimeError::new("kipisi_ante: Cannot convert type", line)),
            }
        }
        "suli_ijo" => {
            if args.len() != 1 {
                return Err(RuntimeError::new("suli_ijo: Argument(s) required", line));
            }
            let v = eval_expr(&args[0], env, line)?;
            match v {
                Value::Str(s) => Ok(Some(Value::Int(s.chars().count() as i64))),
                _ => Err(RuntimeError::new("suli_ijo: String type required", line)),
            }
        }
        _ => Ok(None),
    }
}

fn eval_binary_op(
    op: &BinaryOpKind,
    lv: Value,
    rv: Value,
    line: usize,
) -> Result<Value, RuntimeError> {
    match op {
        BinaryOpKind::Add => match (lv, rv) {
            (Value::Int(a), Value::Int(b)) => Ok(Value::Int(a + b)),
            (Value::Float(a), Value::Float(b)) => Ok(Value::Float(a + b)),
            (Value::Int(a), Value::Float(b)) => Ok(Value::Float(a as f64 + b)),
            (Value::Float(a), Value::Int(b)) => Ok(Value::Float(a + b as f64)),
            (Value::Str(a), Value::Str(b)) => Ok(Value::Str(a + &b)),
            _ => Err(RuntimeError::new("+ op: Type mismatch", line)),
        },
        BinaryOpKind::Sub => match (lv, rv) {
            (Value::Int(a), Value::Int(b)) => Ok(Value::Int(a - b)),
            (Value::Float(a), Value::Float(b)) => Ok(Value::Float(a - b)),
            (Value::Int(a), Value::Float(b)) => Ok(Value::Float(a as f64 - b)),
            (Value::Float(a), Value::Int(b)) => Ok(Value::Float(a - b as f64)),
            _ => Err(RuntimeError::new("- op: Type mismatch", line)),
        },
        BinaryOpKind::Mul => match (lv, rv) {
            (Value::Int(a), Value::Int(b)) => Ok(Value::Int(a * b)),
            (Value::Float(a), Value::Float(b)) => Ok(Value::Float(a * b)),
            (Value::Int(a), Value::Float(b)) => Ok(Value::Float(a as f64 * b)),
            (Value::Float(a), Value::Int(b)) => Ok(Value::Float(a * b as f64)),
            _ => Err(RuntimeError::new("* op: Type mismatch", line)),
        },
        BinaryOpKind::Div => match (lv, rv) {
            (Value::Int(a), Value::Int(b)) => {
                if b == 0 {
                    Err(RuntimeError::new("Division by zero", line))
                } else {
                    Ok(Value::Int(a / b))
                }
            }
            (Value::Float(a), Value::Float(b)) => Ok(Value::Float(a / b)),
            (Value::Int(a), Value::Float(b)) => Ok(Value::Float(a as f64 / b)),
            (Value::Float(a), Value::Int(b)) => Ok(Value::Float(a / b as f64)),
            _ => Err(RuntimeError::new("/ op: Type mismatch", line)),
        },
        BinaryOpKind::Mod => match (lv, rv) {
            (Value::Int(a), Value::Int(b)) => {
                if b == 0 {
                    Err(RuntimeError::new("Modulo by zero", line))
                } else {
                    Ok(Value::Int(a % b))
                }
            }
            _ => Err(RuntimeError::new("% op: only for integers", line)),
        },
        BinaryOpKind::Eq => Ok(Value::Bool(values_equal(&lv, &rv))),
        BinaryOpKind::NotEq => Ok(Value::Bool(!values_equal(&lv, &rv))),
        BinaryOpKind::Lt => compare_values(lv, rv, |a, b| a < b, line),
        BinaryOpKind::Gt => compare_values(lv, rv, |a, b| a > b, line),
        BinaryOpKind::LtEq => compare_values(lv, rv, |a, b| a <= b, line),
        BinaryOpKind::GtEq => compare_values(lv, rv, |a, b| a >= b, line),
        BinaryOpKind::And => match (lv, rv) {
            (Value::Bool(a), Value::Bool(b)) => Ok(Value::Bool(a && b)),
            _ => Err(RuntimeError::new("&& op: Boolean values only", line)),
        },
        BinaryOpKind::Or => match (lv, rv) {
            (Value::Bool(a), Value::Bool(b)) => Ok(Value::Bool(a || b)),
            _ => Err(RuntimeError::new("|| op: Boolean values only", line)),
        },
    }
}

fn values_equal(a: &Value, b: &Value) -> bool {
    match (a, b) {
        (Value::Int(x), Value::Int(y)) => x == y,
        (Value::Float(x), Value::Float(y)) => x == y,
        (Value::Str(x), Value::Str(y)) => x == y,
        (Value::Bool(x), Value::Bool(y)) => x == y,
        (Value::Void, Value::Void) => true,
        (Value::Array(x), Value::Array(y)) => {
            let x = x.borrow();
            let y = y.borrow();
            x.len() == y.len() && x.iter().zip(y.iter()).all(|(a, b)| values_equal(a, b))
        }
        _ => false,
    }
}

fn compare_values<F>(lv: Value, rv: Value, cmp: F, line: usize) -> Result<Value, RuntimeError>
where
    F: Fn(f64, f64) -> bool,
{
    match (lv, rv) {
        (Value::Int(a), Value::Int(b)) => Ok(Value::Bool(cmp(a as f64, b as f64))),
        (Value::Float(a), Value::Float(b)) => Ok(Value::Bool(cmp(a, b))),
        _ => Err(RuntimeError::new("Comparison: numeric types only", line)),
    }
}

pub fn eval_stmt(stmt: &Stmt, env: &mut Environment) -> Result<Option<Signal>, RuntimeError> {
    let line = stmt.span.line;
    match &stmt.kind {
        StmtKind::VarDecl { name, value, .. } => {
            let val = eval_expr(value, env, line)?;
            env.set(name.clone(), val);
            Ok(None)
        }

        StmtKind::FuncDef {
            name, params, body, ..
        } => {
            let func = Value::Function {
                params: params.clone(),
                body: body.clone(),
            };
            env.set(name.clone(), func);
            Ok(None)
        }

        StmtKind::Return(expr_opt) => {
            let val = match expr_opt {
                Some(expr) => eval_expr(expr, env, line)?,
                None => Value::Void,
            };
            Ok(Some(Signal::Return(val)))
        }

        StmtKind::If {
            cond,
            then_block,
            else_block,
        } => {
            let cond_val = eval_expr(cond, env, line)?;
            match cond_val {
                Value::Bool(true) => eval_block(then_block, env),
                Value::Bool(false) => {
                    if let Some(else_stmts) = else_block {
                        eval_block(else_stmts, env)
                    } else {
                        Ok(None)
                    }
                }
                _ => Err(RuntimeError::new("Condition requires boolean", line)),
            }
        }

        StmtKind::WhileLoop { cond, body } => {
            loop {
                let cond_val = eval_expr(cond, env, line)?;
                match cond_val {
                    Value::Bool(true) => {}
                    Value::Bool(false) => break,
                    _ => return Err(RuntimeError::new("While condition requires boolean", line)),
                }
                match eval_block(body, env)? {
                    Some(Signal::Break) => break,
                    Some(Signal::Continue) => continue,
                    Some(sig @ Signal::Return(_)) => return Ok(Some(sig)),
                    None => {}
                }
            }
            Ok(None)
        }

        StmtKind::ForLoop {
            init,
            cond,
            step,
            body,
        } => {
            eval_stmt(init, env)?;
            loop {
                let cond_val = eval_expr(cond, env, line)?;
                match cond_val {
                    Value::Bool(true) => {}
                    Value::Bool(false) => break,
                    _ => return Err(RuntimeError::new("Loop condition requires boolean", line)),
                }
                match eval_block(body, env)? {
                    Some(Signal::Break) => break,
                    Some(Signal::Continue) => {
                        eval_stmt(step, env)?;
                        continue;
                    }
                    Some(sig @ Signal::Return(_)) => return Ok(Some(sig)),
                    None => {}
                }
                eval_stmt(step, env)?;
            }
            Ok(None)
        }

        StmtKind::Break => Ok(Some(Signal::Break)),
        StmtKind::Continue => Ok(Some(Signal::Continue)),

        StmtKind::ExprStmt(expr) => {
            eval_expr(expr, env, line)?;
            Ok(None)
        }

        StmtKind::StructDef { name, .. } => {
            env.set(name.clone(), Value::Str(format!("<struct {}>", name)));
            Ok(None)
        }

        StmtKind::TryCatch {
            try_block,
            error_name,
            catch_block,
        } => match eval_block(try_block, env) {
            Ok(sig) => Ok(sig),
            Err(e) => {
                env.set(error_name.clone(), Value::Str(e.message.clone()));
                eval_block(catch_block, env)
            }
        },

        StmtKind::Import(path) => {
            let source = std::fs::read_to_string(path)
                .map_err(|e| RuntimeError::new(format!("Import failed '{}': {}", path, e), line))?;
            let tokens = crate::lexer::tokenize(&source);
            let program = crate::parser::parse(tokens).map_err(|e| {
                RuntimeError::new(format!("'{}' Parse error: {}", path, e.message), line)
            })?;
            eval_block(&program.stmts, env)?;
            Ok(None)
        }

        StmtKind::Match { expr, arms } => {
            let val = eval_expr(expr, env, line)?;
            for arm in arms {
                if pattern_matches(&arm.pattern, &val, env) {
                    return eval_block(&arm.body, env);
                }
            }
            Ok(None)
        }

        StmtKind::ImplBlock {
            struct_name,
            methods,
        } => {
            for method_stmt in methods {
                if let StmtKind::FuncDef {
                    name,
                    params,
                    return_type: _,
                    body,
                } = &method_stmt.kind
                {
                    let key = format!("{}::{}", struct_name, name);
                    let func = Value::Function {
                        params: params.clone(),
                        body: body.clone(),
                    };
                    env.set(key, func);
                }
            }
            Ok(None)
        }

        StmtKind::EnumDef { name, variants } => {
            for (i, variant) in variants.iter().enumerate() {
                let key = format!("{}::{}", name, variant);
                env.set(key, Value::Int(i as i64));
            }
            Ok(None)
        }

        StmtKind::ForIn {
            var_name,
            iterable,
            body,
        } => {
            let iter_val = eval_expr(iterable, env, line)?;
            match iter_val {
                Value::Array(arr) => {
                    let items = arr.borrow().clone();
                    for item in items {
                        env.set(var_name.clone(), item);
                        match eval_block(body, env)? {
                            Some(Signal::Break) => break,
                            Some(Signal::Continue) => continue,
                            Some(sig @ Signal::Return(_)) => return Ok(Some(sig)),
                            None => {}
                        }
                    }
                    Ok(None)
                }
                Value::Str(s) => {
                    for ch in s.chars() {
                        env.set(var_name.clone(), Value::Str(ch.to_string()));
                        match eval_block(body, env)? {
                            Some(Signal::Break) => break,
                            Some(Signal::Continue) => continue,
                            Some(sig @ Signal::Return(_)) => return Ok(Some(sig)),
                            None => {}
                        }
                    }
                    Ok(None)
                }
                _ => Err(RuntimeError::new("for-in requires array or string", line)),
            }
        }
    }
}

fn pattern_matches(pattern: &Pattern, value: &Value, env: &mut Environment) -> bool {
    match (pattern, value) {
        (Pattern::Wildcard, _) => true,
        (Pattern::IntLiteral(n), Value::Int(v)) => n == v,
        (Pattern::FloatLiteral(f), Value::Float(v)) => (f - v).abs() < f64::EPSILON,
        (Pattern::StringLiteral(s), Value::Str(v)) => s == v,
        (Pattern::BoolLiteral(b), Value::Bool(v)) => b == v,
        (Pattern::Identifier(name), val) => {
            env.set(name.clone(), val.clone());
            true
        }
        (Pattern::ArrayPattern(pats), Value::Array(arr)) => {
            let arr = arr.borrow();
            if pats.len() != arr.len() {
                return false;
            }
            pats.iter()
                .zip(arr.iter())
                .all(|(p, v)| pattern_matches(p, v, env))
        }
        _ => false,
    }
}

pub fn eval_block(stmts: &[Stmt], env: &mut Environment) -> Result<Option<Signal>, RuntimeError> {
    for stmt in stmts {
        if let Some(sig) = eval_stmt(stmt, env)? {
            return Ok(Some(sig));
        }
    }
    Ok(None)
}

pub fn interpret(program: Program) -> Result<(), RuntimeError> {
    let mut interp = Interpreter::new();
    interp.interpret(&program)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_env_set_get() {
        let mut env = Environment::new();
        env.set("age".to_string(), Value::Int(20));
        assert!(matches!(env.get("age"), Some(Value::Int(20))));
    }

    #[test]
    fn test_env_scope_chain() {
        let mut outer = Environment::new();
        outer.set("x".to_string(), Value::Int(10));
        let inner = Environment::new_enclosed(outer);
        assert!(matches!(inner.get("x"), Some(Value::Int(10))));
        assert!(inner.get("y").is_none());
    }

    #[test]
    fn test_value_display() {
        assert_eq!(Value::Int(42).to_string(), "42");
        assert_eq!(Value::Bool(true).to_string(), "kin");
        assert_eq!(Value::Bool(false).to_string(), "ala");
        assert_eq!(Value::Void.to_string(), "weka");
    }

    #[test]
    fn test_env_update() {
        let mut env = Environment::new();
        env.set("x".to_string(), Value::Int(1));
        env.update("x", Value::Int(2));
        assert!(matches!(env.get("x"), Some(Value::Int(2))));
    }

    #[test]
    fn test_eval_arithmetic() {
        let mut env = Environment::new();
        let expr = Expr::BinaryOp {
            op: BinaryOpKind::Add,
            left: Box::new(Expr::IntLiteral(3)),
            right: Box::new(Expr::BinaryOp {
                op: BinaryOpKind::Mul,
                left: Box::new(Expr::IntLiteral(5)),
                right: Box::new(Expr::IntLiteral(2)),
            }),
        };
        let result = eval_expr(&expr, &mut env, 0).unwrap();
        assert!(matches!(result, Value::Int(13)));
    }

    #[test]
    fn test_eval_var_decl() {
        let mut env = Environment::new();
        let stmt = Stmt::unspanned(StmtKind::VarDecl {
            name: "age".to_string(),
            ty: None,
            value: Expr::IntLiteral(20),
            mutable: true,
        });
        eval_stmt(&stmt, &mut env).unwrap();
        assert!(matches!(env.get("age"), Some(Value::Int(20))));
    }

    #[test]
    fn test_eval_if_stmt() {
        let mut env = Environment::new();
        let stmt = Stmt::unspanned(StmtKind::If {
            cond: Expr::BoolLiteral(true),
            then_block: vec![Stmt::unspanned(StmtKind::VarDecl {
                name: "x".to_string(),
                ty: None,
                value: Expr::IntLiteral(1),
                mutable: false,
            })],
            else_block: None,
        });
        eval_stmt(&stmt, &mut env).unwrap();
        assert!(matches!(env.get("x"), Some(Value::Int(1))));
    }

    #[test]
    fn test_eval_fibonacci() {
        let fib_body = vec![
            Stmt::unspanned(StmtKind::If {
                cond: Expr::BinaryOp {
                    op: BinaryOpKind::LtEq,
                    left: Box::new(Expr::Identifier("n".to_string())),
                    right: Box::new(Expr::IntLiteral(1)),
                },
                then_block: vec![Stmt::unspanned(StmtKind::Return(Some(Expr::Identifier(
                    "n".to_string(),
                ))))],
                else_block: None,
            }),
            Stmt::unspanned(StmtKind::Return(Some(Expr::BinaryOp {
                op: BinaryOpKind::Add,
                left: Box::new(Expr::Call {
                    name: "fibonacci".to_string(),
                    args: vec![Expr::BinaryOp {
                        op: BinaryOpKind::Sub,
                        left: Box::new(Expr::Identifier("n".to_string())),
                        right: Box::new(Expr::IntLiteral(1)),
                    }],
                }),
                right: Box::new(Expr::Call {
                    name: "fibonacci".to_string(),
                    args: vec![Expr::BinaryOp {
                        op: BinaryOpKind::Sub,
                        left: Box::new(Expr::Identifier("n".to_string())),
                        right: Box::new(Expr::IntLiteral(2)),
                    }],
                }),
            }))),
        ];

        let program = Program::new(vec![
            Stmt::unspanned(StmtKind::FuncDef {
                name: "fibonacci".to_string(),
                params: vec![("n".to_string(), Type::Nanpa)],
                return_type: Some(Type::Nanpa),
                body: fib_body,
            }),
            Stmt::unspanned(StmtKind::VarDecl {
                name: "result".to_string(),
                ty: None,
                value: Expr::Call {
                    name: "fibonacci".to_string(),
                    args: vec![Expr::IntLiteral(10)],
                },
                mutable: false,
            }),
        ]);

        let mut env = Environment::new();
        eval_block(&program.stmts, &mut env).unwrap();
        assert!(matches!(env.get("result"), Some(Value::Int(55))));
    }

    #[test]
    fn test_toki() {
        let program = Program::new(vec![Stmt::unspanned(StmtKind::ExprStmt(Expr::Call {
            name: "toki".to_string(),
            args: vec![Expr::StringLiteral("toki".to_string())],
        }))]);
        let result = interpret(program);
        assert!(result.is_ok());
    }
}
