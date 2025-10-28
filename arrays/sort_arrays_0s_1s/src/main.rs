fn sort_small_integers(arr: &mut Vec<i32>) {
    if arr.is_empty() {
        return;
    }

    let max_val = *arr.iter().max().unwrap() as usize;
    let mut count = vec![0; max_val + 1];

   
    for &num in arr.iter() {
        count[num as usize] += 1;
    }

    let mut idx = 0;
    for (num, &freq) in count.iter().enumerate() {
        for _ in 0..freq {
            arr[idx] = num as i32;
            idx += 1;
        }
    }
}

fn main() {
    let mut arr1 = vec![0, 1, 2, 4,  0, 1, 2, 3, 1, 3, 0, 4];
    sort_small_integers(&mut arr1);
    println!("{:?}", arr1); 
}
