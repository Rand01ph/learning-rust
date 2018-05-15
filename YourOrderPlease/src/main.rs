fn order(sentence: &str) -> String {
    let mut words: Vec<&str> = sentence.split_whitespace().collect();
//    let order: Vec<i32> = sentence.matches(char::is_numeric).map(|s| s.parse().unwrap()).collect();
    words.sort_by(|a, b| a.matches(char::is_numeric).last().unwrap().parse::<i32>().unwrap().cmp(&b.matches(char::is_numeric).last().unwrap().parse::<i32>().unwrap()));
    words.join(" ")
}


#[test]
fn returns_expected() {
    assert_eq!(order("is2 Thi1s T4est 3a"), "Thi1s is2 3a T4est");
    assert_eq!(order(""), "");
}
