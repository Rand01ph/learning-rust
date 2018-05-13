fn list_squared(m: u64, n: u64) -> Vec<(u64, u64)> {
    let mut aaa = Vec::new();
    for i in m..n+1 {
        let mut factors = (1..i+1).into_iter().filter(|&x| i % x == 0).collect::<Vec<u64>>();
        let mut sum = factors.iter().map(|&x| x * x).sum();
        let b = (sum as f64).sqrt() as u64;
        if b * b == sum {
            aaa.push((i, sum));
        }
    }
    return aaa
}


fn testing(m: u64, n: u64, exp: Vec<(u64, u64)>) -> () {
    assert_eq!(list_squared(m, n), exp)
}

#[test]
fn basics_list_squared() {

    testing(1, 250, vec![(1, 1), (42, 2500), (246, 84100)]);
    testing(1, 250, vec![(1, 1), (42, 2500), (246, 84100)]);
    testing(42, 250, vec![(42, 2500), (246, 84100)]);
    testing(300, 600, vec![]);

}
