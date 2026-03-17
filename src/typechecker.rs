use crate::ast::*;
use std::collections::HashMap;

#[derive(Debug)]
pub struct TypeError {
    pub message: String,
    pub line: usize,
}

impl TypeError {
    fn new(msg: impl Into<String>, line: usize) -> Self {
        Self {
            message: msg.into(),
            line,
        }
    }
}

struct TypeEnv {
    vars: HashMap<String, Type>,
    funcs: HashMap<String, (Vec<Type>, Option<Type>)>,
    structs: HashMap<String, Vec<(String, Type)>>,
}

impl TypeEnv {
    fn new() -> Self {
        Self {
            vars: HashMap::new(),
            funcs: HashMap::new(),
            structs: HashMap::new(),
        }
    }
}

fn array_element_type(items: &[Expr], env: &TypeEnv) -> Option<Type> {
    if items.is_empty() {
        return Some(Type::Nanpa);
    }

    let mut inferred_type = infer_expr_type(&items[0], env)?;

    for item in &items[1..] {
        let actual_type = infer_expr_type(item, env)?;
        if types_compatible(&inferred_type, &actual_type) {
            continue;
        }
        if types_compatible(&actual_type, &inferred_type) {
            inferred_type = actual_type;
        }
    }

    Some(inferred_type)
}

fn array_element_mismatch(items: &[Expr], env: &TypeEnv) -> Option<(Type, Type)> {
    if items.len() < 2 {
        return None;
    }

    let mut inferred_type = infer_expr_type(&items[0], env)?;

    for item in &items[1..] {
        let actual_type = infer_expr_type(item, env)?;
        if types_compatible(&inferred_type, &actual_type) {
            continue;
        }
        if types_compatible(&actual_type, &inferred_type) {
            inferred_type = actual_type;
            continue;
        }
        return Some((inferred_type, actual_type));
    }

    None
}

fn infer_expr_type(expr: &Expr, env: &TypeEnv) -> Option<Type> {
    match expr {
        Expr::IntLiteral(_) => Some(Type::Nanpa),
        Expr::FloatLiteral(_) => Some(Type::Kipisi),
        Expr::StringLiteral(_) => Some(Type::Sitelen),
        Expr::BoolLiteral(_) => Some(Type::Lawa),
        Expr::NullLiteral => Some(Type::Weka),
        Expr::Identifier(name) => env.vars.get(name).cloned(),
        Expr::BinaryOp { op, left, right } => {
            let lt = infer_expr_type(left, env)?;
            let rt = infer_expr_type(right, env)?;
            match op {
                BinaryOpKind::Eq
                | BinaryOpKind::NotEq
                | BinaryOpKind::Lt
                | BinaryOpKind::Gt
                | BinaryOpKind::LtEq
                | BinaryOpKind::GtEq
                | BinaryOpKind::And
                | BinaryOpKind::Or => Some(Type::Lawa),
                BinaryOpKind::Add => match (&lt, &rt) {
                    (Type::Sitelen, Type::Sitelen) => Some(Type::Sitelen),
                    (Type::Kipisi, _) | (_, Type::Kipisi) => Some(Type::Kipisi),
                    _ => Some(lt),
                },
                _ => {
                    if matches!(lt, Type::Kipisi) || matches!(rt, Type::Kipisi) {
                        Some(Type::Kipisi)
                    } else {
                        Some(lt)
                    }
                }
            }
        }
        Expr::Call { name, .. } => env.funcs.get(name).and_then(|(_, ret)| ret.clone()),
        Expr::ArrayLiteral(items) => Some(Type::Kulupu(Box::new(array_element_type(items, env)?))),
        Expr::TupleLiteral(_) => Some(Type::Wan(Vec::new())),
        Expr::Range { .. } => Some(Type::Kulupu(Box::new(Type::Nanpa))),
        Expr::StructLiteral { name, .. } => Some(Type::KulupuIjo(name.clone())),
        _ => None,
    }
}

