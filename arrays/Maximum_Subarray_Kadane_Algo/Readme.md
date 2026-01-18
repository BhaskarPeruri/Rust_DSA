# Maximum Subarray Sum (Kadane's Algorithm)

## Problem Statement

Given an integer array `arr[]`, find the contiguous subarray (containing at least one element) which has the **maximum sum** and return that sum.

## Approach: Kadane's Algorithm

Kadane's Algorithm is a **dynamic programming** approach that solves this problem in a single pass with O(n) time complexity.

### Core Idea

At each position `i`, we make a simple decision:

> **Should we extend the previous subarray or start fresh from the current element?**

This translates to: `current_sum = max(arr[i], current_sum + arr[i])`

### Algorithm Steps

1. Initialize `max_sum` and `current_sum` with the first element
2. For each subsequent element:
   - **Extend or restart**: Take the maximum of:
     - Current element alone (start new subarray)
     - Current element + previous sum (extend existing subarray)
   - **Update global maximum**: Compare with `max_sum` and update if larger
3. Return `max_sum`

### Why It Works

- If `current_sum` becomes negative, adding it to the next element would only decrease the sum
- Starting fresh with the next element is better than carrying a negative sum
- By tracking the maximum at each step, we ensure we don't miss the optimal subarray

## Complexity Analysis

| Metric               | Value                                          |
| -------------------- | ---------------------------------------------- |
| **Time Complexity**  | O(n) - Single pass through the array           |
| **Space Complexity** | O(1) - Only two variables used                 |

## Examples

### Example 1: Mixed positive and negative

**Input:** `[2, 3, -8, 7, -1, 2, 3]`

| Index | Element | current_sum calculation | current_sum | max_sum |
|-------|---------|------------------------|-------------|---------|
| 0     | 2       | (initial)              | 2           | 2       |
| 1     | 3       | max(3, 2+3) = 5        | 5           | 5       |
| 2     | -8      | max(-8, 5-8) = -3      | -3          | 5       |
| 3     | 7       | max(7, -3+7) = 7       | 7           | 7       |
| 4     | -1      | max(-1, 7-1) = 6       | 6           | 7       |
| 5     | 2       | max(2, 6+2) = 8        | 8           | 8       |
| 6     | 3       | max(3, 8+3) = 11       | 11          | 11      |

**Output:** `11` (subarray: `[7, -1, 2, 3]`)

### Example 2: All negative

**Input:** `[-2, -4]`

**Output:** `-2` (subarray: `[-2]`)

When all elements are negative, the algorithm correctly returns the **least negative** element.

### Example 3: All positive

**Input:** `[5, 4, 1, 7, 8]`

**Output:** `25` (subarray: entire array `[5, 4, 1, 7, 8]`)

When all elements are positive, the maximum subarray is always the entire array.

## Edge Cases Handled

- **Empty array**: Returns `None`
- **Single element**: Returns that element
- **All negative**: Returns the maximum (least negative) element
- **All positive**: Returns sum of entire array

## Usage

```bash
cargo run
```

Run tests:

```bash
cargo test
```

## Code Highlights

The Rust implementation uses:

- `Option<i32>` return type to handle empty arrays gracefully
- Slice (`&[i32]`) for zero-copy array access
- `max()` method for clean comparisons
- Comprehensive test suite with `#[cfg(test)]` module
