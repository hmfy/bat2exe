use std::fs;
use chardet::{detect, charset2encoding};
use encoding_rs::Encoding;
use anyhow::{Result, Context};

#[derive(Debug, Clone)]
pub struct EncodingInfo {
    pub encoding: String,
    pub confidence: f32,
    pub system_encoding: String,
    pub needs_conversion: bool,
}

pub struct EncodingDetector;

impl EncodingDetector {
    pub fn new() -> Self {
        Self
    }    pub fn detect_file_encoding(&self, file_path: &str) -> Result<EncodingInfo> {
        // 读取文件内容
        let content = fs::read(file_path)
            .context("无法读取文件")?;

        // 使用 chardet 检测编码
        let detection_result = detect(&content);
        let mut detected_encoding = charset2encoding(&detection_result.0);
        let mut confidence = detection_result.1;
        
        // 如果置信度太低，尝试其他方法
        if confidence < 0.7 {
            detected_encoding = self.heuristic_encoding_detection(&content);
            confidence = 0.8; // 给启发式检测一个合理的置信度
        }
        
        // 获取系统默认编码 (Windows 中文版默认为 GBK)
        let system_encoding = self.get_system_encoding();
        
        // 判断是否需要转换
        let needs_conversion = !self.is_encoding_compatible(&detected_encoding, &system_encoding);
        
        Ok(EncodingInfo {
            encoding: detected_encoding.to_string(),
            confidence,
            system_encoding,
            needs_conversion,
        })
    }

    fn heuristic_encoding_detection(&self, content: &[u8]) -> &str {
        // 启发式编码检测
        
        // 检测 BOM
        if content.starts_with(&[0xEF, 0xBB, 0xBF]) {
            return "utf-8";
        }
        if content.starts_with(&[0xFF, 0xFE]) {
            return "utf-16le";
        }
        if content.starts_with(&[0xFE, 0xFF]) {
            return "utf-16be";
        }
        
        // 尝试按 UTF-8 解码
        if std::str::from_utf8(content).is_ok() {
            // 检查是否包含中文字符
            if let Ok(s) = std::str::from_utf8(content) {
                if s.chars().any(|c| '\u{4e00}' <= c && c <= '\u{9fff}') {
                    return "utf-8";
                }
            }
            return "utf-8";
        }
        
        // 检查是否可能是 GBK
        let (decoded, _, had_errors) = encoding_rs::GBK.decode(content);
        if !had_errors && decoded.chars().any(|c| '\u{4e00}' <= c && c <= '\u{9fff}') {
            return "gbk";
        }
        
        // 默认假设是 GBK (对于中文 Windows 系统)
        "gbk"
    }fn get_system_encoding(&self) -> String {
        // 在 Windows 中文环境下，cmd.exe 和 PowerShell 默认使用 GBK 编码
        // 这里可以通过 Windows API 获取系统代码页，但为了简化，我们假设是 GBK
        #[cfg(windows)]
        {
            use std::process::Command;
            
            // 通过 chcp 命令获取当前代码页
            if let Ok(output) = Command::new("cmd").args(&["/c", "chcp"]).output() {
                if let Ok(output_str) = String::from_utf8(output.stdout) {
                    if output_str.contains("936") {
                        return "gbk".to_string();
                    } else if output_str.contains("65001") {
                        return "utf-8".to_string();
                    } else if output_str.contains("950") {
                        return "big5".to_string();
                    }
                }
            }
            
            // 默认返回 GBK
            "gbk".to_string()
        }
        
        #[cfg(not(windows))]
        "utf-8".to_string()
    }

    fn is_encoding_compatible(&self, detected: &str, system: &str) -> bool {
        let detected_lower = detected.to_lowercase();
        let system_lower = system.to_lowercase();
        
        // 规范化编码名称
        let detected_normalized = self.normalize_encoding_name(&detected_lower);
        let system_normalized = self.normalize_encoding_name(&system_lower);
        
        detected_normalized == system_normalized
    }

    fn normalize_encoding_name(&self, encoding: &str) -> String {
        match encoding {
            "gb2312" | "gbk" | "gb18030" | "chinese" => "gbk".to_string(),
            "utf-8" | "utf8" => "utf-8".to_string(),
            "utf-16" | "utf16" | "utf-16le" | "utf-16be" => "utf-16".to_string(),
            "big5" | "big5-hkscs" => "big5".to_string(),
            "ascii" | "us-ascii" => "ascii".to_string(),
            _ => encoding.to_string(),
        }
    }

    pub fn get_target_encoding_for_conversion(&self, system_encoding: &str) -> &'static Encoding {
        match system_encoding.to_lowercase().as_str() {
            "gbk" | "gb2312" | "gb18030" => encoding_rs::GBK,
            "big5" => encoding_rs::BIG5,
            "utf-8" => encoding_rs::UTF_8,
            _ => encoding_rs::GBK, // 默认使用 GBK
        }
    }    pub fn get_source_encoding(&self, detected_encoding: &str) -> Option<&'static Encoding> {
        match detected_encoding.to_lowercase().as_str() {
            "utf-8" | "utf8" => Some(encoding_rs::UTF_8),
            "utf-16" | "utf16" | "utf-16le" => Some(encoding_rs::UTF_16LE),
            "utf-16be" => Some(encoding_rs::UTF_16BE),
            "gbk" | "gb2312" | "gb18030" | "chinese" => Some(encoding_rs::GBK),
            "big5" => Some(encoding_rs::BIG5),
            "ascii" | "us-ascii" => Some(encoding_rs::UTF_8), // ASCII 兼容 UTF-8
            "tis-620" | "windows-874" => Some(encoding_rs::GBK), // 泰文编码，但可能是误检测的 GBK
            _ => {
                // 如果检测失败，尝试按 GBK 处理
                Some(encoding_rs::GBK)
            },
        }
    }
}
