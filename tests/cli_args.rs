use std::process::Command;

/// Test that the binary shows help message
#[test]
fn test_help_message() {
    let output = Command::new(env!("CARGO_BIN_EXE_c2rust-init"))
        .arg("--help")
        .output()
        .expect("Failed to execute binary");

    assert!(output.status.success(), "Help command should succeed");

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("Initialize c2rust project structure"),
        "Help should contain program description"
    );
    assert!(
        stdout.contains("init"),
        "Help should mention init subcommand"
    );
}

/// Test that the binary requires a subcommand
#[test]
fn test_missing_subcommand() {
    let output = Command::new(env!("CARGO_BIN_EXE_c2rust-init"))
        .output()
        .expect("Failed to execute binary");

    assert!(!output.status.success(), "Should fail without subcommand");

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("required") || stderr.contains("Usage"),
        "Error should indicate missing subcommand"
    );
}

/// Test init subcommand help
#[test]
fn test_init_help() {
    let output = Command::new(env!("CARGO_BIN_EXE_c2rust-init"))
        .arg("init")
        .arg("--help")
        .output()
        .expect("Failed to execute binary");

    assert!(output.status.success(), "Init help should succeed");

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("Initialize the .c2rust directory"),
        "Init help should contain description"
    );
}
