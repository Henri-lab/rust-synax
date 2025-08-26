pub fn bucket_sort(arr: &mut [f64]) {
    if arr.is_empty() {
        return;
    }
    
    let n = arr.len();
    let mut buckets: Vec<Vec<f64>> = vec![vec![]; n];
    
    for &item in arr.iter() {
        let bucket_idx = (item * n as f64) as usize;
        let bucket_idx = if bucket_idx >= n { n - 1 } else { bucket_idx };
        buckets[bucket_idx].push(item);
    }
    
    for bucket in buckets.iter_mut() {
        bucket.sort_by(|a, b| a.partial_cmp(b).unwrap());
    }
    
    let mut idx = 0;
    for bucket in buckets {
        for item in bucket {
            arr[idx] = item;
            idx += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bucket_sort() {
        let mut arr = vec![0.897, 0.565, 0.656, 0.1234, 0.665, 0.3434];
        bucket_sort(&mut arr);
        assert_eq!(arr, vec![0.1234, 0.3434, 0.565, 0.656, 0.665, 0.897]);
    }

    #[test]
    fn test_empty_array() {
        let mut arr: Vec<f64> = vec![];
        bucket_sort(&mut arr);
        assert_eq!(arr, Vec::<i32>::new());
    }

    #[test]
    fn test_single_element() {
        let mut arr = vec![0.5];
        bucket_sort(&mut arr);
        assert_eq!(arr, vec![0.5]);
    }
}