fn main() {
    println!("Hello, world!");
}

pub fn check_zero_ones(s: String) -> bool {
    let mut s_1 = String::from("1");
    let mut s_0 = String::from("0");
    let num_1 = {
        let mut quantity = 0;

        while s.contains(&s_1) {
            println!("s: {}, contains: {}, lenght: {}", s, s_1, s_1.len());
            quantity = s_1.len();
            s_1.push('1');
        }
        println!("Quantity: {}", quantity);

        quantity
    };

    let num_0 = {
        let mut quantity = 0;

        while s.contains(&s_0) {
            println!("s: {}, contains: {}, lenght: {}", s, s_0, s_0.len());
            quantity = s_0.len();
            s_0.push('0');
        }
        
        println!("Quantity: {}", quantity);
        quantity
    };


     num_1 > num_0
}

#[test]
fn test_check_zero_ones() {
    assert_eq!(check_zero_ones("1101".to_string()), true);
    assert_eq!(check_zero_ones("111000".to_string()), false);
    assert_eq!(check_zero_ones("110100010".to_string()), false);
    assert_eq!(check_zero_ones("0111010011".to_string()), true);
}

/*
Given a binary string s, return true if the longest contiguous segment of 1's is strictly longer than the longest contiguous segment of 0's in s, or return false otherwise.

    For example, in s = "110100010" the longest continuous segment of 1s has length 2, and the longest continuous segment of 0s has length 3.

Note that if there are no 0's, then the longest continuous segment of 0's is considered to have a length 0. The same applies if there is no 1's.

Example 1:

Input: s = "1101"
Output: true
Explanation:
The longest contiguous segment of 1s has length 2: "1101"
The longest contiguous segment of 0s has length 1: "1101"
The segment of 1s is longer, so return true.

Example 2:

Input: s = "111000"
Output: false
Explanation:
The longest contiguous segment of 1s has length 3: "111000"
The longest contiguous segment of 0s has length 3: "111000"
The segment of 1s is not longer, so return false.

Example 3:

Input: s = "110100010"
Output: false
Explanation:
The longest contiguous segment of 1s has length 2: "110100010"
The longest contiguous segment of 0s has length 3: "110100010"
The segment of 1s is not longer, so return false.
*/
