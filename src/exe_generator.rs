use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use tempfile;
use anyhow::{Result, Context};

pub struct ExeGenerator;

impl ExeGenerator {
    pub fn new() -> Self {
        Self
    }

    pub fn generate_exe(
        &self,
        bat_path: &Path,
        output_path: &Path,
        icon_path: Option<&PathBuf>,
        verbose: bool,
    ) -> Result<()> {
        if verbose {
            println!("开始生成EXE文件...");
        }

        // 创建临时的 Rust 项目来生成 EXE
        let temp_dir = tempfile::tempdir()
            .context("无法创建临时目录")?;
        
        let temp_project_path = temp_dir.path().join("bat_runner");
        fs::create_dir_all(&temp_project_path)
            .context("无法创建临时项目目录")?;

        if verbose {
            println!("临时项目路径: {}", temp_project_path.display());
        }

        // 生成 Cargo.toml
        self.create_cargo_toml(&temp_project_path, verbose)?;

        // 创建 src 目录
        let src_dir = temp_project_path.join("src");
        fs::create_dir_all(&src_dir)
            .context("无法创建 src 目录")?;

        // 生成 main.rs
        self.create_main_rs(&src_dir, bat_path, verbose)?;

        // 处理图标文件
        if let Some(icon) = icon_path {
            self.setup_icon(&temp_project_path, icon, verbose)?;
        }

        // 编译项目
        self.compile_project(&temp_project_path, verbose)?;

        // 复制生成的 EXE 到目标位置
        let exe_source = temp_project_path.join("target").join("release").join("bat_runner.exe");
        if exe_source.exists() {
            fs::copy(&exe_source, output_path)
                .context("无法复制生成的EXE文件")?;
            
            if verbose {
                println!("EXE文件已复制到: {}", output_path.display());
            }
        } else {
            return Err(anyhow::anyhow!("编译生成的EXE文件不存在"));
        }

        Ok(())
    }

    fn create_cargo_toml(&self, project_path: &Path, _verbose: bool) -> Result<()> {
        let cargo_toml_content = r#"[package]
name = "bat_runner"
version = "0.1.0"
edition = "2021"

[profile.release]
opt-level = "z"
lto = true
codegen-units = 1
panic = "abort"
strip = true

[[bin]]
name = "bat_runner"
path = "src/main.rs"
"#;

        let cargo_toml_path = project_path.join("Cargo.toml");
        fs::write(cargo_toml_path, cargo_toml_content)
            .context("无法创建 Cargo.toml")?;

        Ok(())
    }    fn create_main_rs(&self, src_dir: &Path, bat_path: &Path, verbose: bool) -> Result<()> {
        // 直接读取 BAT 文件的原始字节
        let raw_content = fs::read(bat_path)
            .context("无法读取BAT文件")?;
        
        // 将原始字节转换为十六进制字符串，这样可以避免编码问题
        let hex_content = raw_content.iter()
            .map(|b| format!("{:02x}", b))
            .collect::<Vec<_>>()
            .join("");

        let main_rs_content = format!(r###"#![windows_subsystem = "console"]

use std::fs;
use std::path::PathBuf;
use std::process::Command;
use std::env;

const BAT_CONTENT_HEX: &str = "{}";

fn main() -> Result<(), Box<dyn std::error::Error>> {{
    // 设置控制台编码为系统默认编码页 (通过 chcp 命令)
    let _ = Command::new("cmd").args(&["/c", "chcp", "936"]).output();

    // 将十六进制字符串转换回字节
    let mut bat_content = Vec::new();
    let chars: Vec<char> = BAT_CONTENT_HEX.chars().collect();
    for chunk in chars.chunks(2) {{
        if chunk.len() == 2 {{
            let hex_str: String = chunk.iter().collect();
            if let Ok(byte_val) = u8::from_str_radix(&hex_str, 16) {{
                bat_content.push(byte_val);
            }}
        }}
    }}

    // 获取临时目录
    let temp_dir = env::temp_dir();
    let bat_file_path = temp_dir.join("bat2exe_temp.bat");

    // 将字节内容写入临时文件
    fs::write(&bat_file_path, &bat_content)?;

    // 执行 BAT 文件
    let mut cmd = Command::new("cmd");
    cmd.arg("/C");
    cmd.arg(&bat_file_path);

    // 传递命令行参数
    let args: Vec<String> = env::args().skip(1).collect();
    for arg in args {{
        cmd.arg(arg);
    }}

    let status = cmd.status()?;

    // 清理临时文件
    let _ = fs::remove_file(&bat_file_path);

    // 返回与原始 BAT 文件相同的退出代码
    if !status.success() {{
        if let Some(code) = status.code() {{
            std::process::exit(code);
        }} else {{
            std::process::exit(1);
        }}
    }}

    Ok(())
}}
"###, hex_content);

        let main_rs_path = src_dir.join("main.rs");
        
        if verbose {
            println!("生成的 main.rs 大小: {} 字节", main_rs_content.len());
        }
        
        fs::write(main_rs_path, main_rs_content)
            .context("无法创建 main.rs")?;

        Ok(())
    }

    fn setup_icon(&self, project_path: &Path, icon_path: &Path, verbose: bool) -> Result<()> {
        if !icon_path.exists() {
            return Err(anyhow::anyhow!("图标文件不存在: {}", icon_path.display()));
        }

        // 复制图标文件到项目目录
        let target_icon_path = project_path.join("icon.ico");
        fs::copy(icon_path, &target_icon_path)
            .context("无法复制图标文件")?;

        // 创建 build.rs 文件
        let build_rs_content = r#"extern crate winres;

fn main() {
    if cfg!(target_os = "windows") {
        let mut res = winres::WindowsResource::new();
        res.set_icon("icon.ico");
        res.set_language(0x0804); // 简体中文
        res.set("FileDescription", "BAT Script Runner")
           .set("ProductName", "BAT to EXE Converter")
           .set("FileVersion", "1.0.0")
           .set("ProductVersion", "1.0.0")
           .set("LegalCopyright", "Copyright © 2025");
           
        if let Err(e) = res.compile() {
            eprintln!("Warning: {}", e);
        }
    }
}
"#;

        let build_rs_path = project_path.join("build.rs");
        fs::write(build_rs_path, build_rs_content)
            .context("无法创建 build.rs")?;

        // 更新 Cargo.toml 以包含 build dependencies
        let cargo_toml_path = project_path.join("Cargo.toml");
        let mut cargo_content = fs::read_to_string(&cargo_toml_path)?;
        cargo_content.push_str("\n[build-dependencies]\nwinres = \"0.1\"\n");
        fs::write(cargo_toml_path, cargo_content)
            .context("无法更新 Cargo.toml")?;

        if verbose {
            println!("图标设置完成: {}", target_icon_path.display());
        }

        Ok(())
    }

    fn compile_project(&self, project_path: &Path, verbose: bool) -> Result<()> {
        if verbose {
            println!("开始编译项目...");
        }

        let mut cmd = Command::new("cargo");
        cmd.arg("build")
           .arg("--release")
           .current_dir(project_path);

        if !verbose {
            cmd.arg("--quiet");
        }

        let output = cmd.output()
            .context("无法执行 cargo build 命令")?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(anyhow::anyhow!("编译失败:\n{}", stderr));
        }

        if verbose {
            println!("编译完成！");
            if !output.stdout.is_empty() {
                println!("编译输出:\n{}", String::from_utf8_lossy(&output.stdout));
            }
        }

        Ok(())
    }
}
