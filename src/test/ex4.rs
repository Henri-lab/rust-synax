// 测试特性：原子操作(Atomic Operations)、无锁并发、线程安全
// 语法要点：AtomicI32、Ordering、static变量、原子操作方法
// 功能：演示使用原子类型实现线程安全的计数器，避免使用Mutex的开销
use std::thread;
use std::sync::atomic::{AtomicI32, Ordering};

static SAFE_COUNTER: AtomicI32 = AtomicI32::new(0);

fn main() {
    let mut handles = vec![];

    for _ in 0..10 {
        let handle = thread::spawn(|| {
            // 使用原子操作保证线程安全
            SAFE_COUNTER.fetch_add(1, Ordering::SeqCst);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    // 结果是可预测的，应该是 10
    println!("最终计数 (安全): {}", SAFE_COUNTER.load(Ordering::SeqCst));
}