pub fn check(program: &Program) -> Vec<TypeError> {
    let mut errors = Vec::new();
    let mut env = TypeEnv::new();

    for stmt in &program.stmts {
        check_stmt(stmt, &mut env, &mut errors);
    }

    errors
}

fn check_call_args(
    name: &str,
    args: &[Expr],
    env: &TypeEnv,
    line: usize,
    errors: &mut Vec<TypeError>,
) {
    let Some((param_types, _)) = env.funcs.get(name) else {
        return;
    };

    if args.len() != param_types.len() {
        errors.push(TypeError::new(
            format!(
                "Argument count mismatch: '{}' expects {} args but got {}",
                name,
                param_types.len(),
                args.len()
            ),
            line,
        ));
    }

    for (index, (arg, param_type)) in args.iter().zip(param_types.iter()).enumerate() {
        if let Some(actual_type) = infer_expr_type(arg, env)
            && !types_compatible(param_type, &actual_type)
        {
            errors.push(TypeError::new(
                format!(
                    "Argument type mismatch: '{}' arg {} expects {:?}, got {:?}",
                    name,
                    index + 1,
                    param_type,
                    actual_type
                ),
                line,
            ));
        }
    }
}

fn check_expr(expr: &Expr, env: &TypeEnv, errors: &mut Vec<TypeError>, line: usize) {
    match expr {
        Expr::BinaryOp { left, right, .. } => {
            check_expr(left, env, errors, line);
            check_expr(right, env, errors, line);
        }
        Expr::UnaryOp { expr, .. } => check_expr(expr, env, errors, line),
        Expr::Call { name, args } => {
            for arg in args {
                check_expr(arg, env, errors, line);
            }
            check_call_args(name, args, env, line, errors);
        }
        Expr::Assign { value, .. } => check_expr(value, env, errors, line),
        Expr::ArrayLiteral(items) => {
            for item in items {
                check_expr(item, env, errors, line);
            }
            if let Some((expected_type, actual_type)) = array_element_mismatch(items, env) {
                errors.push(TypeError::new(
                    format!(
                        "Array element type mismatch: expected {:?}, found {:?}",
                        expected_type, actual_type
                    ),
                    line,
                ));
            }
        }
        Expr::TupleLiteral(items) => {
            for item in items {
                check_expr(item, env, errors, line);
            }
        }
        Expr::Index { object, index } => {
            check_expr(object, env, errors, line);
            check_expr(index, env, errors, line);
        }
        Expr::IndexAssign {
            object,
            index,
            value,
        } => {
            check_expr(object, env, errors, line);
            check_expr(index, env, errors, line);
            check_expr(value, env, errors, line);
        }
        Expr::MethodCall { object, args, .. } => {
            check_expr(object, env, errors, line);
            for arg in args {
                check_expr(arg, env, errors, line);
            }
        }
        Expr::FieldAccess { object, .. } => check_expr(object, env, errors, line),
        Expr::StructLiteral { name, fields } => {
            for (field_name, value) in fields {
                check_expr(value, env, errors, line);
                let Some(declared_fields) = env.structs.get(name) else {
                    continue;
                };
                let Some((_, declared_type)) = declared_fields
                    .iter()
                    .find(|(declared_name, _)| declared_name == field_name)
                else {
                    continue;
                };
                if let Some(actual_type) = infer_expr_type(value, env)
                    && !types_compatible(declared_type, &actual_type)
                {
                    errors.push(TypeError::new(
                        format!(
                            "Struct field type mismatch: '{}.{}' expects {:?}, got {:?}",
                            name, field_name, declared_type, actual_type
                        ),
                        line,
                    ));
                }
            }
        }
        Expr::FieldAssign { object, value, .. } => {
            check_expr(object, env, errors, line);
            check_expr(value, env, errors, line);
        }
        Expr::Range { start, end } => {
            check_expr(start, env, errors, line);
            check_expr(end, env, errors, line);
        }
        _ => {}
    }
}

