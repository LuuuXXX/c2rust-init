use std::fs;
use std::io;

/// Creates a `.c2rust` directory in the current working directory.
/// 
/// # Returns
/// 
/// - `Ok(())` if the directory is created successfully
/// - `Err(io::Error)` if creation fails
/// 
/// # Behavior
/// 
/// - If the directory already exists, returns an error with kind `ErrorKind::AlreadyExists`
/// - If creation fails due to other reasons (e.g., permission denied), returns the corresponding error
/// 
/// # Examples
/// 
/// ```no_run
/// use c2rust_init::create_c2rust_dir;
/// 
/// match create_c2rust_dir() {
///     Ok(()) => println!("Successfully created .c2rust directory"),
///     Err(e) if e.kind() == std::io::ErrorKind::AlreadyExists => {
///         println!(".c2rust directory already exists")
///     }
///     Err(e) => eprintln!("Failed to create .c2rust directory: {}", e),
/// }
/// ```
pub fn create_c2rust_dir() -> io::Result<()> {
    let dir_name = ".c2rust";
    fs::create_dir(dir_name)
}

/// Creates a `.c2rust` directory in the current working directory with user-friendly output.
/// 
/// This is a convenience function that calls `create_c2rust_dir()` and prints
/// appropriate messages to stdout/stderr.
/// 
/// # Examples
/// 
/// ```no_run
/// use c2rust_init::create_c2rust_dir_with_output;
/// 
/// create_c2rust_dir_with_output();
/// ```
pub fn create_c2rust_dir_with_output() {
    match create_c2rust_dir() {
        Ok(()) => println!("Successfully created .c2rust directory"),
        Err(e) if e.kind() == io::ErrorKind::AlreadyExists => {
            println!(".c2rust directory already exists")
        }
        Err(e) => eprintln!("Failed to create .c2rust directory: {}", e),
    }
}
