use std::collections::HashSet;

fn main() {
    println!("{:?}", contains_duplicate(vec![1,2,3,4]));
}

pub fn contains_duplicate(nums: Vec<i32>) -> bool {
    let mut sete = HashSet::new();
    for element in nums.iter() {
        sete.insert(element);
    }
    return sete.len() != nums.len();
}
