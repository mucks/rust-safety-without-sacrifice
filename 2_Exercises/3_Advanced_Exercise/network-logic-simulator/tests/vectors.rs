
use std::fs;
use std::path::PathBuf;
use std::process::Command;
use std::time::Instant;

fn normalize(s: &str) -> String {
    s.replace("\r\n", "\n")
}

fn run_case(case: &str) -> String {
    let bin_path = env!("CARGO_BIN_EXE_network-logic-simulator");
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests").join("data");
    let input_path = root.join(format!("{}.in", case));
    let output = Command::new(bin_path)
        .arg(&input_path)
        .output()
        .expect("failed to run binary");
    String::from_utf8_lossy(&output.stdout).to_string()
}

#[test]
fn test_case1_summary() {
    let out = run_case("case1");
    let expected = fs::read_to_string(
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests").join("data").join("case1.out")
    ).unwrap();
    assert_eq!(normalize(&out), normalize(&expected));
}

#[test]
fn test_case2_routing() {
    let out = run_case("case2");
    let expected = fs::read_to_string(
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests").join("data").join("case2.out")
    ).unwrap();
    assert_eq!(normalize(&out), normalize(&expected));
}

#[test]
fn test_case3_dag_schedule() {
    let out = run_case("case3");
    let expected = fs::read_to_string(
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests").join("data").join("case3.out")
    ).unwrap();
    assert_eq!(normalize(&out), normalize(&expected));
}

#[test]
fn test_case4_threads_timing() {
    let bin_path = env!("CARGO_BIN_EXE_network-logic-simulator");
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests").join("data");
    let input_path = root.join("case4.in");
    let start = Instant::now();
    let output = Command::new(bin_path).arg(&input_path).output().expect("run");
    let elapsed_ms = start.elapsed().as_millis() as u64;

    let out = String::from_utf8_lossy(&output.stdout).to_string();
    let expected = fs::read_to_string(root.join("case4.out")).unwrap();
    assert_eq!(normalize(&out), normalize(&expected), "output mismatch");

    // Under concurrent implementation, simulated total ~40ms path max; allow headroom:
    const MAX_MS: u64 = 500;
    assert!(elapsed_ms <= MAX_MS, "threads too slow: {}ms > {}ms", elapsed_ms, MAX_MS);
}

#[test]
fn test_case5_async_timing() {
    let bin_path = env!("CARGO_BIN_EXE_network-logic-simulator");
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests").join("data");
    let input_path = root.join("case5.in");
    let start = Instant::now();
    let output = Command::new(bin_path).arg(&input_path).output().expect("run");
    let elapsed_ms = start.elapsed().as_millis() as u64;

    let out = String::from_utf8_lossy(&output.stdout).to_string();
    let expected = fs::read_to_string(root.join("case5.out")).unwrap();
    assert_eq!(normalize(&out), normalize(&expected), "output mismatch");

    const MAX_MS: u64 = 500;
    assert!(elapsed_ms <= MAX_MS, "async too slow: {}ms > {}ms", elapsed_ms, MAX_MS);
}
