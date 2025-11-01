## Explanation:

The function takes a mutable vector reference &mut Vec<i32> so it can modify it in place.

It loops through the array in steps of k.

For each segment [i..end], it reverses the slice directly using Rust’s built-in .reverse() method.

If the remaining elements are fewer than k, usize::min(i + k, n) ensures we reverse whatever remains.
