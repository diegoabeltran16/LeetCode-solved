import java.util.Arrays;

// Remove Element (LeetCode #27)
//
// Given an integer array nums and an integer val, remove all occurrences of val in nums in-place.
// The order of the elements may be changed. Then return the number of elements in nums which are not equal to val.
//
// Example:
//     $ javac RemoveElement.java && java RemoveElement
//     removeElement([3, 2, 2, 3], 3) -> k = 2, nums = [2, 2]
//     removeElement([0, 1, 2, 2, 3, 0, 4, 2], 2) -> k = 5, nums = [0, 1, 3, 0, 4]

public class RemoveElement {
    public int removeElement(int[] nums, int val) {
        int k = 0; // Pointer for the next position of non-val elements
        for (int i = 0; i < nums.length; i++) {
            if (nums[i] != val) {
                nums[k] = nums[i];
                k++;
            }
        }
        return k;
    }

    public static void main(String[] args) {
        RemoveElement solution = new RemoveElement();

        // Example usage and simple tests
        int[] nums1 = {3, 2, 2, 3};
        int val1 = 3;
        int k1 = solution.removeElement(nums1, val1);
        System.out.println("removeElement([3, 2, 2, 3], 3) -> k = " + k1 + ", nums = " + Arrays.toString(Arrays.copyOf(nums1, k1)));

        int[] nums2 = {0, 1, 2, 2, 3, 0, 4, 2};
        int val2 = 2;
        int k2 = solution.removeElement(nums2, val2);
        System.out.println("removeElement([0, 1, 2, 2, 3, 0, 4, 2], 2) -> k = " + k2 + ", nums = " + Arrays.toString(Arrays.copyOf(nums2, k2)));
    }
}