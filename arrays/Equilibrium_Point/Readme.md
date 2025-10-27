# Equilibrium Point Finder

This Rust program implements a solution to find an equilibrium point in an array. An equilibrium point is a position in the array where the sum of elements to its left equals the sum of elements to its right.

## Problem Description

Given an array of integers, find an index (equilibrium point) where:

- The sum of all elements to the left of the index equals
- The sum of all elements to the right of the index

If no such index exists, return -1.

## Implementation Details

The solution is implemented in the `find_partition` function with the following approach:

```rust
fn find_partition(arr: &[i32]) -> i32
```

### Algorithm

1. First checks if array has at least 3 elements (requirement for valid partition)
2. Calculates total sum of array
3. Iterates through positions 1 to n-2 (excluding first and last elements)
4. For each position:
   - Maintains running sum of left elements
   - Calculates right sum by subtracting left sum and current element from total
   - If left sum equals right sum, returns current position
5. Returns -1 if no equilibrium point is found

## Example Usage

The program includes several test cases demonstrating different scenarios:

```rust
let arr1 = vec![1, 4, 2, 5, 0];     // Has equilibrium point
let arr2 = vec![1, 3, 10, 1, 9, 3, 2]; // Has equilibrium point
let arr3 = vec![4, 2, 2, 2];        // No equilibrium point
let arr4 = vec![10, 11, 11, 18, 3]; // No equilibrium point
```

## Running the Program

To run this program:

1. Ensure you have Rust installed
2. Navigate to the project directory
3. Run `cargo build` to compile
4. Run `cargo run` to execute
