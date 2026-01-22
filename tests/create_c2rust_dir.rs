use std::fs;
use std::process::Command;
use tempfile::TempDir;

/// Test that the binary successfully creates the .c2rust directory
#[test]
fn test_create_c2rust_dir_success() {
    let temp_dir = TempDir::new().unwrap();
    let temp_path = temp_dir.path();

    // Run the binary in the temp directory
    let output = Command::new(env!("CARGO_BIN_EXE_c2rust-init"))
        .current_dir(temp_path)
        .output()
        .expect("Failed to execute binary");

    // Check that the command succeeded
    assert!(output.status.success(), "Command should succeed");

    // Check stdout contains success message
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("已创建目录: .c2rust"),
        "Expected success message in stdout, got: {}",
        stdout
    );

    // Verify the directory was created
    let c2rust_path = temp_path.join(".c2rust");
    assert!(
        c2rust_path.exists() && c2rust_path.is_dir(),
        "Directory .c2rust should exist"
    );
}

/// Test that the binary handles the case when .c2rust directory already exists
#[test]
fn test_create_c2rust_dir_already_exists() {
    let temp_dir = TempDir::new().unwrap();
    let temp_path = temp_dir.path();

    // Pre-create the .c2rust directory
    let c2rust_path = temp_path.join(".c2rust");
    fs::create_dir(&c2rust_path).expect("Failed to pre-create directory");

    // Run the binary in the temp directory
    let output = Command::new(env!("CARGO_BIN_EXE_c2rust-init"))
        .current_dir(temp_path)
        .output()
        .expect("Failed to execute binary");

    // Check that the command succeeded (directory already exists is not an error)
    assert!(output.status.success(), "Command should succeed");

    // Check stdout contains already exists message
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("目录已存在: .c2rust"),
        "Expected 'already exists' message in stdout, got: {}",
        stdout
    );

    // Verify the directory still exists
    assert!(
        c2rust_path.exists() && c2rust_path.is_dir(),
        "Directory .c2rust should still exist"
    );
}

/// Test that the binary handles failure scenarios appropriately
#[test]
fn test_create_c2rust_dir_failure() {
    let temp_dir = TempDir::new().unwrap();
    let temp_path = temp_dir.path();

    // Pre-create .c2rust as a file (not a directory) to cause a conflict
    let c2rust_path = temp_path.join(".c2rust");
    fs::write(&c2rust_path, "test").expect("Failed to create file");

    // Run the binary in the temp directory
    let output = Command::new(env!("CARGO_BIN_EXE_c2rust-init"))
        .current_dir(temp_path)
        .output()
        .expect("Failed to execute binary");

    // The behavior may vary by platform:
    // - Some platforms may fail with an error (exit code != 0)
    // - Some platforms may treat it as AlreadyExists
    // We need to handle both cases as acceptable

    if !output.status.success() {
        // If it failed, stderr should contain error message
        let stderr = String::from_utf8_lossy(&output.stderr);
        assert!(
            stderr.contains("创建目录失败"),
            "Expected error message in stderr, got: {}",
            stderr
        );
    } else {
        // If it succeeded (treated as already exists), stdout should indicate that
        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(
            stdout.contains("目录已存在: .c2rust"),
            "Expected 'already exists' message in stdout when treated as success, got: {}",
            stdout
        );
    }

    // The file should still exist (not replaced)
    assert!(c2rust_path.exists(), "Path .c2rust should still exist");
}
