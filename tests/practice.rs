extern crate practice;

#[test]
fn problem1() {
    assert_eq!(practice::level1::multiples_3_and_5_sum(10), 23);
    assert_eq!(practice::level1::multiples_3_and_5_sum(1000), 233168);
}

#[test]
fn problem2() {
    assert_eq!(practice::level1::even_fib_nums(10), 44);
    assert_eq!(practice::level1::even_fib_nums(32), 4613732);
}

#[test]
fn problem3() {
    assert_eq!(practice::level1::max_prime_factors(13195), 29);
    assert_eq!(practice::level1::max_prime_factors(600851475143), 6857);
}

#[test]
fn problem4() {
    assert_eq!(practice::level1::max_palindrome(), 906609);
}

#[test]
fn problem5() {
    assert_eq!(practice::level1::lcm(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]), 2520);
    assert_eq!(practice::level1::lcm(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20]), 232792560);
}

#[test]
fn problem6() {
    assert_eq!(practice::level1::sum_square_diff(100), 25164150);
    assert_eq!(practice::level1::sum_square_diff(5), 170);
}


#[test]
fn problem7() {
    assert_eq!(practice::level1::primes(100, 1), 2);
    assert_eq!(practice::level1::primes_opt(100, 1), 2);
    assert_eq!(practice::level1::primes(100, 2), 3);
    assert_eq!(practice::level1::primes_opt(100, 2), 3);
    assert_eq!(practice::level1::primes(100, 3), 5);
    assert_eq!(practice::level1::primes_opt(100, 3), 5);
    assert_eq!(practice::level1::primes(100, 4), 7);
    assert_eq!(practice::level1::primes_opt(100, 4), 7);
    assert_eq!(practice::level1::primes(100, 5), 11);
    assert_eq!(practice::level1::primes_opt(100, 5), 11);
    assert_eq!(practice::level1::primes(100, 6), 13);
    assert_eq!(practice::level1::primes_opt(100, 6), 13);
    assert_eq!(practice::level1::primes(100, 7), 17);
    assert_eq!(practice::level1::primes_opt(100, 7), 17);
    assert_eq!(practice::level1::primes(100, 8), 19);
    assert_eq!(practice::level1::primes_opt(100, 8), 19);
    assert_eq!(practice::level1::primes(100, 9), 23);
    assert_eq!(practice::level1::primes_opt(100, 9), 23);
    assert_eq!(practice::level1::primes(100, 10), 29);
    assert_eq!(practice::level1::primes_opt(100, 10), 29);
    assert_eq!(practice::level1::primes(127405, 10001), 104743);

}


#[test]
fn problem8() {
    assert_eq!(practice::level1::largest_product(13), 23514624000);
    assert_eq!(practice::level1::largest_product(4), 5832);
}


#[test]
fn problem9() {
    assert_eq!(practice::level1::pythagorean_triplet(), (375, 200, 425));
    assert_eq!(practice::level1::pythagorean_triplet_product(), 31875000);
}

#[test]
fn problem10() {
    assert_eq!(practice::level1::sum_of_primes(10), 17);
    assert_eq!(practice::level1::sum_of_primes(100), 1060);
    assert_eq!(practice::level1::sum_of_primes(2000000), 142913828922);
}

#[test]
fn problem11() {
    assert_eq!(practice::level1::regex_replace(), ());
    assert_eq!(practice::level1::largest_product_11(), 70600674);
}

#[test]
fn problem12() {
    assert_eq!(practice::level1::solve(), 76576500);
}

#[test]
fn problem13() {
    assert_eq!(practice::level1::big_sum(), "5537376230");
}

#[test]
fn problem14() {
    assert_eq!(practice::level1::longest_collatz_sequence(), 837799);
}
