fn find_missing(arr: &[i32]) -> i32 {
    let n = (arr.len() + 1) as i32;
    
    // Calculate expected sum of 1 to n
    let expected_sum = n * (n + 1) / 2;
    
    // Calculate actual sum of array elements
    let actual_sum: i32 = arr.iter().sum();
    
    // The difference is the missing number
    expected_sum - actual_sum
}

fn main() {
    // Test case 1
    let arr1 = vec![8, 2, 4, 5, 3, 7, 1];
    println!("Input: {:?}", arr1);
    println!("Missing element: {}\n", find_missing(&arr1));
    
    // Test case 2
    let arr2 = vec![1, 2, 3, 5];
    println!("Input: {:?}", arr2);
    println!("Missing element: {}\n", find_missing(&arr2));
    
    // Additional test case
    let arr3 = vec![1, 2, 3, 4, 5, 6, 7, 9, 10];
    println!("Input: {:?}", arr3);
    println!("Missing element: {}", find_missing(&arr3));
}