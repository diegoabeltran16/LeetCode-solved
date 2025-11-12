#!/usr/bin/env node
/**
 * Remove Element (LeetCode #27)
 *
 * Given an integer array nums and an integer val, remove all occurrences of val in nums in-place.
 * The order of the elements may be changed. Then return the number of elements in nums which are not equal to val.
 *
 * Example:
 *     $ node remove_element.ts
 *     removeElement([3, 2, 2, 3], 3) -> k = 2, nums = [2, 2]
 *     removeElement([0, 1, 2, 2, 3, 0, 4, 2], 2) -> k = 5, nums = [0, 1, 3, 0, 4]
 */

/**
 * @param {number[]} nums
 * @param {number} val
 * @return {number}
 */
export function removeElement(nums: number[], val: number): number {
    let k = 0; // Pointer for the next position of non-val elements
    for (let i = 0; i < nums.length; i++) {
        if (nums[i] !== val) {
            nums[k] = nums[i];
            k++;
        }
    }
    return k;
}

if (require.main === module) {
    // Example usage and simple tests
    const testCases = [
        { nums: [3, 2, 2, 3], val: 3 },
        { nums: [0, 1, 2, 2, 3, 0, 4, 2], val: 2 },
    ];

    for (const { nums, val } of testCases) {
        const originalNums = [...nums];
        const k = removeElement(nums, val);
        console.log(
            `removeElement(${JSON.stringify(originalNums)}, ${val}) -> k = ${k}, nums = ${JSON.stringify(nums.slice(0, k))}`
        );
    }
}