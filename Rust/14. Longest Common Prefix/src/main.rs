fn main() {
    let a: Vec<String> = vec![
        "flower".into(),
        "flow".into(),
        "flight".into()
    ];
    println!("{}", longest_common_prefix(a));
}

pub fn longest_common_prefix(strs: Vec<String>) -> String {
    if strs.len() == 1 {
        return strs.first().unwrap().clone();
    }

    let mut strs = strs.clone();
    strs.sort();
    let first = strs.first().unwrap();
    let last = strs.last().unwrap();

    let min = first.capacity().min(last.capacity());

    let mut result = String::with_capacity(min);
    let first: Vec<char> = first.chars().collect(); 
    let last: Vec<char> = last.chars().collect(); 

    for i in 0..min {
        if first.get(i) != last.get(i) {
            return result;
        }
        result.push(first.get(i).unwrap().clone());
    }

    result
}
