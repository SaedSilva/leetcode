fn main() {
    println!("Hello, world!");
}

pub fn find_words_containing(words: Vec<String>, x: char) -> Vec<i32> {
    let mut result = vec![];
    for (index, word) in words.iter().enumerate() {
        for char in word.chars() {
            if char == x {
                result.push(index as i32);
                break;
            }
        }
    }
    
    result
}

#[test]
fn test_find_words_containing() {
    let result_1 = find_words_containing(vec!["leet".to_string(), "code".to_string()], 'e');
    let result_2 = find_words_containing(
        vec![
            "abc".to_string(),
            "bcd".to_string(),
            "aaaa".to_string(),
            "cbc".to_string(),
        ],
        'a',
    );
    let result_3 = find_words_containing(
        vec![
            "abc".to_string(),
            "bcd".to_string(),
            "aaaa".to_string(),
            "cbc".to_string(),
        ],
        'z',
    );

    assert_eq!(vec![0, 1], result_1);
    assert_eq!(vec![0, 2], result_2);
    assert_eq!(Vec::<i32>::new(), result_3);
}
