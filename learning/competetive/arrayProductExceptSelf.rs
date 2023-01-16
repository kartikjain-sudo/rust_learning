// Given an integer array nums, return an array answer such that answer[i] is equal to the product of all the elements of nums except nums[i].

// The product of any prefix or suffix of nums is guaranteed to fit in a 32-bit integer.

// You must write an algorithm that runs in O(n) time and without using the division operation.

 

// Example 1:

// Input: nums = [1,2,3,4]
// Output: [24,12,8,6]
// Example 2:

// Input: nums = [-1,1,0,-3,3]
// Output: [0,0,9,0,0]
 

// Constraints:

// 2 <= nums.length <= 105
// -30 <= nums[i] <= 30
// The product of any prefix or suffix of nums is guaranteed to fit in a 32-bit integer.
 

// Follow up: Can you solve the problem in O(1) extra space complexity? (The output array does not count as extra space for space complexity analysis.)

use std::vec;

pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
    let mut pro = 1;
    let mut zeroCount = 0;
    let mut pro_without_0 = 1;
    for item in &nums {
        pro = pro*item;
        if *item == 0 {
            zeroCount += 1;
        }
        if zeroCount < 2 {
            if *item == 0 {
                continue;
            } else {
                pro_without_0 = pro_without_0*item
            }
        }
    }

    let mut res: Vec<i32> = Vec::new();
    for (i,item) in nums.iter().enumerate() {
        if nums[i] == 0 {
            if zeroCount < 2 {
                res.push(pro_without_0);
            } else {
                res.push(0);
            }
        } else {
            res.push(pro/nums[i]);
        }
    }

    return res;
}