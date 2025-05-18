use std::ops::BitXor;

fn main() {
    println!("Hello, world!");
}

pub fn hamming_distance(x: i32, y: i32) -> i32 {
    let z = x.bitxor(y);
    
    z.count_ones() as i32
}

#[test]
fn test_hamming_distance() {
    assert_eq!(hamming_distance(1, 4), 2);
    assert_eq!(hamming_distance(3, 1), 1);
    assert_eq!(hamming_distance(0, 0), 0);
    assert_eq!(hamming_distance(1, 1), 0);
    assert_eq!(hamming_distance(2, 3), 1);
}

/*
The Hamming distance between two integers is the number of positions at which the corresponding bits are different.

Given two integers x and y, return the Hamming distance between them.

 

Example 1:

Input: x = 1, y = 4
Output: 2
Explanation:
1   (0 0 0 1)
4   (0 1 0 0)
       ↑   ↑
The above arrows point to positions where the corresponding bits are different.

Example 2:

Input: x = 3, y = 1
Output: 1

 

Constraints:

    0 <= x, y <= 231 - 1
*/
