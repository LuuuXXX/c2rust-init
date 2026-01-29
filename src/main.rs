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

fn init_c2rust_dir() -> Result<(), Box<dyn std::error::Error>> {
    // Get the current directory as absolute path
    let current_dir = env::current_dir()?;
    let c2rust_dir = current_dir.join(".c2rust");

    // Create .c2rust directory
    match fs::create_dir(&c2rust_dir) {
        Ok(_) => {
            println!("已创建目录: .c2rust");
        }
        Err(e) if e.kind() == ErrorKind::AlreadyExists => {
            eprintln!("错误: 目录 '.c2rust' 已存在");
            return Err("目录已存在".into());
        }
        Err(e) => {
            eprintln!("创建目录 '.c2rust' 失败: {}", e);
            return Err(e.into());
        }
    }

    // Initialize git repository in .c2rust directory
    match git2::Repository::init(&c2rust_dir) {
        Ok(_) => {
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

    // Set C2RUST_PROJECT_ROOT environment variable
    let project_root = current_dir
        .to_str()
        .ok_or("当前路径包含无效的 UTF-8 字符")?
        .to_string();
    // SAFETY: This is safe because we're in a single-threaded context (main function,
    // before any threads are spawned), and the environment variable is being set
    // with valid UTF-8 strings that we control.
    unsafe {
        env::set_var("C2RUST_PROJECT_ROOT", &project_root);
    }
    println!("已设置环境变量 C2RUST_PROJECT_ROOT={}", project_root);

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
