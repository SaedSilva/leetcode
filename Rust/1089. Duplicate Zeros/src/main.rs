fn main() {
    let mut array_1 = vec![1, 0, 2, 3, 0, 4, 5, 0];
    duplicate_zeros(&mut array_1);
    println!("{:?}", array_1)
}

pub fn duplicate_zeros(arr: &mut Vec<i32>) {
    let mut i = 0;
    while i < arr.len() - 1 {
        if arr[i] == 0 {
            let mut j = arr.len() - 2;
            while j > i {
                arr[j + 1] = arr[j];
                j -= 1;
            }
            arr[i + 1] = arr[i];
            i = i + 1;
        
        }
        i += 1;
    }
}
