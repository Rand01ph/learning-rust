fn max_ball(v0: i32) -> i32 {
    // your code
    let v = v0 as f32;
    let v2 = (v / 3.6 / 9.81 * 10.0).round() as i32;
    v2
}

fn testequal(v0: i32, exp: i32) -> () {
    assert_eq!(exp, max_ball(v0))
}

#[test]
fn basic_tests() {
    testequal(37, 10);
    testequal(45, 13);
}
