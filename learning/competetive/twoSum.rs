/**
Given an array of integers nums and an integer target, return indices of the two numbers such that they add up to target.

You may assume that each input would have exactly one solution, and you may not use the same element twice.

You can return the answer in any order.

Example 1:

Input: nums = [2,7,11,15], target = 9
Output: [0,1]
Explanation: Because nums[0] + nums[1] == 9, we return [0, 1].
Example 2:

Input: nums = [3,2,4], target = 6
Output: [1,2]
Example 3:

Input: nums = [3,3], target = 6
Output: [0,1]
 

Constraints:

2 <= nums.length <= 104
-109 <= nums[i] <= 109
-109 <= target <= 109
Only one valid answer exists.

 */

 /**
  * help: you can convert a `usize` to an `i32` and panic if the converted value doesn't fit
  * |
  * |                 hm.insert(target - &num, i.try_into().unwrap());
  */

 use std::collections::HashMap;

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {

    let mut hm/* : HashMap<i32, usize>*/ = HashMap::with_capacity(nums.len());
    for (i, num) in nums.iter().enumerate() {

        match hm.get(num) {
            Some(&j) => return vec![i as i32, j as i32],
            None => {
                hm.insert(target - num, i);
            },
        }
    }
    panic!("Not found"); 
    // unreachable!();
}


fn main() {
    let res:Vec<i32> = two_sum(vec![1,2,7,9,4,3,2,4], 6);
    println!("{:?}", res);
}