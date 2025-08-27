// 测试特性：闭包(Closures)、move关键字、所有权转移(Ownership)
// 语法要点：move closure、函数指针、const常量、所有权转移
// 功能：对比普通函数与闭包，演示move闭包如何强制获取所有权
const X: i32 = 2;

fn func(x: i32) -> i32 {
    x + 1
}

pub fn main() {
    let fun2 = |x: i32| -> i32 { x + 1 };
    let result = func(X);
    let result2 = fun2(X);
    println!("Result: {}", result);
    println!("Result2: {}", result2);

    let my_string = String::from("hello");

    // 使用 `move` 强制闭包获取 `my_string` 的所有权
    let move_closure = move || {
        println!("Inside closure: {}", my_string);
    };

    // 尝试访问 `my_string` 会报错，因为所有权已经移动了
    // println!("Outside closure: {}", my_string); // 编译错误！
    // println!("result:{}",result);
    main2();

    move_closure();
}
static a: i32=1;
// 通过宏自动派生（derive）Clone 和 Copy trait
#[derive(Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

fn main2() {
    let p1 = Point { x: 1, y: 2 };

    // 赋值时，p1 的值被复制到了 p2
    let mut p2 = p1;
    let clone = p1.clone();
    println!("p2: ({}, {})", p2.x, p2.y);
    p2 = clone;

    println!("p2: ({}, {})", p2.x, p2.y);
    // p1 仍然有效，因为 Point 实现了 Copy
    println!("p1: ({}, {})", p1.x, p1.y);

    // 如果 Point 没有实现 Copy，这行代码会报错
}
