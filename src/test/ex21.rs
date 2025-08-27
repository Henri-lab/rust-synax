use std::alloc::{alloc, dealloc, Layout};

pub fn main() {
    // 1. 定义一个布局，指定分配内存的大小和对齐方式
    let layout = Layout::new::<i32>();
    
    // 2. 使用不安全的原始指针来手动分配内存
    let ptr = unsafe {
        alloc(layout) as *mut i32
    };

    if ptr.is_null() {
        panic!("Memory allocation failed");
    }

    // 3. 将数据写入这块内存
    unsafe {
        *ptr = 123;
    }

    // 4. 从这块内存中读取数据
    let value = unsafe {
        *ptr
    };

    println!("Value from raw pointer: {}", value);
    
    // 5. 手动释放内存，这是最容易出错的一步
    unsafe {
        dealloc(ptr as *mut u8, layout);
    }

    // 警告：如果你忘记了第5步，就会发生内存泄漏
    // 如果你在两个地方释放了同一块内存，就会发生双重释放错误
}