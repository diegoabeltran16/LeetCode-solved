using System;

public class Problem {
    public static void Main(string[] args) {
        Solution solution = new Solution();

        int[] nums1 = {3, 2, 2, 3};
        int val1 = 3;
        int k1 = solution.RemoveElement(nums1, val1);
        Console.WriteLine($"removeElement([3, 2, 2, 3], 3) -> k = {k1}, nums = [{string.Join(", ", nums1[..k1])}]");

        int[] nums2 = {0, 1, 2, 2, 3, 0, 4, 2};
        int val2 = 2;
        int k2 = solution.RemoveElement(nums2, val2);
        Console.WriteLine($"removeElement([0, 1, 2, 2, 3, 0, 4, 2], 2) -> k = {k2}, nums = [{string.Join(", ", nums2[..k2])}]");
    }
}