fn check_stmt(stmt: &Stmt, env: &mut TypeEnv, errors: &mut Vec<TypeError>) {
    let line = stmt.span.line;

    match &stmt.kind {
        StmtKind::VarDecl {
            name, ty, value, ..
        } => {
            check_expr(value, env, errors, line);

            if let Some(declared_ty) = ty {
                if let Some(actual_ty) = infer_expr_type(value, env)
                    && !types_compatible(declared_ty, &actual_ty)
                {
                    errors.push(TypeError::new(
                        format!(
                            "Type mismatch: '{}' declared as {:?}, assigned {:?}",
                            name, declared_ty, actual_ty
                        ),
                        line,
                    ));
                }
                env.vars.insert(name.clone(), declared_ty.clone());
            } else if let Some(inferred) = infer_expr_type(value, env) {
                env.vars.insert(name.clone(), inferred);
            }
        }

        StmtKind::FuncDef {
            name,
            params,
            return_type,
            body,
        } => {
            let param_types: Vec<Type> = params.iter().map(|(_, t)| t.clone()).collect();
            env.funcs
                .insert(name.clone(), (param_types, return_type.clone()));

            for (pname, pty) in params {
                env.vars.insert(pname.clone(), pty.clone());
            }

            for s in body {
                check_stmt(s, env, errors);
            }

            if let Some(ret_ty) = return_type {
                check_return_types(body, ret_ty, env, errors);
            }
        }

        StmtKind::Return(Some(expr)) => check_expr(expr, env, errors, line),

        StmtKind::StructDef { name, fields } => {
            env.structs.insert(name.clone(), fields.clone());
        }

        StmtKind::If {
            cond,
            then_block,
            else_block,
        } => {
            check_expr(cond, env, errors, line);
            for s in then_block {
                check_stmt(s, env, errors);
            }
            if let Some(else_stmts) = else_block {
                for s in else_stmts {
                    check_stmt(s, env, errors);
                }
            }
        }

        StmtKind::ForLoop {
            init,
            cond,
            step,
            body,
        } => {
            check_stmt(init, env, errors);
            check_expr(cond, env, errors, line);
            check_stmt(step, env, errors);
            for s in body {
                check_stmt(s, env, errors);
            }
        }

        StmtKind::ForIn {
            var_name,
            iterable,
            body,
        } => {
            check_expr(iterable, env, errors, line);
            let item_type = match infer_expr_type(iterable, env) {
                Some(Type::Kulupu(inner)) => *inner,
                Some(Type::Sitelen) => Type::Sitelen,
                _ => Type::Nanpa,
            };
            env.vars.insert(var_name.clone(), item_type);
            for s in body {
                check_stmt(s, env, errors);
            }
        }

        StmtKind::WhileLoop { cond, body } => {
            check_expr(cond, env, errors, line);
            for s in body {
                check_stmt(s, env, errors);
            }
        }

        StmtKind::ExprStmt(expr) => check_expr(expr, env, errors, line),

        StmtKind::TryCatch {
            try_block,
            error_name,
            catch_block,
        } => {
            for s in try_block {
                check_stmt(s, env, errors);
            }
            env.vars.insert(error_name.clone(), Type::Sitelen);
            for s in catch_block {
                check_stmt(s, env, errors);
            }
        }

        StmtKind::Match { expr, arms } => {
            check_expr(expr, env, errors, line);
            for arm in arms {
                for s in &arm.body {
                    check_stmt(s, env, errors);
                }
            }
        }

        StmtKind::ImplBlock { methods, .. } => {
            for m in methods {
                check_stmt(m, env, errors);
            }
        }

        _ => {}
    }
}

fn check_return_types(body: &[Stmt], expected: &Type, env: &TypeEnv, errors: &mut Vec<TypeError>) {
    for stmt in body {
        if let StmtKind::Return(Some(expr)) = &stmt.kind
            && let Some(actual) = infer_expr_type(expr, env)
            && !types_compatible(expected, &actual)
        {
            errors.push(TypeError::new(
                format!(
                    "Return type mismatch: expected {:?}, got {:?}",
                    expected, actual
                ),
                stmt.span.line,
            ));
        }
    }
}

