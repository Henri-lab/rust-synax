// 测试特性：文件I/O操作、路径处理、错误处理、标准库I/O
// 语法要点：std::fs、std::io、Path、PathBuf、Read/Write trait
// 功能：演示Rust中的文件读写、路径操作和I/O错误处理

use std::fs::{self, File, OpenOptions};
use std::io::{self, Read, Write, BufRead, BufReader, BufWriter};
use std::path::{Path, PathBuf};
use std::env;

fn create_sample_files() -> io::Result<()> {
    // 创建示例文件和目录
    fs::create_dir_all("temp_demo")?;
    
    // 创建一个示例文本文件
    let mut file1 = File::create("temp_demo/sample.txt")?;
    file1.write_all(b"Hello, Rust!\nThis is a sample file.\nLine 3 content.")?;
    
    // 创建另一个文件
    let mut file2 = File::create("temp_demo/numbers.txt")?;
    for i in 1..=10 {
        writeln!(file2, "Number: {}", i)?;
    }
    
    // 创建JSON样式的文件
    let mut json_file = File::create("temp_demo/data.json")?;
    json_file.write_all(br#"{"name": "Alice", "age": 30, "city": "Beijing"}"#)?;
    
    Ok(())
}

fn cleanup_demo_files() -> io::Result<()> {
    // 清理演示文件
    if Path::new("temp_demo").exists() {
        fs::remove_dir_all("temp_demo")?;
    }
    Ok(())
}

fn demonstrate_basic_file_ops() -> io::Result<()> {
    println!("1. 基本文件操作:");
    
    // 读取整个文件内容
    let content = fs::read_to_string("temp_demo/sample.txt")?;
    println!("  文件内容:");
    println!("  {}", content);
    
    // 读取文件为字节
    let bytes = fs::read("temp_demo/sample.txt")?;
    println!("  文件大小: {} 字节", bytes.len());
    
    // 写入新文件
    fs::write("temp_demo/output.txt", "这是新创建的文件内容\n你好，世界！")?;
    println!("  已创建新文件: output.txt");
    
    Ok(())
}

fn demonstrate_buffered_io() -> io::Result<()> {
    println!("\n2. 缓冲I/O操作:");
    
    // 使用BufReader逐行读取
    let file = File::open("temp_demo/numbers.txt")?;
    let reader = BufReader::new(file);
    
    println!("  逐行读取:");
    for (line_num, line) in reader.lines().enumerate() {
        let line = line?;
        if line_num < 3 {  // 只显示前3行
            println!("    行 {}: {}", line_num + 1, line);
        }
    }
    
    // 使用BufWriter写入
    let file = File::create("temp_demo/buffered_output.txt")?;
    let mut writer = BufWriter::new(file);
    
    for i in 1..=5 {
        writeln!(writer, "缓冲写入行 {}", i)?;
    }
    writer.flush()?;  // 确保数据写入磁盘
    println!("  已使用缓冲写入创建文件");
    
    Ok(())
}

fn demonstrate_file_attributes() -> io::Result<()> {
    println!("\n3. 文件属性和元数据:");
    
    let metadata = fs::metadata("temp_demo/sample.txt")?;
    println!("  文件长度: {} 字节", metadata.len());
    println!("  是否为文件: {}", metadata.is_file());
    println!("  是否为目录: {}", metadata.is_dir());
    println!("  是否只读: {}", metadata.permissions().readonly());
    
    if let Ok(modified) = metadata.modified() {
        println!("  修改时间: {:?}", modified);
    }
    
    Ok(())
}

fn demonstrate_path_operations() {
    println!("\n4. 路径操作:");
    
    // 创建路径
    let path = Path::new("temp_demo/sample.txt");
    println!("  路径: {:?}", path);
    println!("  父目录: {:?}", path.parent());
    println!("  文件名: {:?}", path.file_name());
    println!("  扩展名: {:?}", path.extension());
    println!("  文件茎: {:?}", path.file_stem());
    
    // PathBuf可变路径
    let mut path_buf = PathBuf::from("temp_demo");
    path_buf.push("new_file.txt");
    println!("  构建的路径: {:?}", path_buf);
    
    // 路径连接
    let joined = Path::new("temp_demo").join("subdir").join("file.txt");
    println!("  连接路径: {:?}", joined);
    
    // 当前目录
    if let Ok(current_dir) = env::current_dir() {
        println!("  当前目录: {:?}", current_dir);
    }
}

fn demonstrate_directory_operations() -> io::Result<()> {
    println!("\n5. 目录操作:");
    
    // 创建目录
    fs::create_dir_all("temp_demo/subdir/nested")?;
    println!("  已创建嵌套目录");
    
    // 列出目录内容
    println!("  目录内容:");
    for entry in fs::read_dir("temp_demo")? {
        let entry = entry?;
        let path = entry.path();
        let file_type = if path.is_dir() { "目录" } else { "文件" };
        println!("    {} - {:?}", file_type, entry.file_name());
    }
    
    Ok(())
}

fn demonstrate_file_options() -> io::Result<()> {
    println!("\n6. 文件打开选项:");
    
    // 追加模式
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("temp_demo/append_test.txt")?;
    
    writeln!(file, "追加的第一行")?;
    writeln!(file, "追加的第二行")?;
    
    // 再次追加
    let mut file = OpenOptions::new()
        .append(true)
        .open("temp_demo/append_test.txt")?;
    writeln!(file, "又追加了一行")?;
    
    // 读取结果
    let content = fs::read_to_string("temp_demo/append_test.txt")?;
    println!("  追加文件内容:\n{}", content);
    
    Ok(())
}

fn demonstrate_error_handling() {
    println!("\n7. I/O错误处理:");
    
    // 尝试读取不存在的文件
    match fs::read_to_string("non_existent_file.txt") {
        Ok(content) => println!("  文件内容: {}", content),
        Err(e) => println!("  读取文件失败: {} (错误类型: {:?})", e, e.kind()),
    }
    
    // 使用?操作符的函数示例
    fn try_read_file() -> Result<String, io::Error> {
        let content = fs::read_to_string("temp_demo/sample.txt")?;
        Ok(content.to_uppercase())
    }
    
    match try_read_file() {
        Ok(content) => println!("  成功读取并转换: {}", content.lines().next().unwrap_or("")),
        Err(e) => println!("  操作失败: {}", e),
    }
}

fn demonstrate_copy_operations() -> io::Result<()> {
    println!("\n8. 文件复制和移动:");
    
    // 复制文件
    fs::copy("temp_demo/sample.txt", "temp_demo/sample_copy.txt")?;
    println!("  已复制文件");
    
    // 重命名/移动文件
    fs::rename("temp_demo/sample_copy.txt", "temp_demo/renamed_file.txt")?;
    println!("  已重命名文件");
    
    Ok(())
}

pub fn main() {
    println!("=== 文件I/O操作演示 ===");
    
    // 创建演示文件
    if let Err(e) = create_sample_files() {
        println!("创建示例文件失败: {}", e);
        return;
    }
    
    // 各种演示
    let demos: &[(&str, fn() -> std::io::Result<()>)] = &[
        ("基本文件操作", demonstrate_basic_file_ops as fn() -> std::io::Result<()>),
        ("缓冲I/O", demonstrate_buffered_io as fn() -> std::io::Result<()>),  
        ("文件属性", demonstrate_file_attributes as fn() -> std::io::Result<()>),
        ("目录操作", demonstrate_directory_operations as fn() -> std::io::Result<()>),
        ("文件选项", demonstrate_file_options as fn() -> std::io::Result<()>),
        ("复制操作", demonstrate_copy_operations as fn() -> std::io::Result<()>),
    ];
    
    for (name, demo_fn) in demos {
        if let Err(e) = demo_fn() {
            println!("{}演示失败: {}", name, e);
        }
    }
    
    // 路径操作（不会失败）
    demonstrate_path_operations();
    
    // 错误处理演示
    demonstrate_error_handling();
    
    println!("\n文件I/O总结:");
    println!("- std::fs: 文件系统操作模块");
    println!("- std::io: 输入输出trait和类型");
    println!("- Path/PathBuf: 路径类型和操作");
    println!("- BufReader/BufWriter: 缓冲I/O");
    println!("- OpenOptions: 灵活的文件打开选项");
    println!("- 错误处理: 所有I/O操作都返回Result");
    
    // 清理演示文件
    if let Err(e) = cleanup_demo_files() {
        println!("\n清理文件失败: {}", e);
    } else {
        println!("\n演示文件已清理");
    }
}