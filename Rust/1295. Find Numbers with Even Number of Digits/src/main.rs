fn main() {
    let nums = vec![12,345,2,6,7896];
    println!("{}", find_numbers(nums));
}

pub fn find_numbers(nums: Vec<i32>) -> i32 {
    let mut count_even = 0;

    for num in nums.into_iter() {
        let num = num.to_string();
        if num.len() % 2 == 0 {
            count_even += 1;
        } 
    }


    count_even
}
