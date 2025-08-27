use std::ops::{Deref, DerefMut};
use std::alloc::{alloc, dealloc, Layout};

// 我们自己的智能指针结构体
struct MyBox<T> {
    ptr: *mut T,
}

impl<T> MyBox<T> {
    fn new(value: T) -> MyBox<T> {
        let layout = Layout::new::<T>();
        let ptr = unsafe {
            alloc(layout) as *mut T
        };
        if ptr.is_null() {
            panic!("Memory allocation failed");
        }
        unsafe {
            // 将值写入分配的内存
            std::ptr::write(ptr, value);
        }
        MyBox { ptr }
    }
}

// 核心：实现Deref，让它像引用一样被解引用
impl<T> Deref for MyBox<T> {
    type Target = T;
    fn deref(&self) -> &T {
        unsafe { &*self.ptr }
    }
}

// 核心：实现DerefMut，让它像可变引用一样被解引用
impl<T> DerefMut for MyBox<T> {
    fn deref_mut(&mut self) -> &mut T {
        unsafe { &mut *self.ptr }
    }
}

// 核心：实现Drop，自动释放内存
impl<T> Drop for MyBox<T> {
    fn drop(&mut self) {
        let layout = Layout::new::<T>();
        unsafe {
            // 首先执行T的析构函数
            std::ptr::drop_in_place(self.ptr);
            // 然后释放内存
            dealloc(self.ptr as *mut u8, layout);
        }
        println!("MyBox dropped, memory freed!");
    }
}

pub fn main() {
    // 1. 使用我们自己的智能指针
    let mut my_box = MyBox::new(123);
    
    // 2. 像引用一样使用它，无需unsafe
    println!("Value from MyBox: {}", *my_box);
    
    // 3. 修改它的值，就像可变引用一样
    *my_box += 1;
    println!("New value: {}", *my_box);

    // 4. 当my_box超出作用域时，Drop trait会自动释放内存，无需手动操作
    // MyBox dropped, memory freed!
}