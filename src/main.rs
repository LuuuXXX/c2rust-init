use std::fs;
use std::io::ErrorKind;
use std::process;

fn main() {
    match fs::create_dir(".c2rust") {
        Ok(_) => {
            println!("已创建目录: .c2rust");
        }
        Err(e) if e.kind() == ErrorKind::AlreadyExists => {
            println!("目录已存在: .c2rust");
        }
        Err(e) => {
            eprintln!("创建目录 '.c2rust' 失败: {}", e);
            process::exit(1);
        }
    }
}
