fn main() {
    let s = "anagram";
    let t = "nagaram";

    println!("{}", is_valid_anagram(s, t));
}

fn is_valid_anagram(s: &str, t: &str) -> bool {
    if s.len() != t.len() {
        return false;
    }
    let mut s: Vec<char> = s.chars().collect();
    let mut t: Vec<char> = t.chars().collect();
    s.sort();
    t.sort();
    if s != t {
        return false;
    }

    true
}
