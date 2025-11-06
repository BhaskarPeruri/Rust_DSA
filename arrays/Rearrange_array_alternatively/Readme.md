## Explanation of Logic

1. The array is already sorted (ascending).

2. Two pointers are used:

   - left starts at the beginning (minimum side)
   - right starts at the end (maximum side)

3. In each step:

   - Push arr[right] (max)
   - Push arr[left] (min)
   - Move both pointers inward

4. For odd-length arrays, when both pointers meet, we push the middle element once.
