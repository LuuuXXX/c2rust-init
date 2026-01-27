use clap::{Parser, Subcommand};
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

fn init_c2rust_dir() -> Result<(), std::io::Error> {
    match fs::create_dir(".c2rust") {
        Ok(_) => {
            println!("已创建目录: .c2rust");
            Ok(())
        }
        Err(e) if e.kind() == ErrorKind::AlreadyExists => {
            println!("目录已存在: .c2rust");
            Ok(())
        }
        Err(e) => {
            eprintln!("创建目录 '.c2rust' 失败: {}", e);
            Err(e)
        }
    }
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
