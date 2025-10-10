
use std::fs;
use std::path::PathBuf;
use std::process::Command;

fn normalize(s: &str) -> String {
    s.replace("\r\n", "\n")
}

fn run_case(case: &str) {
    let bin_path = env!("CARGO_BIN_EXE_network-state-directory");
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests").join("data");
    let input_path = root.join(format!("{}.in", case));
    let expected = fs::read_to_string(root.join(format!("{}.out", case))).unwrap();
    let output = Command::new(bin_path)
        .arg(&input_path)
        .output()
        .expect("failed to run binary");
    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    assert_eq!(normalize(&stdout), normalize(&expected), "mismatch in {}", case);
}

#[test]
fn test_case1() { run_case("case1"); }

#[test]
fn test_case2() { run_case("case2"); }

#[test]
fn test_case3() { run_case("case3"); }

#[test]
fn test_case4() { run_case("case4"); }

#[test]
fn test_case5() { run_case("case5"); }
