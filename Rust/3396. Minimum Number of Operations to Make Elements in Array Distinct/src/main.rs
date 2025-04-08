use std::collections::HashSet;

fn main() {
    let nums = vec![1,2,3,4,2,3,3,5,7];

    let nums2 = vec![4,5,6,4,4];

    let nums3 = vec![6,7,8,9];
    println!("{}", minimum_operations(nums));
    println!("{}", minimum_operations(nums2));
    println!("{}", minimum_operations(nums3));
}

pub fn minimum_operations(nums: Vec<i32>) -> i32 {
    if nums.len() == 0 {
        return 0;
    }
    if is_distinct(nums.clone()) {
        return 0;
    }

    let mut op = 1;
    
    let mut index = if 3 > nums.len() {
        nums.len()
    } else {
        3
    };

    while !is_distinct(nums[index..].to_vec()) {
        op += 1;
        index = if index + 3 > nums.len() {
            nums.len()
        } else {
            index + 3
        }
    }

    op
}

fn is_distinct(nums: Vec<i32>) -> bool {
    let mut sete = HashSet::with_capacity(nums.len());

    for element in nums.iter() {
        if sete.contains(element) {
            return false;
        }
        sete.insert(element);
    }

    true
}
