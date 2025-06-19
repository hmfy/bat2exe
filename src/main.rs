use std::fs;
use std::path::PathBuf;
use clap::{Arg, Command as ClapCommand};
use anyhow::{Result, Context};

mod encoding_detector;
mod bat_converter;
mod exe_generator;

use encoding_detector::EncodingDetector;
use bat_converter::BatConverter;
use exe_generator::ExeGenerator;

fn main() -> Result<()> {
    let matches = ClapCommand::new("bat2exe")
        .version("1.0.0")
        .author("BAT2EXE Converter")
        .about("将BAT批处理文件转换为EXE可执行文件")
        .arg(
            Arg::new("input")
                .help("输入的BAT文件路径")
                .required(true)
                .index(1)
        )
        .arg(
            Arg::new("output")
                .help("输出的EXE文件路径")
                .short('o')
                .long("output")
        )
        .arg(
            Arg::new("icon")
                .help("自定义图标文件路径")
                .short('i')
                .long("icon")
        )
        .arg(
            Arg::new("verbose")
                .help("显示详细信息")
                .short('v')
                .long("verbose")
                .action(clap::ArgAction::SetTrue)
        )
        .get_matches();

    let input_path = matches.get_one::<String>("input").unwrap();
    let verbose = matches.get_flag("verbose");
    
    let output_path = match matches.get_one::<String>("output") {
        Some(path) => PathBuf::from(path),
        None => {
            let input_path_buf = PathBuf::from(input_path);
            let mut output = input_path_buf.clone();
            output.set_extension("exe");
            output
        }
    };

    let icon_path = matches.get_one::<String>("icon").map(PathBuf::from);

    if verbose {
        println!("输入文件: {}", input_path);
        println!("输出文件: {}", output_path.display());
        if let Some(ref icon) = icon_path {
            println!("图标文件: {}", icon.display());
        }
    }

    // 步骤1: 检测BAT文件编码
    if verbose {
        println!("正在检测文件编码...");
    }
    
    let detector = EncodingDetector::new();
    let encoding_info = detector.detect_file_encoding(input_path)
        .context("无法检测文件编码")?;
    
    if verbose {
        println!("检测到编码: {}", encoding_info.encoding);
        println!("置信度: {:.2}%", encoding_info.confidence * 100.0);
        println!("系统编码: {}", encoding_info.system_encoding);
    }

    // 步骤2: 转换BAT文件编码（如果需要）
    let converter = BatConverter::new();
    let processed_bat_path = converter.process_bat_file(input_path, &encoding_info, verbose)
        .context("处理BAT文件失败")?;

    if verbose {
        println!("BAT文件处理完成: {}", processed_bat_path.display());
    }

    // 步骤3: 生成EXE文件
    if verbose {
        println!("正在生成EXE文件...");
    }
    
    let generator = ExeGenerator::new();
    generator.generate_exe(&processed_bat_path, &output_path, icon_path.as_ref(), verbose)
        .context("生成EXE文件失败")?;

    // 清理临时文件
    if processed_bat_path != PathBuf::from(input_path) {
        if verbose {
            println!("清理临时文件: {}", processed_bat_path.display());
        }
        let _ = fs::remove_file(&processed_bat_path);
    }

    println!("转换成功！");
    println!("输出文件: {}", output_path.display());

    Ok(())
}
