use std::io;

fn zig_zag(arr: &mut Vec<i32>) {
    let n = arr.len();
    let mut flag = true; // true means "<" relation expected, false means ">" relation expected

    for i in 0..n - 1 {
        if flag {
            // Expect arr[i] < arr[i+1]
            if arr[i] > arr[i + 1] {
                arr.swap(i, i + 1);
            }
        } else {
            // Expect arr[i] > arr[i+1]
            if arr[i] < arr[i + 1] {
                arr.swap(i, i + 1);
            }
        }
        flag = !flag; // flip expectation
    }
}

fn main() {
    let mut input = String::new();

    println!("Enter array elements separated by spaces:");
    io::stdin().read_line(&mut input).unwrap();

    let mut arr: Vec<i32> = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse::<i32>().unwrap())
        .collect();

    zig_zag(&mut arr);

    println!("Zig-zag arrangement: {:?}", arr);
}
