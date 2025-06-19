use std::fs;
use std::path::PathBuf;
use tempfile::NamedTempFile;
use anyhow::{Result, Context};

use crate::encoding_detector::{EncodingInfo, EncodingDetector};

pub struct BatConverter;

impl BatConverter {
    pub fn new() -> Self {
        Self
    }

    pub fn process_bat_file(
        &self,
        input_path: &str,
        encoding_info: &EncodingInfo,
        verbose: bool,
    ) -> Result<PathBuf> {
        if !encoding_info.needs_conversion {
            if verbose {
                println!("文件编码与系统编码一致，无需转换");
            }
            return Ok(PathBuf::from(input_path));
        }

        if verbose {
            println!("需要转换编码: {} -> {}", encoding_info.encoding, encoding_info.system_encoding);
        }

        // 读取原始文件内容
        let raw_content = fs::read(input_path)
            .context("无法读取源文件")?;

        // 检测源编码
        let detector = EncodingDetector::new();
        let source_encoding = detector.get_source_encoding(&encoding_info.encoding)
            .context("不支持的源编码格式")?;

        // 获取目标编码
        let target_encoding = detector.get_target_encoding_for_conversion(&encoding_info.system_encoding);

        // 解码原始内容
        let (decoded_content, _, had_errors) = source_encoding.decode(&raw_content);
        if had_errors && verbose {
            println!("警告: 解码过程中发现错误，可能影响输出质量");
        }

        // 编码为目标编码
        let (encoded_content, _, had_errors) = target_encoding.encode(&decoded_content);
        if had_errors && verbose {
            println!("警告: 编码过程中发现错误，可能影响输出质量");
        }

        // 创建临时文件
        let temp_file = NamedTempFile::new()
            .context("无法创建临时文件")?;
        
        let temp_path = temp_file.path().with_extension("bat");
        
        // 写入转换后的内容
        fs::write(&temp_path, &encoded_content)
            .context("无法写入临时文件")?;

        if verbose {
            println!("编码转换完成: {} -> {}", source_encoding.name(), target_encoding.name());
            println!("临时文件: {}", temp_path.display());
        }

        // 防止临时文件被自动删除
        std::mem::forget(temp_file);        Ok(temp_path)
    }
}
