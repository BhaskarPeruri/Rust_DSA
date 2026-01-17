# Zig-Zag Array Arrangement

## Problem Statement

Given an array of distinct elements, rearrange the elements in a **zig-zag** fashion such that:

```
arr[0] < arr[1] > arr[2] < arr[3] > arr[4] < ...
```

The converted array should be in the form: **a < b > c < d > e < f**

## Approach

This solution uses a **single-pass greedy algorithm** with O(n) time complexity and O(1) space complexity.

### Algorithm

1. Use a boolean `flag` to track the expected relationship:

   - `flag = true` → expect `arr[i] < arr[i+1]`
   - `flag = false` → expect `arr[i] > arr[i+1]`

2. Iterate through the array from index `0` to `n-2`:

   - If the current relationship doesn't match the expected one, **swap** the adjacent elements
   - Flip the flag after each iteration

3. The key insight is that swapping two adjacent elements to fix one relationship **does not break the previous relationship** (since the previous element was already correctly placed).

### Why It Works

When we swap `arr[i]` and `arr[i+1]`:

- If we needed `arr[i] < arr[i+1]` but had `arr[i] > arr[i+1]`, after swapping, the smaller element moves to position `i`
- This doesn't affect position `i-1` because:
  - At `i-1`, we needed `arr[i-1] > arr[i]` (opposite relation)
  - After swap, `arr[i]` becomes smaller, so `arr[i-1] > arr[i]` still holds

## Complexity Analysis

| Metric               | Value                                |
| -------------------- | ------------------------------------ |
| **Time Complexity**  | O(n) - Single pass through the array |
| **Space Complexity** | O(1) - Only uses a boolean flag      |

## Example

**Input:** `[4, 3, 7, 8, 6, 2, 1]`

**Process:**

- i=0: flag=true, expect 4<3? No (4>3), swap → `[3, 4, 7, 8, 6, 2, 1]`
- i=1: flag=false, expect 4>7? No (4<7), swap → `[3, 7, 4, 8, 6, 2, 1]`
- i=2: flag=true, expect 4<8? Yes, no swap
- i=3: flag=false, expect 8>6? Yes, no swap
- i=4: flag=true, expect 6<2? No (6>2), swap → `[3, 7, 4, 8, 2, 6, 1]`
- i=5: flag=false, expect 6>1? Yes, no swap

**Output:** `[3, 7, 4, 8, 2, 6, 1]`

Verification: `3 < 7 > 4 < 8 > 2 < 6 > 1` ✓

## Usage

```bash
cargo run
```

Enter the array elements separated by spaces when prompted.
