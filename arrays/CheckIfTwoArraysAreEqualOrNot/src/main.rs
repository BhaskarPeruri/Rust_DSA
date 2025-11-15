use std::collections::HashMap;

/// Check if two arrays contain the same elements with the same frequencies
fn are_arrays_equal<T: Eq + std::hash::Hash>(a: &[T], b: &[T]) -> bool {
    // If lengths are different, arrays can't be equal
    if a.len() != b.len() {
        return false;
    }
    
    // Count frequency of elements in first array
    let mut freq_map: HashMap<&T, i32> = HashMap::new();
    for item in a {
        *freq_map.entry(item).or_insert(0) += 1;
    }
    
    // Decrement frequencies based on second array
    for item in b {
        match freq_map.get_mut(item) {
            Some(count) => {
                *count -= 1;
                if *count == 0 {
                    freq_map.remove(item);
                }
            }
            None => return false, // Element in b not found in a
        }
    }
    
    // If all frequencies matched, the map should be empty
    freq_map.is_empty()
}

fn main() {
    // Test case 1
    let a1 = vec![1, 2, 5, 4, 0];
    let b1 = vec![2, 4, 5, 0, 1];
    println!("Test 1: {:?} == {:?} ? {}", a1, b1, are_arrays_equal(&a1, &b1));
    
    // Test case 2
    let a2 = vec![1, 2, 5, 4, 0, 2, 1];
    let b2 = vec![2, 4, 5, 0, 1, 1, 2];
    println!("Test 2: {:?} == {:?} ? {}", a2, b2, are_arrays_equal(&a2, &b2));
    
    // Test case 3
    let a3 = vec![1, 7, 1];
    let b3 = vec![7, 7, 1];
    println!("Test 3: {:?} == {:?} ? {}", a3, b3, are_arrays_equal(&a3, &b3));
    
    // Additional test cases
    let a4 = vec![1, 2, 3];
    let b4 = vec![1, 2, 3, 4];
    println!("Test 4: {:?} == {:?} ? {}", a4, b4, are_arrays_equal(&a4, &b4));
    
    let a5: Vec<i32> = vec![];
    let b5: Vec<i32> = vec![];
    println!("Test 5: {:?} == {:?} ? {}", a5, b5, are_arrays_equal(&a5, &b5));
}