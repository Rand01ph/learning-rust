fn get_middle(s:&str) -> &str {
    let len = s.len();
    if len % 2 == 0{
        &s[len/2-1..len/2+1]
    }
    else {
        &s[len/2..len/2+1]
    }
}

#[test]
fn example_tests() {
  assert_eq!(get_middle("test"),"es");
  assert_eq!(get_middle("testing"),"t");
  assert_eq!(get_middle("middle"),"dd");
  assert_eq!(get_middle("A"),"A");
  assert_eq!(get_middle("of"),"of");
}