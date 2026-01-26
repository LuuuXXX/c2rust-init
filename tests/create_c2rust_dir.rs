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
    assert!(
        stdout.contains("已创建配置文件: .c2rust/config.toml"),
        "Expected config file creation message in stdout, got: {}",
        stdout
    );

    // Verify the directory was created
    let c2rust_path = temp_path.join(".c2rust");
    assert!(
        c2rust_path.exists() && c2rust_path.is_dir(),
        "Directory .c2rust should exist"
    );

    // Verify the config file was created
    let config_path = temp_path.join(".c2rust/config.toml");
    assert!(
        config_path.exists() && config_path.is_file(),
        "Config file .c2rust/config.toml should exist"
    );

    // Verify the config file has content
    let config_content = fs::read_to_string(&config_path).expect("Failed to read config file");
    assert!(
        config_content.contains("[global]"),
        "Config file should contain [global] section"
    );
    assert!(
        config_content.contains("[model]"),
        "Config file should contain [model] section"
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

    // Check that the command succeeded (directory already exists is not an error)
    assert!(output.status.success(), "Command should succeed");

    // Check stdout contains already exists message
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("目录已存在: .c2rust"),
        "Expected 'already exists' message in stdout, got: {}",
        stdout
    );
    assert!(
        stdout.contains("已创建配置文件: .c2rust/config.toml"),
        "Expected config file creation message in stdout, got: {}",
        stdout
    );

    // Verify the directory still exists
    assert!(
        c2rust_path.exists() && c2rust_path.is_dir(),
        "Directory .c2rust should still exist"
    );

    // Verify the config file was created
    let config_path = temp_path.join(".c2rust/config.toml");
    assert!(
        config_path.exists() && config_path.is_file(),
        "Config file .c2rust/config.toml should exist"
    );
}

/// Test that the binary doesn't overwrite existing config file
#[test]
fn test_config_file_already_exists() {
    let temp_dir = TempDir::new().unwrap();
    let temp_path = temp_dir.path();

    // Pre-create the .c2rust directory
    let c2rust_path = temp_path.join(".c2rust");
    fs::create_dir(&c2rust_path).expect("Failed to pre-create directory");

    // Pre-create the config file with custom content
    let config_path = c2rust_path.join("config.toml");
    let custom_content = "# Custom configuration\n";
    fs::write(&config_path, custom_content).expect("Failed to create config file");

    // Run the binary in the temp directory
    let output = Command::new(env!("CARGO_BIN_EXE_c2rust-init"))
        .arg("init")
        .current_dir(temp_path)
        .output()
        .expect("Failed to execute binary");

    // Check that the command succeeded
    assert!(output.status.success(), "Command should succeed");

    // Check stdout contains messages
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("目录已存在: .c2rust"),
        "Expected 'already exists' message in stdout, got: {}",
        stdout
    );
    assert!(
        stdout.contains("配置文件已存在: .c2rust/config.toml"),
        "Expected config file already exists message in stdout, got: {}",
        stdout
    );

    // Verify the config file still has the custom content (not overwritten)
    let config_content = fs::read_to_string(&config_path).expect("Failed to read config file");
    assert_eq!(
        config_content, custom_content,
        "Config file should not be overwritten"
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
    // - Some platforms may fail with an error (exit code != 0)
    // - Some platforms may treat it as AlreadyExists
    // We need to handle both cases as acceptable

    if !output.status.success() {
        // If it failed, stderr should contain error message
        let stderr = String::from_utf8_lossy(&output.stderr);
        assert!(
            stderr.contains("创建目录") || stderr.contains("创建配置文件"),
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
