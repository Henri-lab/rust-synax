// 测试特性：模式匹配(Pattern Matching)、Option类型、Match表达式
// 语法要点：match关键字、Some/None、if let、模式解构
// 功能：演示Rust中强大的模式匹配功能

pub fn main() {
    println!("=== 模式匹配演示 ===");
    
    // Option类型的模式匹配
    let some_number = Some(42);
    let no_number: Option<i32> = None;
    
    // match表达式
    match some_number {
        Some(n) => println!("找到数字: {}", n),
        None => println!("没有数字"),
    }
    
    match no_number {
        Some(n) => println!("找到数字: {}", n),
        None => println!("确实没有数字"),
    }
    
    // if let语法糖
    if let Some(value) = some_number {
        println!("使用if let获取值: {}", value);
    }
    
    // 匹配不同类型的值
    let message = "hello";
    match message {
        "hello" => println!("你好!"),
        "goodbye" => println!("再见!"),
        _ => println!("其他消息: {}", message),
    }
    
    // 元组的模式匹配
    let point = (3, 5);
    match point {
        (0, 0) => println!("原点"),
        (0, y) => println!("在Y轴上: y = {}", y),
        (x, 0) => println!("在X轴上: x = {}", x),
        (x, y) => println!("点的坐标: ({}, {})", x, y),
    }
}