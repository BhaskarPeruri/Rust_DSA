## Explanation:

1. Start from the rightmost element, which is always a leader.

2. Maintain a variable max_from_right to keep track of the maximum value seen so far.

3. Traverse the array from right to left:

   If the current element is greater than or equal to max_from_right, it’s a leader.

   Update max_from_right.

4. Finally, reverse the collected leaders to preserve their original order.
