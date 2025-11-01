use std::io;
fn reverse_in_groups(arr: &mut Vec<i32>, k: usize){
    let n = arr.len();
    let mut i = 0;

    while i < n {
        let end = usize::min(i +k, n); //to reduce panic
        arr[i..end].reverse();
        i+=k;
    }
}
fn main() {
    let mut input = String::new();
    println!("Enter array elements separated by spaces: ");
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let mut arr: Vec<i32> = input.trim().split_whitespace().map(|s| s.parse().expect("Please enter valid integers")).collect();
    input.clear();
    println!("Enter the value of k: ");
    io::stdin().read_line(&mut input).expect("Failed to read k");

    let k: usize = input.trim().parse().expect("Please enter a valid integer for k");
    reverse_in_groups(& mut arr, k);
    println!("Array after reversing every group of {} elements", k);
    println!("{:?}", arr);
}