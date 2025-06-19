extern crate winres;

fn main() {
    if cfg!(target_os = "windows") {
        let mut res = winres::WindowsResource::new();
        // 设置应用程序图标
        res.set_icon("app.ico");
        res.set_language(0x0409); // 英语 US
        
        // 设置版本信息
        res.set("FileDescription", "BAT to EXE Converter")
           .set("ProductName", "BAT to EXE Converter")
           .set("FileVersion", "1.0.0")
           .set("ProductVersion", "1.0.0")
           .set("LegalCopyright", "Copyright © 2025");
           
        // 编译资源
        if let Err(e) = res.compile() {
            eprintln!("Error: {}", e);
            // 不要退出，让编译继续
            // std::process::exit(1);
        }
    }
}