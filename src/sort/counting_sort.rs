pub fn counting_sort(arr: &mut [usize], max_val: usize) {
    let n = arr.len();
    if n == 0 {
        return;
    }
    
    let mut count = vec![0usize; max_val + 1];
    let mut output = vec![0usize; n];
    
    for &item in arr.iter() {
        count[item] += 1;
    }
    
    for i in 1..=max_val {
        count[i] += count[i - 1];
    }
    
    for &item in arr.iter().rev() {
        count[item] -= 1;
        output[count[item]] = item;
    }
    
    arr.copy_from_slice(&output);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_counting_sort() {
        let mut arr = vec![4, 2, 2, 8, 3, 3, 1];
        counting_sort(&mut arr, 8);
        assert_eq!(arr, vec![1, 2, 2, 3, 3, 4, 8]);
    }

    #[test]
    fn test_empty_array() {
        let mut arr: Vec<usize> = vec![];
        counting_sort(&mut arr, 0);
        assert_eq!(arr, Vec::<i32>::new());
    }

    #[test]
    fn test_single_element() {
        let mut arr = vec![1];
        counting_sort(&mut arr, 1);
        assert_eq!(arr, vec![1]);
    }
}