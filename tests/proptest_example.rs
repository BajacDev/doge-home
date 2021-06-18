use proptest::prelude::*;

proptest! {
  #[test]
  fn test_add(a in 0..1000i32, b in 0..1000i32) {
    let sum = a + b;
    assert!(sum >= b);
    assert!(sum >= a);
  }
}
