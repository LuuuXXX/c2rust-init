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
        .arg("init")
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

    // Check that git repository was initialized
    assert!(
        stdout.contains("已在 .c2rust 目录初始化 Git 仓库"),
        "Expected git init message in stdout, got: {}",
        stdout
    );

    // Check that environment variable instructions were provided
    assert!(
        stdout.contains("若要在当前 shell 会话中使用该环境变量，请运行："),
        "Expected environment variable instructions in stdout, got: {}",
        stdout
    );
    assert!(
        stdout.contains("export C2RUST_PROJECT_ROOT="),
        "Expected export command in stdout, got: {}",
        stdout
    );

    // Verify the directory was created
    let c2rust_path = temp_path.join(".c2rust");
    assert!(
        c2rust_path.exists() && c2rust_path.is_dir(),
        "Directory .c2rust should exist"
    );

    // Verify git repository was initialized
    let git_path = c2rust_path.join(".git");
    assert!(
        git_path.exists() && git_path.is_dir(),
        "Git repository should be initialized in .c2rust"
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
        .arg("init")
        .current_dir(temp_path)
        .output()
        .expect("Failed to execute binary");

    // Check that the command FAILED (directory already exists is an error)
    assert!(
        !output.status.success(),
        "Command should fail when directory exists"
    );

    // Check stderr contains error message
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("错误: 目录 '.c2rust' 已存在"),
        "Expected 'already exists' error message in stderr, got: {}",
        stderr
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
        .arg("init")
        .current_dir(temp_path)
        .output()
        .expect("Failed to execute binary");

    // The behavior may vary by platform:
    // - Some platforms may fail with AlreadyExists error
    // - Some platforms may fail with a different error (NotADirectory, etc)
    // In all cases, the command should fail

    assert!(!output.status.success(), "Command should fail");

    // stderr should contain error message
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("错误") || stderr.contains("失败"),
        "Expected error message in stderr, got: {}",
        stderr
    );

    // The file should still exist (not replaced)
    assert!(c2rust_path.exists(), "Path .c2rust should still exist");
}
