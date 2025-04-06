fn main() {
    let mut arr1 = vec![0];
    let mut arr2 = vec![1];
    let m = 0;
    let n = 1;
    merge(&mut arr1, m, &mut arr2, n);
    println!("{:?}", arr1);
}

pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
    let mut m: usize = m as usize;
    for element in nums2.into_iter() {
        nums1[m] = *element;
        m += 1;
    }
    nums1.sort();
}