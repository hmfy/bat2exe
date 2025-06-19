# BAT2EXE - BAT 文件转 EXE 工具

一个功能强大的命令行工具，能够将任意编码格式的 BAT 批处理文件转换为独立的 EXE 可执行文件，并自动处理字符编码问题。

## 功能特性

- ✅ **多编码支持**: 自动检测和处理 UTF-8、GBK、Big5 等多种编码格式
- ✅ **智能编码转换**: 根据系统环境自动转换编码，确保中文字符正确显示
- ✅ **零依赖**: 生成的 EXE 文件无需额外依赖，可独立运行
- ✅ **参数传递**: 完整支持命令行参数传递
- ✅ **图标支持**: 可自定义 EXE 文件图标
- ✅ **原生性能**: 使用 Rust 开发，生成的 EXE 文件体积小、运行快

## 安装使用

### 从源码构建

```bash
git clone <repository-url>
cd bat2exe
cargo build --release
```

生成的可执行文件位于 `target/release/bat2exe.exe`

### 使用方法

```bash
# 基本用法
bat2exe input.bat

# 指定输出文件名
bat2exe input.bat -o output.exe

# 添加自定义图标
bat2exe input.bat -i icon.ico

# 显示详细信息
bat2exe input.bat -v

# 查看帮助
bat2exe --help
```

## 命令行选项

- `<input>`: 输入的BAT文件路径（必需）
- `-o, --output <output>`: 输出的EXE文件路径（可选，默认与输入文件同名）
- `-i, --icon <icon>`: 自定义图标文件路径（可选）
- `-v, --verbose`: 显示详细信息
- `-h, --help`: 显示帮助信息
- `-V, --version`: 显示版本信息

## 编码处理

本工具特别针对 Windows 中文环境进行了优化：

1. **自动编码检测**: 使用先进的编码检测算法识别源文件编码
2. **智能编码转换**: 
   - 如果源文件编码与系统编码一致，则无需转换
   - 如果不一致，自动转换为系统默认编码（通常为 GBK）
3. **字符正确显示**: 确保生成的 EXE 文件在 cmd.exe 和 PowerShell 中都能正确显示中文

## 使用示例

### 示例 1: 转换 UTF-8 编码的 BAT 文件

假设你有一个 UTF-8 编码的 `hello.bat` 文件：

```batch
@echo off
echo "你好，世界！"
echo "当前时间: %date% %time%"
pause
```

使用以下命令转换：

```bash
bat2exe hello.bat -v
```

程序将自动检测到 UTF-8 编码，转换为 GBK 编码，并生成 `hello.exe`。

### 示例 2: 转换 GBK 编码的 BAT 文件

如果你的 BAT 文件已经是 GBK 编码，程序会检测到编码匹配，直接生成 EXE：

```bash
bat2exe gbk_file.bat -o my_program.exe
```

### 示例 3: 添加自定义图标

```bash
bat2exe script.bat -i my_icon.ico -o my_program.exe
```

## 技术实现

- **语言**: Rust
- **编码检测**: chardet + 启发式算法
- **编码转换**: encoding_rs
- **EXE 生成**: 动态创建 Rust 项目并编译
- **资源嵌入**: winres（用于图标和版本信息）

## 测试结果

本工具已经过以下场景测试：

✅ UTF-8 编码的 BAT 文件转换  
✅ GBK 编码的 BAT 文件转换  
✅ 包含中文字符的复杂脚本  
✅ 命令行参数传递  
✅ 环境变量使用  
✅ 错误代码传递  

## 注意事项

1. **系统要求**: Windows 系统，需要安装 Rust 编译环境
2. **编码限制**: 主要针对 UTF-8 和 GBK 编码优化，其他编码可能需要手动处理
3. **文件大小**: 生成的 EXE 文件大小约为 200KB-300KB（已优化）
4. **权限要求**: 生成的 EXE 文件继承原 BAT 文件的权限要求

## 贡献

欢迎提交 Issue 和 Pull Request！

---

**作者**: hmfy  
**版本**: 1.0.0  
**更新日期**: 2025年6月18日
