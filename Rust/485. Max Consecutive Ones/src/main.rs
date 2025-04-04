fn main() {
    println!("Hello, world!");

}

pub fn find_max_consecutive_ones(nums: Vec<i32>) -> i32 {
    let mut max = 0; // Maior sequência
    let mut count = 0;
    for num in nums.into_iter() {
        if num == 1 {
            count += 1;
        }
        if num != 1 && count > 0 {
            if count > max {
                max = count;
            }
            count = 0;
        }
    }
    if count > max {
        max = count;
    }

    max
}
