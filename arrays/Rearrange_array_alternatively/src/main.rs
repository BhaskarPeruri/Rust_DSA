use std::io;

fn rearrange_max_min(arr: &mut Vec<i32>) {
    let mut result = Vec::with_capacity(arr.len());
    let (mut left, mut right) = (0, arr.len() - 1);

    // Alternate between max (right) and min (left)
    while left <= right {
        if left != right {
            result.push(arr[right]);
            result.push(arr[left]);
        } else {
            // For odd-length arrays, add the middle element once
            result.push(arr[left]);
        }
        left += 1;
        // Prevent underflow for right
        if right == 0 { break; }
        right -= 1;
    }

    // Copy rearranged elements back into original array
    arr.clone_from(&result);
}

fn main() {
    let mut input = String::new();
    println!("Enter a sorted array of positive integers (space-separated):");
    io::stdin().read_line(&mut input).unwrap();

    let mut arr: Vec<i32> = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse::<i32>().unwrap())
        .collect();

    rearrange_max_min(&mut arr);
    println!("Rearranged array in max-min form: {:?}", arr);
}
