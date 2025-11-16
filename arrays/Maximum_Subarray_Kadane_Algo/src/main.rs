fn max_subarray_sum(arr: &[i32]) -> Option<i32> {
    if arr.is_empty() {
        return None;
    }
    
    let mut max_sum = arr[0];
    let mut current_sum = arr[0];
    
    for &num in &arr[1..] {
        // Either extend the existing subarray or start a new one
        current_sum = num.max(current_sum + num);
        // Update the maximum sum found so far
        max_sum = max_sum.max(current_sum);
    }
    
    Some(max_sum)
}

fn main() {
    // Test case 1
    let arr1 = vec![2, 3, -8, 7, -1, 2, 3];
    match max_subarray_sum(&arr1) {
        Some(sum) => println!("Input: {:?}\nOutput: {}\n", arr1, sum),
        None => println!("Empty array"),
    }
    
    // Test case 2
    let arr2 = vec![-2, -4];
    match max_subarray_sum(&arr2) {
        Some(sum) => println!("Input: {:?}\nOutput: {}\n", arr2, sum),
        None => println!("Empty array"),
    }
    
    // Test case 3
    let arr3 = vec![5, 4, 1, 7, 8];
    match max_subarray_sum(&arr3) {
        Some(sum) => println!("Input: {:?}\nOutput: {}\n", arr3, sum),
        None => println!("Empty array"),
    }
    
    // Additional test case with single element
    let arr4 = vec![10];
    match max_subarray_sum(&arr4) {
        Some(sum) => println!("Input: {:?}\nOutput: {}\n", arr4, sum),
        None => println!("Empty array"),
    }
    
    // Edge case: empty array
    let arr5: Vec<i32> = vec![];
    match max_subarray_sum(&arr5) {
        Some(sum) => println!("Input: {:?}\nOutput: {}\n", arr5, sum),
        None => println!("Input: {:?}\nOutput: Empty array (no subarray exists)\n", arr5),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_example_1() {
        assert_eq!(max_subarray_sum(&[2, 3, -8, 7, -1, 2, 3]), Some(11));
    }
    
    #[test]
    fn test_example_2() {
        assert_eq!(max_subarray_sum(&[-2, -4]), Some(-2));
    }
    
    #[test]
    fn test_example_3() {
        assert_eq!(max_subarray_sum(&[5, 4, 1, 7, 8]), Some(25));
    }
    
    #[test]
    fn test_single_element() {
        assert_eq!(max_subarray_sum(&[10]), Some(10));
    }
    
    #[test]
    fn test_empty_array() {
        assert_eq!(max_subarray_sum(&[]), None);
    }
    
    #[test]
    fn test_all_negative() {
        assert_eq!(max_subarray_sum(&[-5, -2, -8, -1]), Some(-1));
    }
}