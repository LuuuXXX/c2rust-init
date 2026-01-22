use c2rust_init::create_c2rust_dir;
use std::env;
use std::fs;
use std::io::ErrorKind;
use std::path::PathBuf;

/// Helper function to create a temporary test directory and change to it
fn setup_test_dir(test_name: &str) -> PathBuf {
    let temp_dir = env::temp_dir().join(format!("c2rust_test_{}", test_name));
    
    // Clean up if directory exists from previous test run
    if temp_dir.exists() {
        fs::remove_dir_all(&temp_dir).ok();
    }
    
    fs::create_dir_all(&temp_dir).expect("Failed to create test directory");
    temp_dir
}

/// Helper function to clean up test directory
fn cleanup_test_dir(dir: &PathBuf) {
    if dir.exists() {
        fs::remove_dir_all(dir).ok();
    }
}

#[test]
fn test_create_c2rust_dir_success() {
    let test_dir = setup_test_dir("success");
    let original_dir = env::current_dir().unwrap();
    
    // Change to test directory
    env::set_current_dir(&test_dir).expect("Failed to change to test directory");
    
    // Test creating the directory
    let result = create_c2rust_dir();
    assert!(result.is_ok(), "Failed to create .c2rust directory");
    
    // Verify the directory was created
    let c2rust_dir = test_dir.join(".c2rust");
    assert!(c2rust_dir.exists(), ".c2rust directory was not created");
    assert!(c2rust_dir.is_dir(), ".c2rust is not a directory");
    
    // Change back to original directory
    env::set_current_dir(original_dir).unwrap();
    
    cleanup_test_dir(&test_dir);
}

#[test]
fn test_create_c2rust_dir_already_exists() {
    let test_dir = setup_test_dir("already_exists");
    let original_dir = env::current_dir().unwrap();
    
    // Change to test directory
    env::set_current_dir(&test_dir).expect("Failed to change to test directory");
    
    // Create the directory first
    let c2rust_dir = test_dir.join(".c2rust");
    fs::create_dir(&c2rust_dir).expect("Failed to pre-create .c2rust directory");
    
    // Try to create it again
    let result = create_c2rust_dir();
    assert!(result.is_err(), "Expected error when directory already exists");
    
    // Verify the error kind is AlreadyExists
    let err = result.unwrap_err();
    assert_eq!(
        err.kind(),
        ErrorKind::AlreadyExists,
        "Expected AlreadyExists error, got: {:?}",
        err.kind()
    );
    
    // Change back to original directory
    env::set_current_dir(original_dir).unwrap();
    
    cleanup_test_dir(&test_dir);
}

#[test]
fn test_create_c2rust_dir_permission_denied() {
    // This test attempts to create a directory where we don't have permission.
    // Note: This test may not work on all systems or with all permissions
    
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        
        let test_dir = setup_test_dir("permission_denied");
        let original_dir = env::current_dir().unwrap();
        
        // Create a read-only directory
        let readonly_dir = test_dir.join("readonly");
        fs::create_dir(&readonly_dir).expect("Failed to create readonly directory");
        
        // Set permissions to read-only (no write permission)
        let mut perms = fs::metadata(&readonly_dir).unwrap().permissions();
        perms.set_mode(0o444); // r--r--r--
        fs::set_permissions(&readonly_dir, perms).expect("Failed to set permissions");
        
        // Change to readonly directory
        env::set_current_dir(&readonly_dir).ok();
        
        // Try to create .c2rust directory (should fail due to permissions)
        let result = create_c2rust_dir();
        
        // Change back before cleanup
        env::set_current_dir(&original_dir).unwrap();
        
        // Restore permissions for cleanup
        let mut perms = fs::metadata(&readonly_dir).unwrap().permissions();
        perms.set_mode(0o755);
        fs::set_permissions(&readonly_dir, perms).ok();
        
        cleanup_test_dir(&test_dir);
        
        // Verify we got a permission denied error
        if let Err(e) = result {
            assert_eq!(
                e.kind(),
                ErrorKind::PermissionDenied,
                "Expected PermissionDenied error, got: {:?}",
                e.kind()
            );
        } else {
            // On some systems this might succeed, which is also valid
            // so we don't fail the test
            println!("Note: Permission test succeeded (system may have different permission handling)");
        }
    }
    
    #[cfg(not(unix))]
    {
        // Skip this test on non-Unix systems
        println!("Skipping permission test on non-Unix system");
    }
}

#[test]
fn test_create_c2rust_dir_idempotent_check() {
    let test_dir = setup_test_dir("idempotent");
    let original_dir = env::current_dir().unwrap();
    
    // Change to test directory
    env::set_current_dir(&test_dir).expect("Failed to change to test directory");
    
    // First creation should succeed
    let result1 = create_c2rust_dir();
    assert!(result1.is_ok(), "First creation should succeed");
    
    // Second creation should fail with AlreadyExists
    let result2 = create_c2rust_dir();
    assert!(result2.is_err(), "Second creation should fail");
    assert_eq!(result2.unwrap_err().kind(), ErrorKind::AlreadyExists);
    
    // Change back to original directory
    env::set_current_dir(original_dir).unwrap();
    
    cleanup_test_dir(&test_dir);
}
