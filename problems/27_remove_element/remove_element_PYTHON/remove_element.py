#!/usr/bin/env python3
"""
Remove Element (LeetCode #27)

Given an integer array nums and an integer val, remove all occurrences of val in nums in-place.
The order of the elements may be changed. Then return the number of elements in nums which are not equal to val.

Example:
    $ python remove_element.py
    remove_element([3, 2, 2, 3], 3) -> k = 2, nums = [2, 2, _, _]
    remove_element([0, 1, 2, 2, 3, 0, 4, 2], 2) -> k = 5, nums = [0, 1, 4, 0, 3, _, _, _]
"""

def remove_element(nums, val):
    """
    Remove all occurrences of `val` in `nums` in-place.

    Args:
        nums (List[int]): The input array of integers.
        val (int): The value to remove.

    Returns:
        int: The number of elements in `nums` which are not equal to `val`.
    """
    k = 0  # Pointer for the next position of non-val elements
    for i in range(len(nums)):
        if nums[i] != val:
            nums[k] = nums[i]
            k += 1
    return k

if __name__ == "__main__":
    # Example usage and simple tests
    test_cases = [
        ([3, 2, 2, 3], 3),
        ([0, 1, 2, 2, 3, 0, 4, 2], 2),
    ]

    for nums, val in test_cases:
        original_nums = nums[:]
        k = remove_element(nums, val)
        print(f"remove_element({original_nums}, {val}) -> k = {k}, nums = {nums[:k]} + {['_' for _ in range(len(nums) - k)]}")
