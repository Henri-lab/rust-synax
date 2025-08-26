// 测试特性：错误处理(Error Handling)、Result类型、?操作符、panic!
// 语法要点：Result<T, E>、Ok()/Err()、?操作符、unwrap()、expect()
// 功能：演示Rust的错误处理机制和最佳实践

use std::fs::File;
use std::io::{self, Read};

// 自定义错误类型
#[derive(Debug)]
enum CalculatorError {
    DivisionByZero,
    InvalidInput,
}

// 可能出错的除法函数
fn divide(a: f64, b: f64) -> Result<f64, CalculatorError> {
    if b == 0.0 {
        Err(CalculatorError::DivisionByZero)
    } else {
        Ok(a / b)
    }
}

// 解析和计算函数
fn parse_and_divide(input: &str) -> Result<f64, CalculatorError> {
    let numbers: Vec<&str> = input.split('/').collect();
    
    if numbers.len() != 2 {
        return Err(CalculatorError::InvalidInput);
    }
    
    let a: f64 = numbers[0].trim().parse().map_err(|_| CalculatorError::InvalidInput)?;
    let b: f64 = numbers[1].trim().parse().map_err(|_| CalculatorError::InvalidInput)?;
    
    divide(a, b)
}

// 读取文件内容 (可能失败的操作)
fn read_file_content(filename: &str) -> Result<String, io::Error> {
    let mut file = File::open(filename)?;  // ?操作符传播错误
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

pub fn main() {
    println!("=== 错误处理演示 ===");
    
    // 1. 基本的Result处理
    match divide(10.0, 2.0) {
        Ok(result) => println!("10 ÷ 2 = {}", result),
        Err(e) => println!("计算错误: {:?}", e),
    }
    
    match divide(10.0, 0.0) {
        Ok(result) => println!("10 ÷ 0 = {}", result),
        Err(e) => println!("计算错误: {:?}", e),
    }
    
    // 2. 解析输入并处理错误
    let inputs = vec!["20/4", "15/3", "10/0", "invalid"];
    
    for input in inputs {
        match parse_and_divide(input) {
            Ok(result) => println!("'{}' 的结果是: {}", input, result),
            Err(e) => println!("处理 '{}' 时出错: {:?}", input, e),
        }
    }
    
    // 3. unwrap 和 expect 示例 (谨慎使用!)
    let good_result = divide(8.0, 2.0);
    let value = good_result.unwrap(); // 如果是Err会panic
    println!("使用unwrap: {}", value);
    
    let another_result = divide(12.0, 3.0);
    let value2 = another_result.expect("这不应该失败"); // 自定义panic消息
    println!("使用expect: {}", value2);
    
    // 4. 文件操作错误处理
    match read_file_content("non_existent_file.txt") {
        Ok(content) => println!("文件内容: {}", content),
        Err(e) => println!("读取文件失败: {}", e),
    }
    
    println!("\n错误处理最佳实践:");
    println!("- 使用Result<T, E>而不是panic!");
    println!("- 使用?操作符传播错误");
    println!("- 使用match明确处理每种情况");
    println!("- 谨慎使用unwrap()和expect()");
}