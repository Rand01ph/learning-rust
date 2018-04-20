#![allow(unused_variables)]

use std::collections::HashMap;
fn main() {
    let data = "initial contents";

    let s = data.to_string();

// the method also works on a literal directly:
    let s = "initial contents".to_string();

    let mut q = String::from("qqqqq");
    q.push_str("bbbbbb");

    println!("{}", q);

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    println!("{:?}", scores);

    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);

}
