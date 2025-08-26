// 测试特性：结构体(Structs)、方法定义、关联函数、self关键字
// 语法要点：struct关键字、impl块、&self、Self::new()
// 功能：演示如何定义和使用结构体及其方法

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // 关联函数 (类似静态方法)
    fn new(width: u32, height: u32) -> Self {
        Rectangle { width, height }
    }
    
    // 方法 (需要self参数)
    fn area(&self) -> u32 {
        self.width * self.height
    }
    
    fn perimeter(&self) -> u32 {
        2 * (self.width + self.height)
    }
    
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
    
    // 可变方法
    fn scale(&mut self, factor: u32) {
        self.width *= factor;
        self.height *= factor;
    }
}

pub fn main() {
    println!("=== 结构体和方法演示 ===");
    
    // 使用关联函数创建实例
    let mut rect1 = Rectangle::new(30, 50);
    let rect2 = Rectangle::new(10, 40);
    
    println!("矩形1: {:?}", rect1);
    println!("矩形1面积: {}", rect1.area());
    println!("矩形1周长: {}", rect1.perimeter());
    
    if rect1.can_hold(&rect2) {
        println!("矩形1可以容纳矩形2");
    } else {
        println!("矩形1不能容纳矩形2");
    }
    
    // 调用可变方法
    println!("放大矩形1前: {:?}", rect1);
    rect1.scale(2);
    println!("放大矩形1后: {:?}", rect1);
}