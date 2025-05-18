fn main() {
    println!("Hello, world!");
}

pub fn max_repeating(sequence: String, word: String) -> i32 {
    let mut mutable_word = word.clone();
    let mut repeticoes = 0;

    while sequence.contains(&mutable_word) {
        repeticoes += 1;
        mutable_word.push_str(&word);
    }

    repeticoes
}

#[test]
fn test_max_repeating() {
    let result_1 = max_repeating("ababc".to_string(), "ab".to_string());
    let result_2 = max_repeating("ababc".to_string(), "ba".to_string());
    let result_3 = max_repeating("ababc".to_string(), "ac".to_string());
    let result_4 = max_repeating(
        "aaabaaaabaaabaaaabaaaabaaaabaaaaba".to_string(),
        "aaaba".to_string(),
    );

    assert_eq!(2, result_1);
    assert_eq!(1, result_2);
    assert_eq!(0, result_3);
    assert_eq!(5, result_4);
}
