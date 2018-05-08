fn potatoes (p0: i64, w0: i64, p1: i64) -> i64 {
    (100 - p0) * w0 / (100-p1)
}

fn dotest(p0: i64, w0: i64, p1: i64, exp: i64) -> () {
    let ans = potatoes(p0, w0, p1);
    assert_eq!(ans, exp)
}

#[test]
fn tests_potatoes() {
    dotest(99, 100, 98, 50);
    dotest(82, 127, 80, 114);
}
