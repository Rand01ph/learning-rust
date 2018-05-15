extern crate stopwatch;

use stopwatch::Stopwatch;

fn list_squared(m: u64, n: u64) -> Vec<(u64, u64)> {
    let sw = Stopwatch::start_new();
    let mut final_vec = Vec::new();
    for i in m..n + 1 {
        let mut sum = 0;
        for x in 1..i + 1 {
            if i % x == 0 {
                sum += x * x;
            }
        }
        let b = (sum as f64).sqrt();
        if b.fract() == 0.0_f64 {
            final_vec.push((i, sum));
        }
    }
    println!("total took {}ms", sw.elapsed_ms());
    final_vec
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
    testing(700, 1500, vec![(728, 722500), (1434, 2856100)]);
    testing(4000, 4300, vec![(4264, 24304900)]);
}