use clap::{Parser, Subcommand};
use std::fs;
use std::fs::File;
use std::io::ErrorKind;
use std::io::Write;
use std::path::Path;
use std::process;

const DEFAULT_CONFIG: &str = r#"[global]
# 全局设置,一般无需配置
compiler = ["gcc"]

[model]
# AI 模型相关配置
api_key = "your-api-key"
model_name = "gpt-4"

[feature.default]
# 路径相对于项目根目录(包含 .c2rust 的目录)
"clean.dir" = "build"
clean = "make clean"
"test.dir" = "build"
test = "make test"
"build.dir" = "build"
build = "make"
"#;

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

fn create_default_config() -> Result<(), std::io::Error> {
    let config_path = Path::new(".c2rust/config.toml");
    
    if config_path.exists() {
        println!("配置文件已存在: .c2rust/config.toml");
        Ok(())
    } else {
        let mut file = File::create(config_path)?;
        file.write_all(DEFAULT_CONFIG.as_bytes())?;
        println!("已创建配置文件: .c2rust/config.toml");
        Ok(())
    }
}

fn init_c2rust_dir() -> Result<(), std::io::Error> {
    match fs::create_dir(".c2rust") {
        Ok(_) => {
            println!("已创建目录: .c2rust");
            if let Err(e) = create_default_config() {
                eprintln!("创建配置文件失败: {}", e);
                return Err(e);
            }
            Ok(())
        }
        Err(e) if e.kind() == ErrorKind::AlreadyExists => {
            println!("目录已存在: .c2rust");
            if let Err(e) = create_default_config() {
                eprintln!("创建配置文件失败: {}", e);
                return Err(e);
            }
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
