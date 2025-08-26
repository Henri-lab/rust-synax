// 测试特性：高级Rust特性集合、接口设计模式、系统编程概念
// 语法要点：pub mod、pub use、模块重新导出、特性组织
// 功能：集合了Rust的高级特性演示，包括trait、闭包、生命周期等

pub mod traits;
pub mod implementations;
pub mod pipeline;
pub mod example;
pub mod async_traits;
pub mod macros;
pub mod lifetimes;
pub mod unsafe_code;
pub mod const_generics;
pub mod closures;
pub mod error_handling;
pub mod concurrency;

// 重新导出主要的类型和trait
pub use traits::*;
pub use implementations::*;
pub use pipeline::*;
pub use example::*;

// 演示函数：展示接口实践的核心概念
pub fn demo_interface_concepts() {
    println!("=== Rust接口特性演示 ===");
    println!("1. Trait系统 - 定义共同行为");
    println!("2. 生命周期 - 内存安全保证");
    println!("3. 闭包系统 - 函数式编程");
    println!("4. 异步编程 - 并发处理");
    println!("5. 错误处理 - Result类型系统");
}