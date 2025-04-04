
fn main() {
    let nums = vec![-4,-1,0,3,10];
    let nums2 = vec![-7,-3,2,3,11];
    println!("{:?}", sorted_squares(nums));
    println!("{:?}", sorted_squares(nums2));
}

pub fn sorted_squares(nums: Vec<i32>) -> Vec<i32> {
    let mut result: Vec<i32> = nums.iter().map(|x| {
        x * x
    }).collect();
    
    result.sort();

    result
}
