// 测试特性：并发编程(Concurrency)、共享状态(Shared State)、Mutex、Arc
// 语法要点：Arc<Mutex<T>>、thread::spawn、智能指针、线程同步
// 功能：演示多线程环境下使用Arc和Mutex安全地共享和修改数据
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    // 创建一个可共享、可变的计数器
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        // 克隆 Arc，以在每个线程中共享所有权
        let counter_clone = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            // 获取 Mutex 的锁
            let mut num = counter_clone.lock().unwrap();
            *num += 1;
            // 锁在 `num` 变量离开作用域时自动释放
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("最终结果: {}", *counter.lock().unwrap());
}