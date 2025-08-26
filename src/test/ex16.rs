// 测试特性：智能指针(Smart Pointers)、Box、Rc、RefCell、内存管理
// 语法要点：Box<T>、Rc<T>、RefCell<T>、Deref、Drop trait
// 功能：演示Rust的智能指针系统和不同的内存管理策略

use std::rc::Rc;
use std::cell::RefCell;

// 使用Box的链表节点
#[derive(Debug)]
struct ListNode {
    value: i32,
    next: Option<Box<ListNode>>,
}

impl ListNode {
    fn new(value: i32) -> Self {
        ListNode { value, next: None }
    }
    
    fn append(&mut self, value: i32) {
        match &mut self.next {
            Some(node) => node.append(value),
            None => self.next = Some(Box::new(ListNode::new(value))),
        }
    }
}

// 使用Rc的共享数据
#[derive(Debug)]
struct SharedData {
    name: String,
    value: i32,
}

// 使用RefCell的可变借用
#[derive(Debug)]
struct Counter {
    count: RefCell<i32>,
}

impl Counter {
    fn new() -> Self {
        Counter {
            count: RefCell::new(0),
        }
    }
    
    fn increment(&self) {
        let mut count = self.count.borrow_mut();
        *count += 1;
    }
    
    fn get_count(&self) -> i32 {
        *self.count.borrow()
    }
}

// 二叉树节点使用Rc和RefCell
type TreeNodeRef = Rc<RefCell<TreeNode>>;

#[derive(Debug)]
struct TreeNode {
    value: i32,
    left: Option<TreeNodeRef>,
    right: Option<TreeNodeRef>,
    parent: Option<std::rc::Weak<RefCell<TreeNode>>>, // 使用Weak避免循环引用
}

impl TreeNode {
    fn new(value: i32) -> TreeNodeRef {
        Rc::new(RefCell::new(TreeNode {
            value,
            left: None,
            right: None,
            parent: None,
        }))
    }
    
    fn add_left_child(parent: &TreeNodeRef, value: i32) -> TreeNodeRef {
        let child = TreeNode::new(value);
        child.borrow_mut().parent = Some(Rc::downgrade(parent));
        parent.borrow_mut().left = Some(child.clone());
        child
    }
}

pub fn main() {
    println!("=== 智能指针演示 ===");
    
    // 1. Box<T> - 堆分配的智能指针
    println!("1. Box<T> 演示:");
    let mut list = ListNode::new(1);
    list.append(2);
    list.append(3);
    println!("  链表: {:?}", list);
    
    // Box用于递归类型
    let boxed_value = Box::new(42);
    println!("  Box值: {}", boxed_value);
    println!("  Box地址: {:p}", &*boxed_value);
    
    // 2. Rc<T> - 引用计数智能指针
    println!("\n2. Rc<T> 演示:");
    let shared_data = Rc::new(SharedData {
        name: "共享数据".to_string(),
        value: 100,
    });
    
    let reference1 = Rc::clone(&shared_data);
    let reference2 = Rc::clone(&shared_data);
    let reference3 = shared_data.clone(); // 等价于Rc::clone
    
    println!("  引用计数: {}", Rc::strong_count(&shared_data));
    println!("  数据: {:?}", shared_data);
    
    drop(reference1);
    println!("  drop后引用计数: {}", Rc::strong_count(&shared_data));
    
    // 3. RefCell<T> - 内部可变性
    println!("\n3. RefCell<T> 演示:");
    let counter = Counter::new();
    println!("  初始计数: {}", counter.get_count());
    
    counter.increment();
    counter.increment();
    println!("  增加后计数: {}", counter.get_count());
    
    // 4. Rc<RefCell<T>> - 共享的可变数据
    println!("\n4. Rc<RefCell<T>> 演示:");
    let shared_counter = Rc::new(RefCell::new(0));
    
    let counter_clone1 = Rc::clone(&shared_counter);
    let counter_clone2 = Rc::clone(&shared_counter);
    
    *counter_clone1.borrow_mut() += 5;
    *counter_clone2.borrow_mut() += 3;
    
    println!("  共享计数器: {}", shared_counter.borrow());
    
    // 5. 树形结构示例
    println!("\n5. 树形结构演示:");
    let root = TreeNode::new(1);
    let left_child = TreeNode::add_left_child(&root, 2);
    
    println!("  根节点: {}", root.borrow().value);
    println!("  左子节点: {}", left_child.borrow().value);
    
    // 访问父节点
    if let Some(parent_weak) = &left_child.borrow().parent {
        if let Some(parent) = parent_weak.upgrade() {
            println!("  子节点的父节点: {}", parent.borrow().value);
        }
    }
    
    // 6. 智能指针的自动解引用
    println!("\n6. 自动解引用演示:");
    let string_box = Box::new(String::from("Hello"));
    let string_rc = Rc::new(String::from("World"));
    
    // 可以直接调用String的方法
    println!("  Box字符串长度: {}", string_box.len());
    println!("  Rc字符串长度: {}", string_rc.len());
    
    // 7. 循环引用问题演示（注释掉避免内存泄露）
    println!("\n7. Weak引用避免循环:");
    println!("  使用Weak<T>可以避免Rc的循环引用问题");
    println!("  父节点持有子节点的强引用");
    println!("  子节点持有父节点的弱引用");
    
    println!("\n智能指针总结:");
    println!("- Box<T>: 单一所有者，堆分配");
    println!("- Rc<T>: 多个所有者，引用计数");
    println!("- RefCell<T>: 内部可变性，运行时借用检查");
    println!("- Weak<T>: 弱引用，避免循环引用");
    println!("- 所有智能指针都实现了Deref和Drop");
}