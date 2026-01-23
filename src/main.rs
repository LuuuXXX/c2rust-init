use clap::{Parser, Subcommand};
use std::fs;
use std::io::ErrorKind;
use std::process;

#[derive(Parser)]
#[command(name = "c2rust-init")]
#[command(about = "Initialize c2rust project structure", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialize the .c2rust directory
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
            if let Err(_) = init_c2rust_dir() {
                process::exit(1);
            }
        }
    }
}
