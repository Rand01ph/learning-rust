fn longest_consec(strarr: Vec<&str>, k: usize) -> String {
    let mut m_s = String::new();
    if k == 0 {
        return m_s;
    } else if k > strarr.len() {
        return m_s;
    } else if strarr.len() == 0 {
        return m_s;
    } else {
        for i in 0..strarr.len()-k+1 {
            if strarr[i..i+k].concat().len() > m_s.len() {
                m_s = strarr[i..i+k].concat();
            }
        }
        return m_s
    }
}

fn testing(strarr: Vec<&str>, k: usize, exp: &str) -> () {
    assert_eq!(&longest_consec(strarr, k), exp)
}

#[test]
fn basics_longest_consec() {
    testing(vec!["zone", "abigail", "theta", "form", "libe", "zas"], 2, "abigailtheta");
    testing(vec!["ejjjjmmtthh", "zxxuueeg", "aanlljrrrxx", "dqqqaaabbb", "oocccffuucccjjjkkkjyyyeehh"], 1,
            "oocccffuucccjjjkkkjyyyeehh");
    testing(vec![], 3, "");
    testing(vec!["it", "wkppv", "ixoyx", "3452", "zzzzzzzzzzzz"], 3, "ixoyx3452zzzzzzzzzzzz");
    testing(vec!["it", "wkppv", "ixoyx", "3452", "zzzzzzzzzzzz"], 15, "");
    testing(vec!["it", "wkppv", "ixoyx", "3452", "zzzzzzzzzzzz"], 0, "");
}
