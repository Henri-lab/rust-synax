// Rust语法练习项目库文件
// 为了支持测试和文档测试，将主要功能作为库暴露

pub mod test;
pub mod tree;
pub mod sort;
pub mod interface_practice;

// 重新导出一些常用的函数以支持文档测试
pub use test::ex20::{gcd, is_prime};