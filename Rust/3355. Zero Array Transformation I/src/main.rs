fn main() {
    // Solution::is_zero_array(vec![1, 0, 1], vec![vec![0, 2]]);
    Solution::is_zero_array(vec![4, 3, 2, 1], vec![vec![1, 3], vec![0, 2]]);
}

struct Solution;

impl Solution {
    pub fn is_zero_array(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> bool {
        let mut delta: Vec<i32> = nums.iter().map(|_| 0).collect();
        delta.push(0);

        for i in 0..queries.len() {
            let query = &queries[i];
            let start = query[0] as usize;
            let end = query[1] as usize;
            
            delta[start] += 1;
            delta[end + 1] -= 1;
        }
        
        for i in 0..delta.len() {
            if i == 0 {
                continue
            }
            delta[i] += delta[i - 1];
        }
        
        for i in 0..nums.len() {
            if nums[i] > delta[i] {
                return false
            }
        }
        
        true
    }
}

#[test]
fn test_is_zero_array() {
    assert_eq!(
        Solution::is_zero_array(vec![1, 0, 1], vec![vec![0, 2]]),
        true
    );
    assert_eq!(
        Solution::is_zero_array(vec![4, 3, 2, 1], vec![vec![1, 3], vec![0, 2]]),
        false
    );
    assert_eq!(
        Solution::is_zero_array(vec![1, 2, 1, 0, 0, 0], vec![vec![0, 3], vec![2, 4]]),
        false
    );
    assert_eq!(
        Solution::is_zero_array(vec![2], vec![vec![0, 0], vec![0, 0], vec![0, 0]]),
        true
    );
}
