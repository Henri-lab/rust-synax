// 测试特性：字符串处理(String)、&str vs String、字符串方法、UTF-8
// 语法要点：String::new()、&str、push_str()、format!、字符串切片
// 功能：演示Rust中字符串的创建、操作和处理方法

pub fn main() {
    println!("=== 字符串处理演示 ===");
    
    // 1. 创建字符串的不同方法
    let s1 = String::new(); // 空字符串
    let s2 = String::from("Hello"); // 从字面量创建
    let s3 = "World".to_string(); // 转换为String
    let s4 = "Rust".to_owned(); // 获得所有权
    
    println!("s1 (空): '{}'", s1);
    println!("s2: '{}'", s2);
    println!("s3: '{}'", s3);
    println!("s4: '{}'", s4);
    
    // 2. 字符串拼接
    let mut greeting = String::from("Hello");
    greeting.push_str(", ");
    greeting.push('世');
    greeting.push_str("界!");
    println!("拼接结果: '{}'", greeting);
    
    // 使用format!宏
    let name = "Alice";
    let age = 30;
    let info = format!("姓名: {}, 年龄: {}", name, age);
    println!("格式化字符串: {}", info);
    
    // 3. 字符串切片
    let text = "Hello, 世界!";
    let hello = &text[0..5];
    println!("切片 [0..5]: '{}'", hello);
    
    // 注意: 中文字符占用多个字节
    // let world = &text[7..10]; // 这会panic，因为切到了字符中间
    
    // 4. 字符串方法演示
    let sample = "  Rust Programming  ";
    println!("\n字符串方法演示:");
    println!("原始: '{}'", sample);
    println!("长度: {}", sample.len());
    println!("去空格: '{}'", sample.trim());
    println!("转大写: '{}'", sample.to_uppercase());
    println!("转小写: '{}'", sample.to_lowercase());
    
    // 5. 字符串搜索和替换
    let sentence = "Rust is fast and Rust is safe";
    println!("\n搜索和替换:");
    println!("原句: {}", sentence);
    println!("包含'fast': {}", sentence.contains("fast"));
    println!("以'Rust'开头: {}", sentence.starts_with("Rust"));
    println!("以'safe'结尾: {}", sentence.ends_with("safe"));
    
    let replaced = sentence.replace("Rust", "JavaScript");
    println!("替换后: {}", replaced);
    
    // 6. 字符串分割
    let data = "apple,banana,cherry,date";
    let fruits: Vec<&str> = data.split(',').collect();
    println!("\n分割字符串:");
    println!("原始数据: {}", data);
    println!("分割结果: {:?}", fruits);
    
    // 7. 遍历字符串
    let text = "Hello世界";
    println!("\n字符遍历:");
    for (i, c) in text.chars().enumerate() {
        println!("  字符 {}: '{}'", i, c);
    }
    
    println!("\n字节遍历:");
    for (i, b) in text.bytes().enumerate() {
        println!("  字节 {}: {}", i, b);
    }
    
    // 8. 字符串解析
    let numbers = "42 3.14 100";
    let parsed: Vec<Result<f64, _>> = numbers
        .split_whitespace()
        .map(|s| s.parse::<f64>())
        .collect();
    
    println!("\n解析数字:");
    for (i, result) in parsed.iter().enumerate() {
        match result {
            Ok(num) => println!("  数字 {}: {}", i, num),
            Err(_) => println!("  解析失败"),
        }
    }
    
    println!("\n字符串总结:");
    println!("- String: 可变的、拥有所有权的字符串");
    println!("- &str: 字符串切片，通常是借用的");
    println!("- Rust字符串是UTF-8编码的");
    println!("- 小心处理多字节字符的索引");
}