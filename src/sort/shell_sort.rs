pub fn shell_sort<T: PartialOrd + Clone>(arr: &mut [T]) {
    let n = arr.len();
    let mut gap = n / 2;
    
    while gap > 0 {
        for i in gap..n {
            let temp = arr[i].clone();
            let mut j = i;
            
            while j >= gap && arr[j - gap] > temp {
                arr[j] = arr[j - gap].clone();
                j -= gap;
            }
            
            arr[j] = temp;
        }
        gap /= 2;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shell_sort() {
        let mut arr = vec![64, 34, 25, 12, 22, 11, 90];
        shell_sort(&mut arr);
        assert_eq!(arr, vec![11, 12, 22, 25, 34, 64, 90]);
    }

    #[test]
    fn test_empty_array() {
        let mut arr: Vec<i32> = vec![];
        shell_sort(&mut arr);
        assert_eq!(arr, Vec::<i32>::new());
    }

    #[test]
    fn test_single_element() {
        let mut arr = vec![1];
        shell_sort(&mut arr);
        assert_eq!(arr, vec![1]);
    }
}