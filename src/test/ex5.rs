// 测试特性：多线程编程(Multi-threading)、请求处理、Arc+Mutex模式
// 语法要点：thread::sleep、Duration、线程ID获取、请求处理模拟
// 功能：模拟多线程服务器处理客户端请求，演示线程间协作和资源共享
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

// 代表处理客户端请求的函数
fn handle_request(request_id: u32, counter: &Arc<Mutex<u32>>) {
    println!("线程 {:?} 正在处理请求 #{}", thread::current().id(), request_id);
    
    // 模拟处理请求的耗时操作
    thread::sleep(Duration::from_millis(50));
    
    // 获取 Mutex 的锁以安全地修改计数器
    let mut num = counter.lock().unwrap();
    *num += 1;
    
    // 锁在 `num` 变量离开作用域时自动释放
}

fn main() {
    // 1. 使用 Arc 在多线程间共享所有权
    let counter = Arc::new(Mutex::new(0u32));
    let mut handles = vec![];
    
    // 2. 创建多个线程来处理请求
    for i in 0..10 {
        // 克隆 Arc，让每个线程都拥有对计数器的所有权
        let counter_clone = Arc::clone(&counter);
        
        let handle = thread::spawn(move || {
            // 在这里，`counter_clone` 的所有权被移动到闭包中
            handle_request(i, &counter_clone);
        });
        
        handles.push(handle);
    }
    
    // 3. 等待所有线程完成
    for handle in handles {
        handle.join().unwrap();
    }
    
    // 4. 打印最终结果
    // 再次获取锁以安全地读取最终计数器值
    println!("---------------------");
    println!("所有请求已处理完毕.");
    println!("最终请求总数: {}", *counter.lock().unwrap());
}