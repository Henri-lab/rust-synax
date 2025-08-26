// 测试特性：枚举(Enums)、方法实现、数据携带、模式匹配
// 语法要点：enum关键字、数据变体、impl for enum、match with data
// 功能：演示枚举的不同用法和数据携带能力

#[derive(Debug)]
enum Message {
    Quit,                       // 无数据变体
    Move { x: i32, y: i32 },    // 命名字段变体
    Write(String),              // 元组变体
    ChangeColor(i32, i32, i32), // 多元素元组变体
}

impl Message {
    fn process(&self) {
        match self {
            Message::Quit => {
                println!("收到退出消息");
            },
            Message::Move { x, y } => {
                println!("移动到坐标 ({}, {})", x, y);
            },
            Message::Write(text) => {
                println!("写入文本: {}", text);
            },
            Message::ChangeColor(r, g, b) => {
                println!("改变颜色到 RGB({}, {}, {})", r, g, b);
            },
        }
    }
}

#[derive(Debug)]
enum WebEvent {
    PageLoad,
    PageUnload,
    KeyPress(char),
    Paste(String),
    Click { x: i64, y: i64 },
}

pub fn main() {
    println!("=== 枚举演示 ===");
    
    // 创建不同类型的消息
    let messages = vec![
        Message::Write(String::from("Hello, Rust!")),
        Message::Move { x: 10, y: 30 },
        Message::ChangeColor(255, 0, 0),
        Message::Quit,
    ];
    
    // 处理每个消息
    for msg in &messages {
        println!("处理消息: {:?}", msg);
        msg.process();
        println!();
    }
    
    // 另一个枚举示例
    let events = vec![
        WebEvent::PageLoad,
        WebEvent::KeyPress('x'),
        WebEvent::Paste(String::from("clipboard content")),
        WebEvent::Click { x: 320, y: 240 },
    ];
    
    println!("处理Web事件:");
    for event in events {
        match event {
            WebEvent::PageLoad => println!("页面加载"),
            WebEvent::PageUnload => println!("页面卸载"),
            WebEvent::KeyPress(c) => println!("按键: '{}'", c),
            WebEvent::Paste(s) => println!("粘贴: {}", s),
            WebEvent::Click { x, y } => println!("点击位置: ({}, {})", x, y),
        }
    }
}