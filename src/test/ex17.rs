// 测试特性：宏系统(Macros)、声明式宏、代码生成、元编程
// 语法要点：macro_rules!、$()、重复模式、宏展开、过程宏基础
// 功能：演示Rust宏系统的基本用法和代码生成能力

// 1. 简单的声明式宏
macro_rules! say_hello {
    () => {
        println!("Hello from macro!");
    };
}

// 2. 带参数的宏
macro_rules! print_value {
    ($val:expr) => {
        println!("Value: {}", $val);
    };
}

// 3. 多种模式的宏
macro_rules! calculate {
    (add $a:expr, $b:expr) => {
        $a + $b
    };
    (multiply $a:expr, $b:expr) => {
        $a * $b
    };
    (square $x:expr) => {
        $x * $x
    };
}

// 4. 重复模式宏
macro_rules! create_function {
    ($func_name:ident, $return_type:ty, $body:expr) => {
        fn $func_name() -> $return_type {
            $body
        }
    };
}

// 5. 可变参数宏
macro_rules! print_all {
    ($($item:expr),*) => {
        $(
            println!("Item: {}", $item);
        )*
    };
}

// 6. 更复杂的重复模式
macro_rules! hash_map {
    ($($key:expr => $value:expr),*) => {
        {
            let mut temp_map = std::collections::HashMap::new();
            $(
                temp_map.insert($key, $value);
            )*
            temp_map
        }
    };
}

// 7. 创建结构体的宏
macro_rules! create_struct {
    ($struct_name:ident {
        $($field_name:ident: $field_type:ty),*
    }) => {
        #[derive(Debug)]
        struct $struct_name {
            $(
                $field_name: $field_type,
            )*
        }
        
        impl $struct_name {
            fn new($($field_name: $field_type),*) -> Self {
                $struct_name {
                    $(
                        $field_name,
                    )*
                }
            }
        }
    };
}

// 8. 条件编译宏
macro_rules! debug_print {
    ($($arg:tt)*) => {
        #[cfg(debug_assertions)]
        {
            println!("[DEBUG] {}", format!($($arg)*));
        }
    };
}

// 9. 实现trait的宏
macro_rules! impl_display {
    ($struct_name:ident, $format_str:expr) => {
        impl std::fmt::Display for $struct_name {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(f, $format_str, self.name, self.age, self.email)
            }
        }
    };
}

// 10. 测试宏
macro_rules! assert_approx_eq {
    ($left:expr, $right:expr, $epsilon:expr) => {
        if ($left - $right).abs() > $epsilon {
            panic!("assertion failed: {} ≈ {} (epsilon: {})", $left, $right, $epsilon);
        }
    };
}

// 使用宏创建结构体
create_struct!(Person {
    name: String,
    age: u32,
    email: String
});

// 使用宏创建函数
create_function!(get_magic_number, i32, 42);
create_function!(get_pi, f64, 3.14159);

impl_display!(Person, "Person[name={}, age={}, email={}]");

pub fn main() {
    println!("=== 宏系统演示 ===");
    
    // 1. 基本宏调用
    println!("1. 基本宏:");
    say_hello!();
    print_value!(42);
    print_value!("Hello World");
    
    // 2. 模式匹配宏
    println!("\n2. 模式匹配宏:");
    let sum = calculate!(add 5, 3);
    let product = calculate!(multiply 4, 6);
    let square = calculate!(square 7);
    println!("  5 + 3 = {}", sum);
    println!("  4 × 6 = {}", product);
    println!("  7² = {}", square);
    
    // 3. 使用宏创建的函数
    println!("\n3. 宏生成的函数:");
    println!("  魔法数字: {}", get_magic_number());
    println!("  π值: {}", get_pi());
    
    // 4. 可变参数宏
    println!("\n4. 可变参数宏:");
    print_all!(1, 2, 3, 4, 5);
    print_all!("apple", "banana", "cherry");
    
    // 5. HashMap宏
    println!("\n5. HashMap创建宏:");
    let scores = hash_map!(
        "Alice" => 95,
        "Bob" => 87,
        "Charlie" => 92
    );
    println!("  分数: {:?}", scores);
    
    let colors = hash_map!(
        "red" => "#FF0000",
        "green" => "#00FF00",
        "blue" => "#0000FF"
    );
    println!("  颜色: {:?}", colors);
    
    // 6. 使用宏创建的结构体
    println!("\n6. 宏生成的结构体:");
    let person = Person::new(
        "Alice".to_string(),
        25,
        "alice@example.com".to_string()
    );
    println!("  {}", person);
    println!("  详细: {:?}", person);
    
    // 7. 调试宏
    println!("\n7. 条件编译宏:");
    debug_print!("这是调试信息: {}", 42);
    debug_print!("调试模式才会显示");
    
    // 8. 内置宏演示
    println!("\n8. 内置宏:");
    println!("  文件: {}", file!());
    println!("  行号: {}", line!());
    println!("  模块: {}", module_path!());
    
    // 9. 格式化宏
    println!("\n9. 格式化宏:");
    let formatted = format!("用户: {}, 年龄: {}", "Bob", 30);
    println!("  {}", formatted);
    
    // 10. 自定义断言宏
    println!("\n10. 自定义断言宏:");
    let pi: f64 = 3.14159;
    let approx_pi: f64 = 3.14;
    assert_approx_eq!(pi, approx_pi, 0.01f64);
    println!("  近似相等检查通过!");
    
    // 11. vec!宏演示
    println!("\n11. vec!宏:");
    let numbers = vec![1, 2, 3, 4, 5];
    let repeated = vec![0; 5];
    println!("  数字: {:?}", numbers);
    println!("  重复: {:?}", repeated);
    
    println!("\n宏系统总结:");
    println!("- 声明式宏: 使用macro_rules!定义");
    println!("- 模式匹配: 支持复杂的参数模式");
    println!("- 重复模式: $()* 处理可变参数");
    println!("- 代码生成: 编译时生成代码");
    println!("- 卫生性: 避免变量名冲突");
    println!("- 内置宏: vec!, println!, format!等");
}