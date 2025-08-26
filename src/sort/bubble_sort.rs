// 测试特性：冒泡排序算法、泛型(Generics)、Trait约束、单元测试
// 语法要点：<T: PartialOrd>、切片操作、swap方法、#[cfg(test)]、assert_eq!
// 功能：实现泛型冒泡排序算法，支持任何实现PartialOrd的类型
pub fn bubble_sort<T: PartialOrd>(arr: &mut [T]) {
    let n = arr.len();
    for i in 0..n {
        for j in 0..n - 1 - i {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bubble_sort() {
        let mut arr = vec![64, 34, 25, 12, 22, 11, 90];
        bubble_sort(&mut arr);
        assert_eq!(arr, vec![11, 12, 22, 25, 34, 64, 90]);
    }

    #[test]
    fn test_empty_array() {
        let mut arr: Vec<i32> = vec![];
        bubble_sort(&mut arr);
        assert_eq!(arr, Vec::<i32>::new());
    }

    #[test]
    fn test_single_element() {
        let mut arr = vec![1];
        bubble_sort(&mut arr);
        assert_eq!(arr, vec![1]);
    }
}