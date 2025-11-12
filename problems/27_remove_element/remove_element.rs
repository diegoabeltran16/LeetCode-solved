// Remove Element (LeetCode #27)
//
// Given an integer array nums and an integer val, remove all occurrences of val in nums in-place.
// The order of the elements may be changed. Then return the number of elements in nums which are not equal to val.
//
// Example:
//     $ cargo run
//     remove_element(vec![3, 2, 2, 3], 3) -> k = 2, nums = [2, 2]
//     remove_element(vec![0, 1, 2, 2, 3, 0, 4, 2], 2) -> k = 5, nums = [0, 1, 3, 0, 4]
//
fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
    let mut k = 0; // Pointer for the next position of non-val elements
    for i in 0..nums.len() {
        if nums[i] != val {
            nums[k] = nums[i];
            k += 1;
        }
    }
    k as i32
}

fn main() {
    // Example usage and simple tests
    let test_cases = vec![
        (vec![3, 2, 2, 3], 3),
        (vec![0, 1, 2, 2, 3, 0, 4, 2], 2),
    ];

    for (mut nums, val) in test_cases {
        let original_nums = nums.clone();
        let k = remove_element(&mut nums, val);
        println!(
            "remove_element({:?}, {}) -> k = {}, nums = {:?}",
            original_nums,
            val,
            k,
            &nums[..k]
        );
    }
}
