// 测试特性：Trait定义和实现、多态、默认实现、Trait边界
// 语法要点：trait关键字、impl Trait for Type、默认方法、where子句
// 功能：演示Rust的Trait系统，实现代码复用和多态

// 定义基本trait
trait Drawable {
    fn draw(&self);
    
    // 默认实现
    fn description(&self) -> String {
        String::from("这是一个可绘制的对象")
    }
}

trait Calculable {
    fn area(&self) -> f64;
    fn perimeter(&self) -> f64;
}

// 定义结构体
struct Circle {
    radius: f64,
}

struct Rectangle {
    width: f64,
    height: f64,
}

struct Triangle {
    a: f64,
    b: f64,
    c: f64,
}

// 为Circle实现trait
impl Drawable for Circle {
    fn draw(&self) {
        println!("绘制半径为{}的圆形", self.radius);
    }
    
    fn description(&self) -> String {
        format!("半径为{}的圆形", self.radius)
    }
}

impl Calculable for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }
    
    fn perimeter(&self) -> f64 {
        2.0 * std::f64::consts::PI * self.radius
    }
}

// 为Rectangle实现trait
impl Drawable for Rectangle {
    fn draw(&self) {
        println!("绘制{}x{}的矩形", self.width, self.height);
    }
}

impl Calculable for Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }
    
    fn perimeter(&self) -> f64 {
        2.0 * (self.width + self.height)
    }
}

// 为Triangle实现trait
impl Drawable for Triangle {
    fn draw(&self) {
        println!("绘制边长为{}, {}, {}的三角形", self.a, self.b, self.c);
    }
}

impl Calculable for Triangle {
    fn area(&self) -> f64 {
        // 使用海伦公式
        let s = (self.a + self.b + self.c) / 2.0;
        (s * (s - self.a) * (s - self.b) * (s - self.c)).sqrt()
    }
    
    fn perimeter(&self) -> f64 {
        self.a + self.b + self.c
    }
}

// 使用trait作为参数
fn draw_shape(shape: &dyn Drawable) {
    shape.draw();
    println!("  {}", shape.description());
}

fn print_measurements(shape: &dyn Calculable) {
    println!("  面积: {:.2}", shape.area());
    println!("  周长: {:.2}", shape.perimeter());
}

// Trait边界的函数
fn process_shape<T>(shape: &T) 
where 
    T: Drawable + Calculable,
{
    shape.draw();
    println!("  面积: {:.2}", shape.area());
    println!("  周长: {:.2}", shape.perimeter());
}

// 返回trait对象
fn create_shape(shape_type: &str) -> Box<dyn Drawable> {
    match shape_type {
        "circle" => Box::new(Circle { radius: 5.0 }),
        "rectangle" => Box::new(Rectangle { width: 4.0, height: 6.0 }),
        "triangle" => Box::new(Triangle { a: 3.0, b: 4.0, c: 5.0 }),
        _ => Box::new(Circle { radius: 1.0 }),
    }
}

pub fn main() {
    println!("=== Trait演示 ===");
    
    // 1. 创建不同的形状
    let circle = Circle { radius: 3.0 };
    let rectangle = Rectangle { width: 5.0, height: 4.0 };
    let triangle = Triangle { a: 3.0, b: 4.0, c: 5.0 };
    
    // 2. 直接调用trait方法
    println!("1. 直接调用trait方法:");
    circle.draw();
    rectangle.draw();
    triangle.draw();
    
    // 3. 多态使用
    println!("\n2. 多态使用 - 绘制:");
    let shapes: Vec<&dyn Drawable> = vec![&circle, &rectangle, &triangle];
    for shape in shapes {
        draw_shape(shape);
    }
    
    // 4. 计算属性
    println!("\n3. 计算属性:");
    let calculable_shapes: Vec<&dyn Calculable> = vec![&circle, &rectangle, &triangle];
    for (i, shape) in calculable_shapes.iter().enumerate() {
        println!("形状 {}:", i + 1);
        print_measurements(*shape);
    }
    
    // 5. 使用trait边界
    println!("\n4. 使用trait边界:");
    process_shape(&circle);
    process_shape(&rectangle);
    process_shape(&triangle);
    
    // 6. 动态创建trait对象
    println!("\n5. 动态创建对象:");
    let shape_types = vec!["circle", "rectangle", "triangle"];
    for shape_type in shape_types {
        let shape = create_shape(shape_type);
        println!("创建 {}:", shape_type);
        draw_shape(shape.as_ref());
    }
    
    // 7. 默认实现演示
    println!("\n6. 默认实现:");
    println!("圆形描述: {}", circle.description());
    println!("矩形描述: {}", rectangle.description()); // 使用默认实现
    
    println!("\nTrait特点总结:");
    println!("- 定义共同的行为接口");
    println!("- 支持默认实现");
    println!("- 实现代码复用和多态");
    println!("- 可以作为trait边界约束泛型");
    println!("- 支持trait对象进行动态分发");
}