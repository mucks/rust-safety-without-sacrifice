
use std::fs;
use std::path::PathBuf;
use std::process::Command;

fn normalize(s: &str) -> String {
    s.replace("\r\n", "\n")
}

fn run_case(case: &str) -> String {
    let bin_path = env!("CARGO_BIN_EXE_tiny-cli-lab");
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests").join("data");
    let input_path = root.join(format!("{}.in", case));
    let output = Command::new(bin_path)
        .arg(&input_path)
        .output()
        .expect("failed to run binary");
    String::from_utf8_lossy(&output.stdout).to_string()
}

#[test] fn test_case1() { 
    let out = run_case("case1");
    let exp = fs::read_to_string(PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests").join("data").join("case1.out")).unwrap();
    assert_eq!(normalize(&out), normalize(&exp));
}

#[test] fn test_case2() { 
    let out = run_case("case2");
    let exp = fs::read_to_string(PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests").join("data").join("case2.out")).unwrap();
    assert_eq!(normalize(&out), normalize(&exp));
}

#[test] fn test_case3() { 
    let out = run_case("case3");
    let exp = fs::read_to_string(PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests").join("data").join("case3.out")).unwrap();
    assert_eq!(normalize(&out), normalize(&exp));
}

#[test] fn test_case4() { 
    let out = run_case("case4");
    let exp = fs::read_to_string(PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests").join("data").join("case4.out")).unwrap();
    assert_eq!(normalize(&out), normalize(&exp));
}

#[test] fn test_case5() { 
    let out = run_case("case5");
    let exp = fs::read_to_string(PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests").join("data").join("case5.out")).unwrap();
    assert_eq!(normalize(&out), normalize(&exp));
}
