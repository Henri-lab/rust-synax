// 测试特性：排序算法集合、模块组织、算法复杂度对比
// 语法要点：pub mod声明、算法实现、性能测试
// 功能：包含各种排序算法的实现，方便对比和学习不同算法特点

pub mod bubble_sort;
pub mod selection_sort;
pub mod insertion_sort;
pub mod merge_sort;
pub mod quick_sort;
pub mod heap_sort;
pub mod radix_sort;
pub mod counting_sort;
pub mod shell_sort;
pub mod bucket_sort;

// 重新导出常用的排序函数
pub use bubble_sort::bubble_sort;
pub use quick_sort::quick_sort;
pub use merge_sort::merge_sort;

// 演示函数：测试所有排序算法
pub fn demo_all_sorts() {
    println!("=== 排序算法演示 ===");
    
    let mut test_data = vec![64, 34, 25, 12, 22, 11, 90];
    println!("原始数据: {:?}", test_data);
    
    let mut bubble_data = test_data.clone();
    bubble_sort(&mut bubble_data);
    println!("冒泡排序结果: {:?}", bubble_data);
    
    let mut quick_data = test_data.clone();
    quick_sort(&mut quick_data);
    println!("快速排序结果: {:?}", quick_data);
    
    let mut merge_data = test_data.clone();
    merge_sort(&mut merge_data);
    println!("归并排序结果: {:?}", merge_data);
}