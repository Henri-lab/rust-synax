// 测试特性：HashMap集合、键值对操作、哈希映射、entry API
// 语法要点：HashMap::new()、insert()、get()、entry()、or_insert()
// 功能：演示HashMap的创建、操作和常用模式

use std::collections::HashMap;

pub fn main() {
    println!("=== HashMap演示 ===");
    
    // 1. 创建HashMap的不同方法
    let mut scores = HashMap::new();
    
    // 插入键值对
    scores.insert(String::from("Alice"), 95);
    scores.insert(String::from("Bob"), 87);
    scores.insert(String::from("Charlie"), 92);
    
    println!("初始分数: {:?}", scores);
    
    // 2. 访问值
    let alice_score = scores.get("Alice");
    match alice_score {
        Some(score) => println!("Alice的分数: {}", score),
        None => println!("没找到Alice的分数"),
    }
    
    // 3. 遍历HashMap
    println!("\n所有分数:");
    for (name, score) in &scores {
        println!("  {}: {}", name, score);
    }
    
    // 4. 更新值的不同方式
    println!("\n更新值演示:");
    
    // 直接覆盖
    scores.insert(String::from("Alice"), 98);
    println!("覆盖后Alice的分数: {:?}", scores.get("Alice"));
    
    // 只在键不存在时插入
    scores.entry(String::from("David")).or_insert(88);
    scores.entry(String::from("Alice")).or_insert(100); // Alice已存在，不会插入
    println!("插入David后: {:?}", scores);
    
    // 5. 基于旧值更新
    let text = "hello world wonderful world";
    let mut word_count = HashMap::new();
    
    for word in text.split_whitespace() {
        let count = word_count.entry(word).or_insert(0);
        *count += 1;
    }
    
    println!("\n单词计数:");
    for (word, count) in &word_count {
        println!("  '{}': {}", word, count);
    }
    
    // 6. HashMap的常用方法
    println!("\nHashMap方法演示:");
    println!("长度: {}", scores.len());
    println!("是否为空: {}", scores.is_empty());
    println!("包含Alice: {}", scores.contains_key("Alice"));
    
    // 移除元素
    if let Some(removed_score) = scores.remove("Bob") {
        println!("移除了Bob的分数: {}", removed_score);
    }
    println!("移除Bob后: {:?}", scores);
    
    // 7. 使用自定义类型作为键
    #[derive(Hash, Eq, PartialEq, Debug)]
    struct Student {
        name: String,
        id: u32,
    }
    
    let mut student_grades = HashMap::new();
    
    let alice = Student { 
        name: String::from("Alice"), 
        id: 1001 
    };
    let bob = Student { 
        name: String::from("Bob"), 
        id: 1002 
    };
    
    student_grades.insert(alice, vec![85, 92, 78]);
    student_grades.insert(bob, vec![90, 88, 94]);
    
    println!("\n学生成绩记录:");
    for (student, grades) in &student_grades {
        println!("  {:?}: {:?}", student, grades);
    }
    
    // 8. HashMap的迭代方式
    println!("\n不同的迭代方式:");
    
    // 只迭代键
    println!("学生姓名:");
    for name in scores.keys() {
        println!("  {}", name);
    }
    
    // 只迭代值  
    println!("所有分数:");
    for score in scores.values() {
        println!("  {}", score);
    }
    
    // 可变引用迭代值
    for score in scores.values_mut() {
        *score += 5; // 所有分数加5分
    }
    println!("加分后: {:?}", scores);
    
    println!("\nHashMap特点总结:");
    println!("- 基于哈希表的键值对集合");
    println!("- 键必须实现Hash和Eq trait");
    println!("- 平均O(1)时间复杂度的查找");
    println!("- 键和值的所有权会转移到HashMap");
}