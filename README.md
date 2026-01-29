# c2rust-init

c2rust-init 是一个用于初始化 c2rust 项目结构的命令行工具。它可以快速创建 `.c2rust` 目录，为使用 c2rust 工具链做好准备。

## 项目简介

c2rust-init 提供了一个简单的命令行接口来初始化 c2rust 项目的基础目录结构。该工具确保：
- 创建 `.c2rust` 目录
- 优雅地处理目录已存在的情况
- 提供清晰的错误信息

## 安装方法

### 从源码构建

确保您已安装 Rust 工具链（推荐使用 rustup）。

```bash
# 克隆仓库
git clone https://github.com/LuuuXXX/c2rust-init.git
cd c2rust-init

# 构建项目
cargo build --release

# 可选：将二进制文件安装到系统路径
cargo install --path .
```

### 系统要求

- 支持 Rust 2024 edition 的 Rust 工具链（Rust 1.85 或更高版本）
- Cargo 包管理器
- **构建依赖**：本项目使用 `git2` crate，需要系统安装以下开发包：
  - **Linux**: `libssl-dev`, `pkg-config`, `cmake` (Debian/Ubuntu: `apt-get install libssl-dev pkg-config cmake`)
  - **macOS**: 通常已预装，或通过 Homebrew: `brew install openssl pkg-config`
  - **Windows**: 需要安装 [Visual Studio Build Tools](https://visualstudio.microsoft.com/downloads/) 或使用 vcpkg

## 使用方法

### 基本用法

初始化 `.c2rust` 目录：

```bash
c2rust-init init
```

### 命令说明

#### `init` 子命令

创建 `.c2rust` 目录并初始化 Git 仓库。如果目录已存在，将报错退出。

```bash
# 在当前目录创建 .c2rust 目录
c2rust-init init
```

**输出示例：**

成功创建（Unix/Linux/macOS）：
```
已创建目录: .c2rust
已在 .c2rust 目录初始化 Git 仓库
c2rust 项目已初始化，项目根目录为：/path/to/current/directory
若要在当前 shell 会话中使用该环境变量，请运行：
    export C2RUST_PROJECT_ROOT='/path/to/current/directory'
```

成功创建（Windows）：
```
已创建目录: .c2rust
已在 .c2rust 目录初始化 Git 仓库
c2rust 项目已初始化，项目根目录为：C:\path\to\current\directory
若要在当前 shell 会话中使用该环境变量，请根据所用 shell 运行：
  在 cmd.exe 中：
    set C2RUST_PROJECT_ROOT=C:\path\to\current\directory
  在 PowerShell 中：
    $env:C2RUST_PROJECT_ROOT = "C:\path\to\current\directory"
```

**注意**：`c2rust-init` 进程无法直接为您的 shell 设置环境变量。您需要按照上述提示手动在当前 shell 中导出 `C2RUST_PROJECT_ROOT` 环境变量，或将其添加到 shell 配置文件（Unix/Linux/macOS: `~/.bashrc` 或 `~/.zshrc`；Windows: 系统环境变量设置）以便永久使用。

目录已存在（错误）：
```
错误: 目录 '.c2rust' 已存在
```

路径已存在但不是目录（错误）：
```
错误: 路径 '.c2rust' 已存在且不是目录
```

其他错误情况：
```
创建目录 '.c2rust' 失败: Permission denied (os error 13)
```

### 查看帮助

```bash
# 查看主帮助
c2rust-init --help

# 查看 init 命令的帮助
c2rust-init init --help
```

## 功能说明

- ✅ **命令行参数解析**：使用 clap 库实现的现代化 CLI 接口
- ✅ **init 子命令**：初始化 `.c2rust` 目录结构
- ✅ **优雅的错误处理**：
  - 自动检测目录是否已存在
  - 提供清晰的错误信息
  - 适当的退出码（成功：0，失败：1）
- ✅ **友好的用户体验**：中文输出信息，易于理解

## 开发信息

### 项目结构

```
c2rust-init/
├── src/
│   └── main.rs          # 主程序入口和 CLI 实现
├── tests/
│   ├── cli_args.rs      # CLI 参数测试
│   └── create_c2rust_dir.rs  # 目录创建功能测试
├── Cargo.toml           # 项目配置和依赖
└── README.md            # 项目文档
```

### 构建项目

```bash
# 开发构建
cargo build

# 发布构建（优化）
cargo build --release
```

### 运行测试

```bash
# 运行所有测试
cargo test

# 运行特定测试
cargo test test_create_c2rust_dir_success

# 显示测试输出
cargo test -- --nocapture
```

### 代码检查

```bash
# 运行 clippy 进行代码检查
cargo clippy

# 格式化代码
cargo fmt
```

### 依赖项

- **运行时依赖**：
  - `clap` (v4) - 命令行参数解析，使用 derive 特性

- **开发依赖**：
  - `tempfile` (v3) - 用于测试中创建临时目录

### 贡献指南

欢迎提交 Issue 和 Pull Request！

在提交代码前，请确保：
1. 所有测试通过：`cargo test`
2. 代码格式正确：`cargo fmt`
3. 没有 clippy 警告：`cargo clippy`

## 许可证

本项目当前在仓库中未附带单独的 LICENSE 文件；在将本项目用于生产环境或进行再分发之前，请先与维护者确认具体的许可条款。

## 相关链接

- [c2rust 项目](https://c2rust.com/)
- [Rust 官方网站](https://www.rust-lang.org/)
- [clap 文档](https://docs.rs/clap/)
