pub fn radix_sort(arr: &mut [u32]) {
    if arr.is_empty() {
        return;
    }
    
    let max_val = *arr.iter().max().unwrap();
    let mut exp = 1;
    
    while max_val / exp > 0 {
        counting_sort_for_radix(arr, exp);
        exp *= 10;
    }
}

fn counting_sort_for_radix(arr: &mut [u32], exp: u32) {
    let n = arr.len();
    let mut output = vec![0u32; n];
    let mut count = [0usize; 10];
    
    for &item in arr.iter() {
        count[((item / exp) % 10) as usize] += 1;
    }
    
    for i in 1..10 {
        count[i] += count[i - 1];
    }
    
    for &item in arr.iter().rev() {
        let digit = ((item / exp) % 10) as usize;
        count[digit] -= 1;
        output[count[digit]] = item;
    }
    
    arr.copy_from_slice(&output);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_radix_sort() {
        let mut arr = vec![170, 45, 75, 90, 2, 802, 24, 66];
        radix_sort(&mut arr);
        assert_eq!(arr, vec![2, 24, 45, 66, 75, 90, 170, 802]);
    }

    #[test]
    fn test_empty_array() {
        let mut arr: Vec<u32> = vec![];
        radix_sort(&mut arr);
        assert_eq!(arr, Vec::<i32>::new());
    }

    #[test]
    fn test_single_element() {
        let mut arr = vec![1];
        radix_sort(&mut arr);
        assert_eq!(arr, vec![1]);
    }
}