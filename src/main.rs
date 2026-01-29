use clap::{Parser, Subcommand};
use std::env;
use std::fs;
use std::io::ErrorKind;
use std::process;

#[derive(Parser)]
#[command(name = "c2rust-init")]
#[command(about = "初始化 c2rust 项目结构", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// 初始化 .c2rust 目录
    Init,
}

/// Print shell-specific instructions for setting the C2RUST_PROJECT_ROOT environment variable
fn print_env_var_instructions(dir_str: &str) {
    if cfg!(windows) {
        println!("若要在当前 shell 会话中使用该环境变量，请根据所用 shell 运行：");
        println!("  在 cmd.exe 中：");
        println!("    set \"C2RUST_PROJECT_ROOT={}\"", dir_str);
        println!("  在 PowerShell 中：");
        let ps_escaped = dir_str.replace("'", "''");
        println!("    $env:C2RUST_PROJECT_ROOT = '{}'", ps_escaped);
    } else {
        println!("若要在当前 shell 会话中使用该环境变量，请运行：");
        let posix_escaped = dir_str.replace("'", "'\\''");
        println!("    export C2RUST_PROJECT_ROOT='{}'", posix_escaped);
    }
}

fn init_c2rust_dir() -> Result<(), Box<dyn std::error::Error>> {
    // Get the current directory as absolute path
    let current_dir = env::current_dir().map_err(|e| {
        eprintln!("错误: 无法获取当前目录: {}", e);
        e
    })?;
    let c2rust_dir = current_dir.join(".c2rust");

    // Set C2RUST_PROJECT_ROOT before git2 initialization
    // SAFETY: We are still executing on the main thread before any call into git2 (which may
    // spawn threads internally), and clap's argument parsing does not spawn threads.
    // Therefore, no concurrent access to process environment variables can occur here,
    // making this unsafe call to `env::set_var` sound.
    unsafe {
        env::set_var("C2RUST_PROJECT_ROOT", current_dir.as_os_str());
    }

    // Create .c2rust directory
    match fs::create_dir(&c2rust_dir) {
        Ok(_) => {
            // Success message deferred until after git init to avoid misleading output
            // if git init fails and the directory is cleaned up
        }
        Err(e) if e.kind() == ErrorKind::AlreadyExists => {
            // `.c2rust` already exists; check whether it's actually a directory
            match fs::metadata(&c2rust_dir) {
                Ok(metadata) => {
                    if metadata.is_dir() {
                        eprintln!("错误: 目录 '.c2rust' 已存在");
                        return Err(
                            std::io::Error::new(ErrorKind::AlreadyExists, "目录已存在").into()
                        );
                    } else {
                        eprintln!("错误: 路径 '.c2rust' 已存在且不是目录");
                        return Err(std::io::Error::new(
                            ErrorKind::AlreadyExists,
                            "路径已存在且不是目录",
                        )
                        .into());
                    }
                }
                Err(meta_err) => {
                    eprintln!("错误: 无法获取 '.c2rust' 的元数据: {}", meta_err);
                    return Err(meta_err.into());
                }
            }
        }
        Err(e) => {
            eprintln!("创建目录 '.c2rust' 失败: {}", e);
            return Err(e.into());
        }
    }

    // Initialize git repository in .c2rust directory
    match git2::Repository::init(&c2rust_dir) {
        Ok(_) => {
            // Success - print messages now that all operations succeeded
            println!("已创建目录: .c2rust");
            println!("已在 .c2rust 目录初始化 Git 仓库");
        }
        Err(e) => {
            eprintln!("初始化 Git 仓库失败: {}", e);
            // Clean up the directory if git init fails to avoid partial state
            if let Err(cleanup_err) = fs::remove_dir_all(&c2rust_dir) {
                eprintln!("警告: 清理目录失败: {}", cleanup_err);
                eprintln!("请手动删除目录: {}", c2rust_dir.display());
            }
            return Err(e.into());
        }
    }

    // Print instructions for setting environment variable in shell after all operations succeed
    println!(
        "c2rust 项目已初始化，项目根目录为：{}",
        current_dir.display()
    );

    // Use explicit UTF-8 conversion to avoid lossy conversion with display()
    match current_dir.to_str() {
        Some(dir_str) => print_env_var_instructions(dir_str),
        None => {
            println!("注意: 当前路径包含非 UTF-8 字符，无法生成 shell 命令。");
            println!("环境变量 C2RUST_PROJECT_ROOT 已在当前进程中设置，但您需要手动配置 shell。");
        }
    }

    Ok(())
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Init => {
            if init_c2rust_dir().is_err() {
                process::exit(1);
            }
        }
    }
}
