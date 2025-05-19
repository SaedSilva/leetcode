fn main() {
    println!("Hello, world!");
}

struct Solution {}

impl Solution {
    pub fn minimum_operations(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        Self::apply(&mut nums, 0)
    }

    fn apply(nums: &mut Vec<i32>, operations: i32) -> i32 {
        if nums.len() == 0 {
            return operations - 1;
        }
        nums.retain(|&x| x != 0);
        let min: i32 = {
            let mut min = 0;
            for num in nums.iter() {
                if min == 0 || num < &min {
                    min = *num;
                }
            }
            min
        };

        for x in nums.iter_mut() {
            *x -= min;
        }
        
        Self::apply(nums, operations + 1)
    }
}

#[test]
fn test_minimum_operations() {
    let nums = vec![0, 1, 2, 3, 4];
    let result = Solution::minimum_operations(nums);
    assert_eq!(result, 4);

    let nums = vec![1, 1, 1, 1];
    let result = Solution::minimum_operations(nums);
    assert_eq!(result, 1);

    let nums = vec![0, 0, 0];
    let result = Solution::minimum_operations(nums);
    assert_eq!(result, 0);

    let nums = vec![1, 5, 0, 3, 5];
    let result = Solution::minimum_operations(nums);
    assert_eq!(result, 3);

    let nums = vec![0];
    let result = Solution::minimum_operations(nums);
    assert_eq!(result, 0);
}

/*
You are given a non-negative integer array nums. In one operation, you must:

    Choose a positive integer x such that x is less than or equal to the smallest non-zero element in nums.
    Subtract x from every positive element in nums.

Return the minimum number of operations to make every element in nums equal to 0.



Example 1:

Input: nums = [1,5,0,3,5]
Output: 3
Explanation:
In the first operation, choose x = 1. Now, nums = [0,4,0,2,4].
In the second operation, choose x = 2. Now, nums = [0,2,0,0,2].
In the third operation, choose x = 2. Now, nums = [0,0,0,0,0].

Example 2:

Input: nums = [0]
Output: 0
Explanation: Each element in nums is already 0 so no operations are needed.



Constraints:

    1 <= nums.length <= 100
    0 <= nums[i] <= 100
*/
