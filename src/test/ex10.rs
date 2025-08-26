// 测试特性：向量(Vectors)、动态数组操作、迭代器、集合方法
// 语法要点：Vec<T>、push()、pop()、iter()、collect()、索引访问
// 功能：演示Vec的创建、操作和常用方法

pub fn main() {
    println!("=== 向量(Vec)演示 ===");
    
    // 1. 创建向量的不同方法
    let mut numbers = Vec::new(); // 空向量
    let mut fruits = vec!["apple", "banana", "cherry"]; // 宏创建
    let zeros = vec![0; 5]; // 创建5个0
    
    println!("初始fruits: {:?}", fruits);
    println!("5个0: {:?}", zeros);
    
    // 2. 添加和删除元素
    numbers.push(1);
    numbers.push(2);
    numbers.push(3);
    println!("添加元素后: {:?}", numbers);
    
    if let Some(last) = numbers.pop() {
        println!("弹出的元素: {}", last);
    }
    println!("弹出元素后: {:?}", numbers);
    
    // 3. 访问元素
    fruits.push("date");
    println!("第一个水果: {}", fruits[0]); // 可能panic
    
    // 安全访问
    match fruits.get(10) {
        Some(fruit) => println!("第11个水果: {}", fruit),
        None => println!("没有第11个水果"),
    }
    
    // 4. 遍历向量
    println!("\n遍历水果:");
    for (index, fruit) in fruits.iter().enumerate() {
        println!("  {}. {}", index + 1, fruit);
    }
    
    // 5. 向量的常用方法
    let mut scores = vec![85, 92, 78, 96, 88];
    println!("\n原始分数: {:?}", scores);
    
    println!("长度: {}", scores.len());
    println!("是否为空: {}", scores.is_empty());
    println!("最高分: {:?}", scores.iter().max());
    println!("最低分: {:?}", scores.iter().min());
    
    // 6. 向量操作
    scores.sort();
    println!("排序后: {:?}", scores);
    
    let doubled: Vec<i32> = scores.iter().map(|x| x * 2).collect();
    println!("分数翻倍: {:?}", doubled);
    
    let high_scores: Vec<&i32> = scores.iter().filter(|&&x| x > 90).collect();
    println!("高分(>90): {:?}", high_scores);
    
    // 7. 向量的切片
    let slice = &scores[1..4];
    println!("切片 [1..4]: {:?}", slice);
    
    // 8. 向量拼接
    let mut vec1 = vec![1, 2, 3];
    let vec2 = vec![4, 5, 6];
    vec1.extend(vec2);
    println!("拼接后: {:?}", vec1);
    
    println!("\nVec特点总结:");
    println!("- 动态大小的数组");
    println!("- 连续内存存储");  
    println!("- 支持随机访问");
    println!("- 丰富的方法和迭代器支持");
}