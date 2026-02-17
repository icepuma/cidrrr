use std::process::{Command, Output};

fn run_raw(args: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_cidrrr"))
        .args(args)
        .output()
        .expect("running cidrrr should work")
}

fn run(args: &[&str]) -> String {
    let output = run_raw(args);
    assert!(output.status.success());
    String::from_utf8(output.stdout).expect("stdout should be valid UTF-8")
}

#[test]
fn plain_output_includes_count() {
    let output = run(&["10.105.4.0/24"]);

    assert_eq!(output, "count: 254\n10.105.4.1\n10.105.4.254\n");
}

#[test]
fn json_output_shape_is_unchanged() {
    let output = run(&["--output", "json", "10.105.4.0/24"]);

    assert_eq!(output, "[\"10.105.4.1\",\"10.105.4.254\"]\n");
}

#[test]
fn csv_output_shape_is_unchanged() {
    let output = run(&["--output", "csv", "10.105.4.0/24"]);

    assert_eq!(output, "\"ips\"\n\"10.105.4.1\"\n\"10.105.4.254\"\n");
}

#[test]
fn invalid_cidr_returns_error_on_stderr() {
    let output = run_raw(&["invalid"]);

    assert!(!output.status.success());

    let stderr = String::from_utf8(output.stderr).expect("stderr should be valid UTF-8");
    assert!(stderr.starts_with("Error: "));
    assert!(stderr.contains("is an invalid CIDR"));
}
