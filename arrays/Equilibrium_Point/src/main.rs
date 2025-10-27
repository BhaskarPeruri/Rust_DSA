fn find_partition(arr: &[i32]) -> i32 {
    let n = arr.len();
    
    // Need at least 3 elements for valid partition
    if n < 3 {
        return -1;
    }
    
    // Calculate total sum
    let total_sum: i32 = arr.iter().sum();
    
    // Track left sum as we iterate
    let mut left_sum = 0;
    
    // Check each element from index 1 to n-2 (excluding first and last)
    for i in 1..n-1 {
        left_sum += arr[i - 1];
        let right_sum = total_sum - left_sum - arr[i];
        
        if left_sum == right_sum {
            return i.try_into().unwrap();
        }
    }
    
    -1
}

fn main() {
    
    let arr1 = vec![1, 4, 2, 5, 0];
    println!("Input: {:?}", arr1);
    println!("Output (Position is): {}", find_partition(&arr1));
    println!();
    
    
    let arr2 = vec![1, 3, 10, 1, 9, 3, 2];
    println!("Input: {:?}", arr2);
    println!("Output (Position is): {}", find_partition(&arr2));
    println!();
    
    
    let arr3 = vec![4, 2, 2,2];
    println!("Input: {:?}", arr3);
    println!("Output (Position is): {}", find_partition(&arr3));
    println!();
    
    
    let arr4 = vec![10, 11, 11, 18, 3];
    println!("Input: {:?}", arr4);
    println!("Output (Position is): {}", find_partition(&arr4));
}