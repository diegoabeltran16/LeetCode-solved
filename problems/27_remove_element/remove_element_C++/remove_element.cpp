#include <iostream>
#include <vector>
using namespace std;

// Remove Element (LeetCode #27)
//
// Given an integer array nums and an integer val, remove all occurrences of val in nums in-place.
// The order of the elements may be changed. Then return the number of elements in nums which are not equal to val.
//
// Example:
//     $ g++ remove_element.cpp -o remove_element && ./remove_element
//     removeElement([3, 2, 2, 3], 3) -> k = 2, nums = [2, 2]
//     removeElement([0, 1, 2, 2, 3, 0, 4, 2], 2) -> k = 5, nums = [0, 1, 3, 0, 4]

class Solution {
public:
    int removeElement(vector<int>& nums, int val) {
        int n = nums.size();
        int j = 0;

        for (int i = 0; i < n; i++) {
            if (nums[i] != val) {
                nums[j] = nums[i];
                j++;
            }
        }
        return j;
    }
};

void printVector(const vector<int>& nums, int size) {
    cout << "[";
    for (int i = 0; i < size; i++) {
        cout << nums[i];
        if (i < size - 1) {
            cout << ", ";
        }
    }
    cout << "]" << endl;
}

int main() {
    Solution solution;

    // Example usage and simple tests
    vector<int> nums1 = {3, 2, 2, 3};
    int val1 = 3;
    int k1 = solution.removeElement(nums1, val1);
    cout << "removeElement([3, 2, 2, 3], 3) -> k = " << k1 << ", nums = ";
    printVector(nums1, k1);

    vector<int> nums2 = {0, 1, 2, 2, 3, 0, 4, 2};
    int val2 = 2;
    int k2 = solution.removeElement(nums2, val2);
    cout << "removeElement([0, 1, 2, 2, 3, 0, 4, 2], 2) -> k = " << k2 << ", nums = ";
    printVector(nums2, k2);

    return 0;
}