fn types_compatible(declared: &Type, actual: &Type) -> bool {
    match (declared, actual) {
        (Type::Nanpa, Type::Nanpa) => true,
        (Type::Kipisi, Type::Kipisi) => true,
        (Type::Kipisi, Type::Nanpa) => true,
        (Type::Sitelen, Type::Sitelen) => true,
        (Type::Lawa, Type::Lawa) => true,
        (Type::Weka, Type::Weka) => true,
        (Type::Kulupu(declared_inner), Type::Kulupu(actual_inner)) => {
            types_compatible(declared_inner, actual_inner)
        }
        (Type::Wan(_), Type::Wan(_)) => true,
        (Type::KulupuIjo(a), Type::KulupuIjo(b)) => a == b,
        (Type::PaliNasin, _) => true,
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::tokenize;
    use crate::parser::parse;

    fn check_src(src: &str) -> Vec<TypeError> {
        let tokens = tokenize(src);
        let program = parse(tokens).expect("parse failed");
        check(&program)
    }

    #[test]
    fn warns_on_function_call_argument_type_mismatch() {
        let errors = check_src(
            r#"pali add(a: nanpa_kind, b: nanpa_kind) -> nanpa_kind { pana a + b }
add("text", 1)"#,
        );

        assert_eq!(errors.len(), 1);
        assert_eq!(errors[0].line, 2);
        assert!(errors[0].message.contains("add"));
        assert!(errors[0].message.contains("Nanpa"));
        assert!(errors[0].message.contains("Sitelen"));
    }

    #[test]
    fn warns_on_function_call_argument_count_mismatch() {
        let errors = check_src(
            "pali add(a: nanpa_kind, b: nanpa_kind) -> nanpa_kind { pana a + b }
add(1)",
        );

        assert_eq!(errors.len(), 1);
        assert_eq!(errors[0].line, 2);
        assert!(errors[0].message.contains("add"));
        assert!(errors[0].message.contains("expects 2 args"));
        assert!(errors[0].message.contains("got 1"));
    }

    #[test]
    fn accepts_matching_function_call_arguments() {
        let errors = check_src(
            "pali add(a: nanpa_kind, b: nanpa_kind) -> nanpa_kind { pana a + b }
add(1, 2)",
        );

        assert!(errors.is_empty());
    }

    #[test]
    fn warns_on_struct_literal_field_type_mismatch() {
        let errors = check_src(
            r#"kulupu Person { age: nanpa_kind }
ijo p = Person { age: "text" }"#,
        );

        assert_eq!(errors.len(), 1);
        assert_eq!(errors[0].line, 2);
        assert!(errors[0].message.contains("Person"));
        assert!(errors[0].message.contains("age"));
        assert!(errors[0].message.contains("Nanpa"));
        assert!(errors[0].message.contains("Sitelen"));
    }

    #[test]
    fn warns_on_mixed_array_literal_elements() {
        let errors = check_src(r#"ijo arr = [1, "hello", kin]"#);

        assert_eq!(errors.len(), 1);
        assert_eq!(errors[0].line, 1);
        assert!(errors[0].message.contains("Array element"));
        assert!(errors[0].message.contains("Nanpa"));
        assert!(errors[0].message.contains("Sitelen"));
    }

    #[test]
    fn warns_on_array_type_annotation_with_wrong_element_type() {
        let errors = check_src(r#"ijo arr: [nanpa_kind] = ["hello"]"#);

        assert_eq!(errors.len(), 1);
        assert_eq!(errors[0].line, 1);
        assert!(errors[0].message.contains("arr"));
        assert!(errors[0].message.contains("Nanpa"));
        assert!(errors[0].message.contains("Sitelen"));
    }

    #[test]
    fn infers_for_in_variable_type_from_array_elements() {
        let errors = check_src(
            r#"pali shout(val: sitelen) { toki(val) }
ijo arr = ["hello"]
sin item insa arr { shout(item) }"#,
        );

        assert!(errors.is_empty());
    }
}
