// 测试特性：测试框架、单元测试、集成测试、文档测试、断言宏
// 语法要点：#[test]、#[cfg(test)]、assert!系列宏、#[should_panic]
// 功能：演示Rust的测试系统和测试驱动开发实践

// 被测试的代码
pub struct Calculator {
    memory: f64,
}

impl Calculator {
    pub fn new() -> Self {
        Calculator { memory: 0.0 }
    }
    
    pub fn add(&mut self, value: f64) -> f64 {
        self.memory += value;
        self.memory
    }
    
    pub fn subtract(&mut self, value: f64) -> f64 {
        self.memory -= value;
        self.memory
    }
    
    pub fn multiply(&mut self, value: f64) -> f64 {
        self.memory *= value;
        self.memory
    }
    
    pub fn divide(&mut self, value: f64) -> Result<f64, String> {
        if value == 0.0 {
            Err("不能除以零".to_string())
        } else {
            self.memory /= value;
            Ok(self.memory)
        }
    }
    
    pub fn clear(&mut self) {
        self.memory = 0.0;
    }
    
    pub fn get_memory(&self) -> f64 {
        self.memory
    }
}

/// 计算两个数的最大公约数
/// 
/// # Examples
/// 
/// ```
/// use synax::gcd;
/// 
/// assert_eq!(gcd(48, 18), 6);
/// assert_eq!(gcd(54, 24), 6);
/// ```
pub fn gcd(a: u32, b: u32) -> u32 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

/// 检查数字是否为质数
/// 
/// # Examples
/// 
/// ```
/// use synax::is_prime;
/// 
/// assert!(is_prime(17));
/// assert!(!is_prime(4));
/// assert!(!is_prime(1));
/// ```
pub fn is_prime(n: u32) -> bool {
    if n < 2 {
        return false;
    }
    for i in 2..=(n as f64).sqrt() as u32 {
        if n % i == 0 {
            return false;
        }
    }
    true
}

pub fn fibonacci(n: u32) -> u64 {
    match n {
        0 => 0,
        1 => 1,
        _ => fibonacci(n - 1) + fibonacci(n - 2),
    }
}

pub fn panic_function() {
    panic!("故意的panic!");
}

// 自定义断言宏
macro_rules! assert_approx_eq {
    ($left:expr, $right:expr, $epsilon:expr) => {
        if ($left - $right).abs() > $epsilon {
            panic!("断言失败: {} ≈ {} (误差: {})", $left, $right, $epsilon);
        }
    };
}

pub fn main() {
    println!("=== 测试和文档演示 ===");
    println!("这个模块主要演示测试功能。");
    println!("运行 'cargo test' 来执行测试！");
    
    // 演示基本功能
    let mut calc = Calculator::new();
    calc.add(10.0);
    calc.multiply(2.0);
    println!("计算器结果: {}", calc.get_memory());
    
    println!("最大公约数 gcd(48, 18) = {}", gcd(48, 18));
    println!("17 是质数吗? {}", is_prime(17));
    println!("斐波那契数列第10项: {}", fibonacci(10));
}

// 测试模块
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculator_basic_operations() {
        let mut calc = Calculator::new();
        
        // 测试加法
        assert_eq!(calc.add(5.0), 5.0);
        assert_eq!(calc.add(3.0), 8.0);
        
        // 测试减法
        assert_eq!(calc.subtract(2.0), 6.0);
        
        // 测试乘法
        assert_eq!(calc.multiply(2.0), 12.0);
        
        // 测试记忆功能
        assert_eq!(calc.get_memory(), 12.0);
        
        // 测试清零
        calc.clear();
        assert_eq!(calc.get_memory(), 0.0);
    }
    
    #[test]
    fn test_calculator_division() {
        let mut calc = Calculator::new();
        calc.add(20.0);
        
        // 正常除法
        assert_eq!(calc.divide(4.0), Ok(5.0));
        assert_eq!(calc.get_memory(), 5.0);
        
        // 除以零的错误情况
        let result = calc.divide(0.0);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "不能除以零");
        
        // 内存应该保持不变
        assert_eq!(calc.get_memory(), 5.0);
    }
    
    #[test]
    fn test_gcd() {
        assert_eq!(gcd(48, 18), 6);
        assert_eq!(gcd(54, 24), 6);
        assert_eq!(gcd(42, 56), 14);
        assert_eq!(gcd(17, 13), 1);  // 互质
        assert_eq!(gcd(100, 25), 25); // 整除
    }
    
    #[test] 
    fn test_is_prime() {
        // 质数测试
        assert!(is_prime(2));
        assert!(is_prime(3));
        assert!(is_prime(17));
        assert!(is_prime(97));
        
        // 非质数测试
        assert!(!is_prime(1));
        assert!(!is_prime(4));
        assert!(!is_prime(15));
        assert!(!is_prime(100));
    }
    
    #[test]
    fn test_fibonacci() {
        assert_eq!(fibonacci(0), 0);
        assert_eq!(fibonacci(1), 1);
        assert_eq!(fibonacci(2), 1);
        assert_eq!(fibonacci(3), 2);
        assert_eq!(fibonacci(4), 3);
        assert_eq!(fibonacci(5), 5);
        assert_eq!(fibonacci(10), 55);
    }
    
    #[test]
    #[should_panic(expected = "故意的panic!")]
    fn test_panic_function() {
        panic_function();
    }
    
    #[test]
    fn test_with_result() -> Result<(), String> {
        let mut calc = Calculator::new();
        calc.add(10.0);
        
        if calc.get_memory() == 10.0 {
            Ok(())
        } else {
            Err("计算器状态不正确".to_string())
        }
    }
    
    #[test]
    fn test_floating_point_precision() {
        let mut calc = Calculator::new();
        calc.add(0.1);
        calc.add(0.2);
        
        // 浮点数精度问题演示
        let result = calc.get_memory();
        assert_approx_eq!(result, 0.3f64, 0.0001f64);
    }
    
    #[test]
    fn test_multiple_assertions() {
        let mut calc = Calculator::new();
        
        // 链式操作测试
        calc.add(10.0);
        calc.multiply(2.0);
        calc.subtract(5.0);
        
        let result = calc.get_memory();
        assert!(result > 0.0, "结果应该是正数");
        assert!(result < 20.0, "结果应该小于20");
        assert_eq!(result, 15.0, "具体值应该是15");
    }
    
    #[test]
    #[ignore] // 这个测试会被忽略，除非使用 --ignored 标志
    fn expensive_test() {
        // 模拟一个耗时的测试
        std::thread::sleep(std::time::Duration::from_secs(1));
        assert_eq!(2 + 2, 4);
    }
    
    // 测试私有函数的例子
    #[test]
    fn test_internal_functionality() {
        // 在同一个模块中，可以测试私有函数
        fn internal_helper(x: i32) -> i32 {
            x * 2 + 1
        }
        
        assert_eq!(internal_helper(5), 11);
        assert_eq!(internal_helper(0), 1);
    }
}

// 基准测试示例（需要nightly Rust和#![feature(test)]）
#[cfg(test)]
mod bench_tests {
    use super::*;
    
    // 注意：实际的基准测试需要test crate和nightly Rust
    // 这里只是演示结构
    
    #[test]
    fn bench_fibonacci() {
        // 简单的性能测试模拟
        let start = std::time::Instant::now();
        let _ = fibonacci(20);
        let duration = start.elapsed();
        
        println!("斐波那契(20)耗时: {:?}", duration);
        // 在实际基准测试中，会有更精确的测量
    }
}