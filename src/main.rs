use clap::{Parser, Subcommand};
use std::fs;
use std::io::{self, Write};
use std::path::Path;
use std::process::Command;
use tkp::codegen::CodeGen;
use tkp::interpreter::Interpreter;
use tkp::lexer::tokenize;
use tkp::parser::parse;
use tkp::typechecker;

#[derive(Parser)]
#[command(name = "tkp", about = "Toki Pona (TKP) Programming Language")]
#[command(version = "0.1.0")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Build a .tkp file to a native binary
    Build { file: String },
    /// Run a .tkp file using the JIT compiler
    Run { file: String },
    /// Execute a .tkp file using the interpreter
    Interpret { file: String },
    /// Start the interactive REPL
    Repl,
    /// Start the Language Server
    Lsp,
}

fn handle_type_errors(errors: Vec<typechecker::TypeError>) {
    for err in errors {
        if err.line > 0 {
            eprintln!("[Type Warning] line {}: {}", err.line, err.message);
        } else {
            eprintln!("[Type Warning] {}", err.message);
        }
    }
}

fn run_build(file: &str) -> Result<(), String> {
    let source = fs::read_to_string(file).map_err(|e| format!("Failed to read file: {}", e))?;
    let tokens = tokenize(&source);
    let program =
        parse(tokens).map_err(|e| format!("[Parse Error] line {}: {}", e.line, e.message))?;

    let type_errors = typechecker::check(&program);
    handle_type_errors(type_errors);

    let mut cg = CodeGen::new();
    let ir = cg.generate(&program);

    let output_name = Path::new(file)
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("output");

    let ll_path = format!("/tmp/{}_build.ll", output_name);
    fs::write(&ll_path, &ir).map_err(|e| format!("Failed to write temporary IR file: {}", e))?;

    let clang_status = Command::new("clang")
        .args([&ll_path, "-o", output_name, "-Wno-override-module"])
        .status();

    match clang_status {
        Ok(s) if s.success() => {
            println!("Build complete: {}", output_name);
            Ok(())
        }
        Ok(_) => Err("Compilation failed (clang exit code mismatch)".to_string()),
        Err(e) => Err(format!("Failed to execute clang: {}", e)),
    }
}

fn run_run(file: &str) -> Result<(), String> {
    let source = fs::read_to_string(file).map_err(|e| format!("Failed to read file: {}", e))?;
    let tokens = tokenize(&source);
    let program =
        parse(tokens).map_err(|e| format!("[Parse Error] line {}: {}", e.line, e.message))?;

    let type_errors = typechecker::check(&program);
    handle_type_errors(type_errors);

    let mut cg = CodeGen::new();
    let ir = cg.generate(&program);

    let ll_path = "/tmp/tkp_temp.ll";
    let bin_path = "/tmp/tkp_temp.bin";
    fs::write(ll_path, &ir).map_err(|e| format!("Failed to write temporary IR file: {}", e))?;

    let clang_status = Command::new("clang")
        .args([ll_path, "-o", bin_path, "-Wno-override-module"])
        .status();

    match clang_status {
        Ok(s) if s.success() => {
            let run_status = Command::new(bin_path).status();
            match run_status {
                Ok(s) => {
                    let _ = fs::remove_file(ll_path);
                    let _ = fs::remove_file(bin_path);
                    if s.success() {
                        Ok(())
                    } else {
                        Err(format!("Binary exited with failure status: {:?}", s.code()))
                    }
                }
                Err(e) => Err(format!("Failed to execute binary: {}", e)),
            }
        }
        Ok(_) => Err("Compilation failed (clang)".to_string()),
        Err(e) => Err(format!("Failed to execute clang: {}", e)),
    }
}

fn run_interpret(file: &str) -> Result<(), String> {
    let source = fs::read_to_string(file).map_err(|e| format!("Failed to read file: {}", e))?;
    let tokens = tokenize(&source);
    let program =
        parse(tokens).map_err(|e| format!("[Parse Error] line {}: {}", e.line, e.message))?;

    let type_errors = typechecker::check(&program);
    handle_type_errors(type_errors);

    let mut interpreter = Interpreter::new();
    let _ = interpreter.interpret(&program);
    Ok(())
}

fn run_repl() {
    println!("\n ◓◒ ─────────────────────────────────── ◓◒\n");
    println!("     TKP (Toki Pona) Programming Language v0.1.0");
    println!("     A minimalist language based on Han\n");
    println!(" ◓◒ ─────────────────────────────────── ◓◒\n");
    println!("Exit: Ctrl+D or type 'pini'\n");

    let mut interpreter = Interpreter::new();
    let stdin = io::stdin();
    let mut input = String::new();

    loop {
        print!("tp> ");
        io::stdout().flush().unwrap();

        input.clear();
        if stdin.read_line(&mut input).is_err() || input.trim() == "pini" || input.is_empty() {
            println!("\npona tawa sina! (Goodbye!)");
            break;
        }

        let tokens = tokenize(&input);
        match parse(tokens) {
            Ok(program) => {
                let type_errors = typechecker::check(&program);
                handle_type_errors(type_errors);

                if let Err(e) = interpreter.eval_program(&program) {
                    if e.line > 0 {
                        eprintln!("[Error] line {}: {}", e.line, e.message);
                    } else {
                        eprintln!("[Error] {}", e.message);
                    }
                }
            }
            Err(e) => {
                eprintln!("[Parse Error] {}", e.message);
            }
        }
    }
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Build { file } => {
            if let Err(e) = run_build(&file) {
                eprintln!("[Build Error] {}", e);
                std::process::exit(1);
            }
        }
        Commands::Run { file } => {
            if let Err(e) = run_run(&file) {
                eprintln!("[Run Error] {}", e);
                std::process::exit(1);
            }
        }
        Commands::Interpret { file } => {
            if let Err(e) = run_interpret(&file) {
                eprintln!("[Interpret Error] {}", e);
                std::process::exit(1);
            }
        }
        Commands::Repl => {
            run_repl();
        }
        Commands::Lsp => {
            #[cfg(feature = "native")]
            {
                if let Err(e) = tkp::lsp::run_server() {
                    eprintln!("[LSP Error] {}", e);
                    std::process::exit(1);
                }
            }
            #[cfg(not(feature = "native"))]
            {
                eprintln!("LSP is only supported in native builds");
                std::process::exit(1);
            }
        }
    }
}
