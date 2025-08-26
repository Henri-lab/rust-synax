// 测试特性：生命周期(Lifetimes)、借用检查、引用有效性、生命周期标注
// 语法要点：'a生命周期参数、&'a引用、函数生命周期、结构体生命周期
// 功能：演示Rust生命周期系统如何确保内存安全

// 生命周期最长的引用
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// 返回第一个参数的引用
fn first_word<'a>(x: &'a str, _y: &str) -> &'a str {
    x.split_whitespace().next().unwrap_or("")
}

// 包含引用的结构体
struct TextHolder<'a> {
    content: &'a str,
    author: &'a str,
}

impl<'a> TextHolder<'a> {
    fn new(content: &'a str, author: &'a str) -> Self {
        TextHolder { content, author }
    }
    
    fn get_info(&self) -> String {
        format!("作者: {}, 内容: {}", self.author, self.content)
    }
    
    // 方法的生命周期
    fn get_content(&self) -> &str {
        self.content
    }
}

// 多个生命周期参数
fn combine_texts<'a, 'b>(
    first: &'a str, 
    second: &'b str
) -> String {
    format!("{} {}", first, second)
}

// 生命周期省略规则示例
fn get_first_char(s: &str) -> Option<char> {  // 省略了生命周期
    s.chars().next()
}

// 静态生命周期
fn get_static_text() -> &'static str {
    "这是一个静态字符串"
}

// 包含引用的枚举
#[derive(Debug)]
enum Message<'a> {
    Text(&'a str),
    Number(i32),
}

pub fn main() {
    println!("=== 生命周期演示 ===");
    
    // 1. 基本的生命周期使用
    let string1 = String::from("Hello");
    let string2 = "World";
    
    let result = longest(&string1, string2);
    println!("最长的字符串: {}", result);
    
    // 2. 不同生命周期的例子
    {
        let text1 = "Rust";
        let text2 = "Programming";
        let first = first_word(text1, text2);
        println!("第一个单词: {}", first);
    } // text1 和 text2 在这里被销毁，但first已经不可用了
    
    // 3. 结构体中的生命周期
    let content = "Rust是一种系统编程语言";
    let author = "Mozilla";
    
    let holder = TextHolder::new(content, author);
    println!("文本信息: {}", holder.get_info());
    println!("获取内容: {}", holder.get_content());
    
    // 4. 生命周期和所有权的交互
    let text = "Hello, Rust!";
    let words: Vec<&str> = text.split_whitespace().collect();
    println!("分割的单词: {:?}", words);
    // words 中的引用依赖于 text 的生命周期
    
    // 5. 多个生命周期参数
    let part1 = "Hello";
    let part2 = "World";
    let combined = combine_texts(part1, part2);
    println!("组合文本: {}", combined);
    
    // 6. 静态生命周期
    let static_text = get_static_text();
    println!("静态文本: {}", static_text);
    
    // 7. 生命周期省略
    if let Some(first_char) = get_first_char("Hello") {
        println!("第一个字符: {}", first_char);
    }
    
    // 8. 枚举中的生命周期
    let message_text = "这是一条消息";
    let messages = vec![
        Message::Text(message_text),
        Message::Number(42),
        Message::Text("另一条消息"),
    ];
    
    println!("消息列表:");
    for (i, msg) in messages.iter().enumerate() {
        println!("  消息 {}: {:?}", i + 1, msg);
    }
    
    // 9. 生命周期约束示例
    demonstrate_lifetime_bounds();
    
    println!("\n生命周期总结:");
    println!("- 确保引用始终有效");
    println!("- 防止悬垂引用");
    println!("- 编译时检查，零运行时开销");
    println!("- 大多数情况下可以省略(生命周期省略规则)");
    println!("- 'static是最长的生命周期");
}

fn demonstrate_lifetime_bounds() {
    println!("\n9. 生命周期约束:");
    
    let data = vec!["apple", "banana", "cherry"];
    
    // 这个闭包捕获了对data的引用
    let processor = |items: &Vec<&str>| {
        for item in items {
            println!("  处理: {}", item);
        }
    };
    
    processor(&data);
    // data的生命周期必须包含processor的使用
}