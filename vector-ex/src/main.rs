use std::collections::HashMap;

fn main() {
    let mut v = vec![1, 2, 3, 4, 4, 2, 7, 3, 9, 8, 1, 2];
    v.sort();
    let mut sum = 0;
    let mut num_map = HashMap::new();
    let mid_num = v.len() / 2;
    println!("mid is {} mid num is {}", mid_num, v[mid_num]);
    for num in v {
        sum += num;
        let count = num_map.entry(num).or_insert(0);
        *count += 1;
    }
    println!("sum is {}", sum);
    let mut big_index = 0;
    let mut big_number = 0;
    for (key, val) in num_map.iter() {
        if *val > big_index {
            big_index = *val;
            big_number = *key;
        }
    }
    println!("big number is {}", big_number)
}
