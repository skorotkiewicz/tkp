use std::fs;
use std::path::PathBuf;
use std::process::Command;
use std::sync::Mutex;
use std::time::{SystemTime, UNIX_EPOCH};

fn run_interpret(file: &str) -> String {
    let output = Command::new("cargo")
        .args(["run", "--quiet", "--", "interpret", file])
        .output()
        .expect("failed to execute tkp");

    assert!(
        output.status.success(),
        "tkp interpret {} failed: {}",
        file,
        String::from_utf8_lossy(&output.stderr)
    );

    String::from_utf8(output.stdout)
        .expect("invalid utf8")
        .trim()
        .to_string()
}

static BUILD_LOCK: Mutex<()> = Mutex::new(());

fn build_and_run(source: &str, stem_prefix: &str) -> String {
    let _lock = BUILD_LOCK.lock().unwrap_or_else(|err| err.into_inner());
    let unique_suffix = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("time went backwards")
        .as_nanos();
    let stem = format!("{}_{}", stem_prefix, unique_suffix);
    let source_path = std::env::temp_dir().join(format!("{}.tkp", stem));
    let output_path = PathBuf::from(format!("./{}", stem));

    fs::write(&source_path, source).expect("failed to write temp source");

    let build_output = Command::new("cargo")
        .args([
            "run",
            "--quiet",
            "--",
            "build",
            source_path
                .to_str()
                .expect("temp path should be valid utf8"),
        ])
        .output()
        .expect("failed to execute tkp build");

    assert!(
        build_output.status.success(),
        "tkp build failed: {}",
        String::from_utf8_lossy(&build_output.stderr)
    );

    let run_output = Command::new(&output_path)
        .output()
        .expect("failed to execute built binary");

    let _ = fs::remove_file(&source_path);
    let _ = fs::remove_file(&output_path);

    assert!(
        run_output.status.success(),
        "built binary failed: {}",
        String::from_utf8_lossy(&run_output.stderr)
    );

    String::from_utf8(run_output.stdout)
        .expect("invalid utf8")
        .trim()
        .to_string()
}

#[test]
fn test_hello() {
    let out = run_interpret("examples/toki.tkp");
    assert_eq!(out, "toki! mi jan Pali");
}

#[test]
fn test_fibonacci() {
    let out = run_interpret("examples/fibonacci.tkp");
    assert_eq!(out, "55");
}

#[test]
fn test_factorial() {
    let out = run_interpret("examples/factorial.tkp");
    assert_eq!(out, "3628800");
}

#[test]
fn test_sum() {
    let out = run_interpret("examples/sum.tkp");
    assert_eq!(out, "5050");
}

#[test]
fn test_even_odd() {
    let out = run_interpret("examples/even_odd.tkp");
    let lines: Vec<&str> = out.lines().collect();
    assert_eq!(lines.len(), 10);
    assert_eq!(lines[0], "odd");
    assert_eq!(lines[1], "even");
    assert_eq!(lines[9], "even");
}

#[test]
fn test_compiled_backend_range_for_in_outputs_sequence() {
    let out = build_and_run("sin i insa 0..5 {\n    toki(i)\n}\n", "tkp_range_for_in");

    assert_eq!(out, "0\n1\n2\n3\n4");
}

#[test]
fn test_compiled_backend_array_for_in_outputs_items() {
    let out = build_and_run(
        "ijo values = [3, 4, 5]\nsin value insa values {\n    toki(value)\n}\n",
        "tkp_array_for_in",
    );

    assert_eq!(out, "3\n4\n5");
}

#[test]
fn test_compiled_backend_try_catch_handles_division_by_zero() {
    let out = build_and_run(
        "lukin {\n    ijo result = 1 / 0\n    toki(111)\n} alasa(err) {\n    toki(222)\n}\n",
        "tkp_try_catch",
    );

    assert_eq!(out, "222");
}

#[test]
fn test_compiled_backend_string_method_length() {
    let out = build_and_run("toki(\"hello\".suli_ijo())\n", "tkp_string_len");

    assert_eq!(out, "5");
}

#[test]
fn test_compiled_backend_array_method_length() {
    let out = build_and_run(
        "ijo values: [nanpa_kind] = [3, 4, 5]\ntoki(values.suli_ijo())\n",
        "tkp_array_len",
    );

    assert_eq!(out, "3");
}

#[test]
#[ignore = "codegen: method call codegen not yet implemented"]
fn test_compiled_backend_struct_impl_method_call() {
    let out = build_and_run(
        "kulupu Rect { width: nanpa_kind, height: nanpa_kind }\nken Rect {\n    pali area(mi: Rect) -> nanpa_kind {\n        pana mi.width * mi.height\n    }\n}\nijo rect: Rect = Rect { width: 2, height: 3 }\ntoki(rect.area())\n",
        "tkp_struct_method",
    );

    assert_eq!(out, "6");
}

#[test]
#[ignore = "codegen: enum variant IR generation in progress"]
fn test_compiled_backend_enum_match_branches_by_variant_tag() {
    let out = build_and_run(
        "nanpa Direction { Up, Down }
ijo dir = Direction::Down
sama dir {
    Up => toki(11)
    Down => toki(22)
    _ => toki(33)
}
",
        "tkp_enum_match",
    );

    assert_eq!(out, "22");
}

#[test]
fn test_compiled_backend_lambda_outputs_value() {
    let out = build_and_run(
        "ijo double = pali(x: nanpa_kind) {
    pana x * 2
}
toki(double(5))
",
        "tkp_lambda_basic",
    );

    assert_eq!(out, "10");
}

#[test]
#[ignore = "codegen: Toki Pona identifiers in LLVM IR not yet supported"]
fn test_compiled_backend_closure_captures_outer_variable() {
    let out = build_and_run(
        "ijo multiplier = 3
ijo multiply = pali(x: nanpa_kind) {
    pana x * multiplier
}
toki(multiply(4))
",
        "tkp_lambda_capture",
    );

    assert_eq!(out, "12");
}
