// 测试特性：快速排序算法、递归函数、切片分割、分治算法
// 语法要点：递归调用、切片操作&mut [T]、分区(partition)函数
// 功能：实现高效的快速排序算法，演示递归和分治思想
pub fn quick_sort<T: PartialOrd>(arr: &mut [T]) {
    if arr.len() <= 1 {
        return;
    }
    
    let pivot = partition(arr);
    quick_sort(&mut arr[..pivot]);
    quick_sort(&mut arr[pivot + 1..]);
}

fn partition<T: PartialOrd>(arr: &mut [T]) -> usize {
    let len = arr.len();
    let pivot = len - 1;
    let mut i = 0;
    
    for j in 0..len - 1 {
        if arr[j] <= arr[pivot] {
            arr.swap(i, j);
            i += 1;
        }
    }
    
    arr.swap(i, pivot);
    i
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quick_sort() {
        let mut arr = vec![64, 34, 25, 12, 22, 11, 90];
        quick_sort(&mut arr);
        assert_eq!(arr, vec![11, 12, 22, 25, 34, 64, 90]);
    }

    #[test]
    fn test_empty_array() {
        let mut arr: Vec<i32> = vec![];
        quick_sort(&mut arr);
        assert_eq!(arr, Vec::<i32>::new());
    }

    #[test]
    fn test_single_element() {
        let mut arr = vec![1];
        quick_sort(&mut arr);
        assert_eq!(arr, vec![1]);
    }
}