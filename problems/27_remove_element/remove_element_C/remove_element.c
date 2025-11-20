// +build ignore,cgo

#include <stdio.h>

// Remove Element (LeetCode #27)
//
// Given an integer array nums and an integer val, remove all occurrences of val in nums in-place.
// The order of the elements may be changed. Then return the number of elements in nums which are not equal to val.
//
// Example:
//     $ gcc remove_element.c -o remove_element && ./remove_element
//     removeElement([3, 2, 2, 3], 3) -> k = 2, nums = [2, 2]
//     removeElement([0, 1, 2, 2, 3, 0, 4, 2], 2) -> k = 5, nums = [0, 1, 3, 0, 4]

int removeElement(int* nums, int numsSize, int val) {
    int k = 0; // Pointer for the next position of non-val elements
    for (int i = 0; i < numsSize; i++) {
        if (nums[i] != val) {
            nums[k] = nums[i];
            k++;
        }
    }
    return k;
}

void printArray(int* nums, int size) {
    printf("[");
    for (int i = 0; i < size; i++) {
        printf("%d", nums[i]);
        if (i < size - 1) {
            printf(", ");
        }
    }
    printf("]\n");
}

int main() {
    // Example usage and simple tests
    int nums1[] = {3, 2, 2, 3};
    int numsSize1 = sizeof(nums1) / sizeof(nums1[0]);
    int val1 = 3;
    int k1 = removeElement(nums1, numsSize1, val1);
    printf("removeElement([3, 2, 2, 3], 3) -> k = %d, nums = ", k1);
    printArray(nums1, k1);

    int nums2[] = {0, 1, 2, 2, 3, 0, 4, 2};
    int numsSize2 = sizeof(nums2) / sizeof(nums2[0]);
    int val2 = 2;
    int k2 = removeElement(nums2, numsSize2, val2);
    printf("removeElement([0, 1, 2, 2, 3, 0, 4, 2], 2) -> k = %d, nums = ", k2);
    printArray(nums2, k2);

    return 0;
}

