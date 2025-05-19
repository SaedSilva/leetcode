fn main() {
    println!("Hello, world!");
}

struct Solution {}

impl Solution {
    pub fn triangle_type(nums: Vec<i32>) -> String {
        if !Self::valid_triangle(&nums) {
            return "none".to_string();
        }

        if nums[0] == nums[1] && nums[0] == nums[2] {
            return "equilateral".to_string();
        }

        if nums[0] != nums[1] && nums[0] != nums[2] && nums[1] != nums[2] {
            return "scalene".to_string();
        }

        "isosceles".to_string()
    }

    fn valid_triangle(nums: &Vec<i32>) -> bool {
        if nums[0] + nums[1] <= nums[2] {
            return false;
        }
        if nums[0] + nums[2] <= nums[1] {
            return false;
        }
        if nums[1] + nums[2] <= nums[0] {
            return false;
        }
        true
    }
}

#[test]
fn test_triangle_type() {
    assert_eq!(Solution::triangle_type(vec![3, 3, 3]), "equilateral");
    assert_eq!(Solution::triangle_type(vec![3, 4, 5]), "scalene");
    assert_eq!(Solution::triangle_type(vec![3, 3, 4]), "isosceles");
    assert_eq!(Solution::triangle_type(vec![1, 2, 3]), "none");
    assert_eq!(Solution::triangle_type(vec![1, 1, 2]), "none");
    assert_eq!(Solution::triangle_type(vec![2, 7, 7]), "isosceles");
}

/*
You are given a 0-indexed integer array nums of size 3 which can form the sides of a triangle.

    A triangle is called equilateral if it has all sides of equal length.
    A triangle is called isosceles if it has exactly two sides of equal length.
    A triangle is called scalene if all its sides are of different lengths.

Return a string representing the type of triangle that can be formed or "none" if it cannot form a triangle.



Example 1:

Input: nums = [3,3,3]
Output: "equilateral"
Explanation: Since all the sides are of equal length, therefore, it will form an equilateral triangle.

Example 2:

Input: nums = [3,4,5]
Output: "scalene"
Explanation:
nums[0] + nums[1] = 3 + 4 = 7, which is greater than nums[2] = 5.
nums[0] + nums[2] = 3 + 5 = 8, which is greater than nums[1] = 4.
nums[1] + nums[2] = 4 + 5 = 9, which is greater than nums[0] = 3.
Since the sum of the two sides is greater than the third side for all three cases, therefore, it can form a triangle.
As all the sides are of different lengths, it will form a scalene triangle.



Constraints:

    nums.length == 3
    1 <= nums[i] <= 100

*/
