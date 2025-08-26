// 测试特性：Trait定义、关联类型(Associated Types)、泛型接口设计
// 语法要点：trait关键字、type Item、Result<T, E>、enum定义
// 功能：定义数据处理相关的trait接口，演示Rust的接口抽象能力
pub trait DataSource {
    type Item;
    fn fetch_data(&self) -> Result<Vec<Self::Item>, String>;
    fn is_available(&self) -> bool;
}

pub trait DataProcessor {
    type Input;
    type Output;
    
    fn process(&self, data: Self::Input) -> Result<Self::Output, String>;
    fn validate_input(&self, data: &Self::Input) -> bool;
}

pub trait DataStorage {
    type Data;
    
    fn save(&mut self, data: Self::Data) -> Result<(), String>;
    fn load(&self, id: &str) -> Result<Option<Self::Data>, String>;
    fn delete(&mut self, id: &str) -> Result<bool, String>;
}

pub trait Logger {
    fn log(&self, level: LogLevel, message: &str);
    fn is_enabled(&self, level: LogLevel) -> bool;
}

#[derive(Debug, Clone, Copy)]
pub enum LogLevel {
    Debug,
    Info,
    Warning,
    Error,
}