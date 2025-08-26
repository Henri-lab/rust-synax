
// 测试特性：模块系统、FnMut闭包、事件处理模式、move语义
// 语法要点：pub mod、impl关键字、FnMut trait、move闭包
// 功能：定义测试模块并演示事件监听器模式

// 基础概念示例 (ex1-ex5)
pub mod ex1;   // 闭包和所有权转移
pub mod ex2;   // move闭包和函数对比
pub mod ex3;   // Arc+Mutex多线程共享状态
pub mod ex4;   // 原子操作和线程安全  
pub mod ex5;   // 多线程请求处理模拟

// 核心语言特性 (ex6-ex15)
pub mod ex6;   // 模式匹配和Option类型
pub mod ex7;   // 结构体和方法定义
pub mod ex8;   // 枚举和数据携带
pub mod ex9;   // 错误处理和Result类型
pub mod ex10;  // 向量操作和集合方法
pub mod ex11;  // 字符串处理和UTF-8
pub mod ex12;  // HashMap和键值对操作
pub mod ex13;  // 迭代器和函数式编程
pub mod ex14;  // Trait定义和实现
pub mod ex15;  // 生命周期和借用检查

// 高级特性 (ex16-ex20)
pub mod ex16;  // 智能指针Box/Rc/RefCell
pub mod ex17;  // 宏系统和代码生成
pub mod ex18;  // 异步编程async/await
pub mod ex19;  // 文件I/O和路径操作
pub mod ex20;  // 测试框架和文档测试
#[derive(Debug)]
// 定义事件监听器结构体
struct EventListener {
    event_count: u32,
}

impl EventListener {
    fn new() -> EventListener {
        EventListener { event_count: 0 }
    }

    // `on_event` 方法接受一个 `FnMut` 闭包
    fn on_event<F>(&mut self, mut f: F)
    where
        F: FnMut(),
    {
        // 调用闭包
        f();
    }
}

pub fn main() {
    let mut listener = EventListener::new();

    // 在这里创建一个闭包，使用 `move` 关键字
    let mut event_handler = move || {
        // 在这里修改 listener 的 event_count
        listener.event_count += 1;
        println!("事件已处理，当前事件计数为：{}", listener.event_count);
    };

    // 调用 `on_event` 方法，传入闭包
    listener.on_event(&mut event_handler);
    listener.on_event(&mut event_handler);

    // 尝试在闭包被移动后，再次访问 `listener`
    // 你会发现，编译器会报错，因为所有权已经移动了。
    // listener.on_event(&mut event_handler);
}