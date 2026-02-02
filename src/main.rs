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
    let current_dir = env::current_dir().map_err(|e| {
        eprintln!("错误: 无法获取当前目录: {}", e);
        e
    })?;
    let c2rust_dir = current_dir.join(".c2rust");

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
        Ok(repo) => {
            // 配置仓库级别的 user.name 和 user.email（非 --global）
            let mut git_config_success = true;
            match repo.config() {
                Ok(mut config) => {
                    // 设置默认的 user.name 和 user.email
                    if let Err(e) = config.set_str("user.name", "c2rust-auto") {
                        eprintln!("警告: 无法设置 git user.name: {}", e);
                        git_config_success = false;
                    }
                    if let Err(e) = config.set_str("user.email", "c2rust-auto@localhost") {
                        eprintln!("警告: 无法设置 git user.email: {}", e);
                        git_config_success = false;
                    }
                }
                Err(e) => {
                    eprintln!("警告: 无法获取 git 配置: {}", e);
                    git_config_success = false;
                }
            }

            // Success - print messages now that all operations succeeded
            println!("已创建目录: .c2rust");
            println!("已在 .c2rust 目录初始化 Git 仓库");
            if git_config_success {
                println!("已配置默认的 git user.name 和 user.email");
            }
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

    // Print success message after all operations succeed
    println!(
        "c2rust 项目已初始化，项目根目录为：{}",
        current_dir.display()
    );

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
