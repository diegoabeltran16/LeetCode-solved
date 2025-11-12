using System;
using System.Collections.Generic;

public class Solution {
    public int RemoveElement(int[] nums, int val) {
        List<int> arr = new List<int>();
        for (int i = 0; i < nums.Length; i++) {
            if (nums[i] != val) {
                arr.Add(nums[i]);
            }
        }
        foreach (var n in arr) {
            Console.WriteLine(n);
        }
        for (int j = 0; j < arr.Count; j++) {
            nums[j] = arr[j];
        }
        return arr.Count;
    }
}