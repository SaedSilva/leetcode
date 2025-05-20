fn main() {
    println!("Hello, world!");
}

struct Solution;

impl Solution {
    pub fn get_longest_subsequence(words: Vec<String>, groups: Vec<i32>) -> Vec<String> {
        if words.is_empty() {
            return words;
        }

        let mut indexs = vec![];
        indexs.push(0);

        for i in 1..groups.len() {
            if groups[i] != groups[i - 1] {
                indexs.push(i);
            }
        }

        indexs
            .into_iter()
            .map(|index| {
                words[index].clone()
            })
            .collect()
    }
}

#[test]
fn test_get_longest_subsequence() {
    let words = vec!["e".to_string(), "a".to_string(), "b".to_string()];
    let groups = vec![0, 0, 1];
    let result = Solution::get_longest_subsequence(words, groups);
    assert_eq!(result, vec!["e".to_string(), "b".to_string()]);

    let words = vec![
        "a".to_string(),
        "b".to_string(),
        "c".to_string(),
        "d".to_string(),
    ];
    let groups = vec![1, 0, 1, 1];
    let result = Solution::get_longest_subsequence(words, groups);
    assert_eq!(
        result,
        vec!["a".to_string(), "b".to_string(), "c".to_string()]
    );
}

/*
You are given a string array words and a binary array groups both of length n.

A

of words is alternating if for any two consecutive strings in the sequence, their corresponding elements at the same indices in groups are different (that is, there cannot be consecutive 0 or 1).

Your task is to select the longest alternating subsequence from words.

Return the selected subsequence. If there are multiple answers, return any of them.

Note: The elements in words are distinct.



Example 1:

Input: words = ["e","a","b"], groups = [0,0,1]

Output: ["e","b"]

Explanation: A subsequence that can be selected is ["e","b"] because groups[0] != groups[2]. Another subsequence that can be selected is ["a","b"] because groups[1] != groups[2]. It can be demonstrated that the length of the longest subsequence of indices that satisfies the condition is 2.

Example 2:

Input: words = ["a","b","c","d"], groups = [1,0,1,1]

Output: ["a","b","c"]

Explanation: A subsequence that can be selected is ["a","b","c"] because groups[0] != groups[1] and groups[1] != groups[2]. Another subsequence that can be selected is ["a","b","d"] because groups[0] != groups[1] and groups[1] != groups[3]. It can be shown that the length of the longest subsequence of indices that satisfies the condition is 3.
*/
