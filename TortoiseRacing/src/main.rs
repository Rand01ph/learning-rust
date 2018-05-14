fn race(v1: i32, v2: i32, g: i32) -> Option<Vec<i32>> {
    if v2>v1 {
        let t = g as f64 * 3600.00 / (v2 as f64 - v1 as f64);
        let mut total_s = t as i32;
        if (t.fract() - 0.5) > 1e-10{
            total_s = total_s +1;
        }
        let h:i32 = total_s / 3600;
        let mut m = 0;
        let mut s = 0;
        if total_s % 3600 != 0 {
            m = total_s % 3600 / 60;
            s = total_s - (h*3600) - (m * 60);
        }
        return Some(vec![h, m, s])
    }
    None
}

#[test]
fn basic_tests() {
  assert_eq!(race(720, 850, 70), Some(vec![0, 32, 18]));
  assert_eq!(race(80, 100, 40), Some(vec![2, 0, 0]));
  assert_eq!(race(80, 91, 37), Some(vec![3, 21, 49]));
  assert_eq!(race(820, 81, 550), None);
}