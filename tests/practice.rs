extern crate practice;

#[test]
fn problem1() {
  assert_eq!(practice::multiples_3_and_5_sum(10), 23);
  assert_eq!(practice::multiples_3_and_5_sum(1000), 233168);
}

#[test]
fn problem2() {
  assert_eq!(practice::even_fib_nums(10), 44);
  assert_eq!(practice::even_fib_nums(32), 4613732);
}

#[test]
fn problem3() {
  assert_eq!(practice::max_prime_factors(13195), 29);
  assert_eq!(practice::max_prime_factors(600851475143), 6857);
}

#[test]
fn problem4() {
  assert_eq!(practice::max_palindrome(), 906609);
}

#[test]
fn problem5() {
  assert_eq!(practice::lcm(vec![1,2,3,4,5,6,7,8,9,10]), 2520);
  assert_eq!(practice::lcm(vec![1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20]), 232792560);
}
