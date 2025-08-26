#![allow(warnings)]

// === Rust 语法练习项目 ===
// 本项目包含多个模块，每个模块演示不同的Rust特性和语法
// 取消注释对应的函数调用即可运行相应的示例

mod test;        // 基础语法测试：闭包、并发、所有权
mod tree;        // 数据结构：二叉树实现
mod sort;        // 算法集合：各种排序算法
mod interface_practice; // 高级特性：trait、生命周期、异步等

fn main() {
    println!("🦀 欢迎使用 Rust 语法练习项目！");
    println!("现在包含 20 个核心特性示例，取消注释来运行：");
    println!();
    
    // === 基础概念 (ex1-ex5) ===
    println!("📚 基础概念:");
    // test::ex1::main();   // 闭包和所有权转移
    // test::ex2::main();   // move闭包和函数对比  
    // test::ex3::main();   // Arc+Mutex多线程共享状态
    // test::ex4::main();   // 原子操作和线程安全
    // test::ex5::main();   // 多线程请求处理模拟
    
    // === 核心语言特性 (ex6-ex15) ===
    println!("🔧 核心语言特性:");
    // test::ex6::main();   // 模式匹配和Option类型
    // test::ex7::main();   // 结构体和方法定义
    // test::ex8::main();   // 枚举和数据携带
    // test::ex9::main();   // 错误处理和Result类型
    // test::ex10::main();  // 向量操作和集合方法
    // test::ex11::main();  // 字符串处理和UTF-8
    // test::ex12::main();  // HashMap和键值对操作
    // test::ex13::main();  // 迭代器和函数式编程
    // test::ex14::main();  // Trait定义和实现
    // test::ex15::main();  // 生命周期和借用检查
    
    // === 高级特性 (ex16-ex20) ===
    println!("🚀 高级特性:");
    // test::ex16::main();  // 智能指针Box/Rc/RefCell
    // test::ex17::main();  // 宏系统和代码生成
    // test::ex18::main();  // 异步编程async/await
    // test::ex19::main();  // 文件I/O和路径操作
    // test::ex20::main();  // 测试框架和文档测试
    
    // === 其他模块 ===
    println!("🛠️  其他模块:");
    // test::main();        // 事件监听器模式
    // tree::main();        // 二叉树遍历算法
    // sort::demo_all_sorts(); // 排序算法对比演示
    // interface_practice::demo_interface_concepts(); // 接口特性概览
    
    // 默认运行第一个示例
    test::ex1::main();
    
    println!("\n💡 学习建议：");
    println!("📖 初学者路径: ex1→ex6→ex7→ex8→ex9");
    println!("🔄 进阶路径: ex10→ex11→ex12→ex13→ex14→ex15");  
    println!("🎯 高级路径: ex16→ex17→ex18→ex19→ex20");
    println!("🧪 运行测试: cargo test");
    println!("📚 查看文档: cargo doc --open");
}