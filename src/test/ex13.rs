// 测试特性：迭代器(Iterators)、函数式编程、链式调用、惰性求值
// 语法要点：iter()、map()、filter()、collect()、fold()、for_each()
// 功能：演示Rust强大的迭代器系统和函数式编程特性

pub fn main() {
    println!("=== 迭代器演示 ===");
    
    // 1. 创建迭代器的方式
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    println!("原始数据: {:?}", numbers);
    
    // 2. 基本的迭代器适配器
    let doubled: Vec<i32> = numbers
        .iter()
        .map(|x| x * 2)
        .collect();
    println!("翻倍结果: {:?}", doubled);
    
    let evens: Vec<&i32> = numbers
        .iter()
        .filter(|&&x| x % 2 == 0)
        .collect();
    println!("偶数: {:?}", evens);
    
    // 3. 链式调用演示
    let result: Vec<i32> = numbers
        .iter()
        .filter(|&&x| x > 3)      // 筛选大于3的数
        .map(|x| x * x)           // 平方
        .filter(|&x| x < 50)     // 筛选小于50的数
        .collect();
    println!("链式处理结果: {:?}", result);
    
    // 4. 消费性适配器
    let sum: i32 = numbers.iter().sum();
    println!("求和: {}", sum);
    
    let product: i32 = numbers.iter().product();
    println!("求积: {}", product);
    
    let max = numbers.iter().max();
    println!("最大值: {:?}", max);
    
    let min = numbers.iter().min();
    println!("最小值: {:?}", min);
    
    // 5. fold 和 reduce
    let sum_fold = numbers.iter().fold(0, |acc, &x| acc + x);
    println!("使用fold求和: {}", sum_fold);
    
    let factorial = (1..=5).fold(1, |acc, x| acc * x);
    println!("5的阶乘: {}", factorial);
    
    // 6. 字符串处理示例
    let words = vec!["hello", "world", "rust", "programming"];
    
    let lengths: Vec<usize> = words
        .iter()
        .map(|word| word.len())
        .collect();
    println!("单词长度: {:?}", lengths);
    
    let long_words: Vec<&str> = words
        .iter()
        .filter(|word| word.len() > 4)
        .cloned()
        .collect();
    println!("长单词: {:?}", long_words);
    
    let uppercase: String = words
        .iter()
        .map(|word| word.to_uppercase())
        .collect::<Vec<String>>()
        .join(" ");
    println!("大写拼接: {}", uppercase);
    
    // 7. enumerate 和 zip
    println!("\nenumerate演示:");
    for (index, value) in numbers.iter().enumerate() {
        if index < 3 {
            println!("  索引{}: 值{}", index, value);
        }
    }
    
    let letters = vec!['a', 'b', 'c', 'd', 'e'];
    let zipped: Vec<(i32, char)> = numbers
        .iter()
        .take(5)
        .cloned()
        .zip(letters.iter().cloned())
        .collect();
    println!("zip结果: {:?}", zipped);
    
    // 8. 迭代器的惰性求值
    println!("\n惰性求值演示:");
    let lazy_iter = numbers
        .iter()
        .map(|&x| {
            println!("  处理: {}", x); // 只有在消费时才会执行
            x * 2
        });
    
    println!("创建了迭代器，但还没有处理任何元素");
    
    // 只处理前3个元素
    let first_three: Vec<i32> = lazy_iter.take(3).collect();
    println!("前三个翻倍: {:?}", first_three);
    
    // 9. 自定义迭代器适配器
    let custom_result: Vec<String> = (1..=5)
        .map(|x| x * x)
        .filter(|&x| x > 5)
        .map(|x| format!("数字: {}", x))
        .collect();
    println!("自定义处理: {:?}", custom_result);
    
    // 10. for_each 副作用操作
    println!("\nfor_each演示:");
    numbers
        .iter()
        .filter(|&&x| x % 3 == 0)
        .for_each(|x| println!("  3的倍数: {}", x));
    
    println!("\n迭代器特点总结:");
    println!("- 惰性求值：只有在消费时才执行");
    println!("- 零成本抽象：编译时优化");
    println!("- 链式调用：可以组合多个操作");
    println!("- 函数式编程：支持map、filter、fold等");
}