extern crate regex;
use regex::Regex;

// Problem 1: the sum of all the multiples of 3 or 5

// If we list all the natural numbers below 10 that are multiples of 3 or 5, we get 3, 5, 6 and 9.
 // The sum of these multiples is 23.
//
// Find the sum of all the multiples of 3 or 5 below 1000.

pub fn multiples_3_and_5_sum(n: i64) -> i64 {
    let mut v = vec![];
    for i in 1..n {
        if i % 3 == 0 || i % 5 == 0 {
            // println!("{:?}", i);
            v.push(i);
        }
    }
    v.iter().sum()
}


// Problem 2: Even Fibonacci numbers

// Each new term in the Fibonacci sequence is generated by adding the previous two terms.
 // By starting with 1 and 2, the first 10 terms will be:
//
// 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, ...
//
// By considering the terms in the Fibonacci sequence whose values do not exceed four million,
// find the sum of the even-valued terms.

fn fib(n: i64) -> i64 {
    if n < 0 {
        panic!("{} is negative!", n);
    }
    match n {
        0 => panic!("zero is not a right argument to fib()!"),
        1 => 1,
        2 => 2,
        _ => fib(n - 1) + fib(n - 2),
    }
}

pub fn even_fib_nums(n: i64) -> i64 {
    let mut v = vec![];

    if fib(n) > 4000000 {
        panic!("{} is out of range!", n);
    }

    for i in 1..n + 1 {
        let fib_v = fib(i);
        match fib_v % 2 == 0 {
            true => v.push(fib_v),
            _ => (),
        }
    }
    v.iter().sum()
}

// problem3 : Largest prime factor
// 质因数（素因数或质因子）在数论里是指能整除给定正整数的质数

// The prime factors of 13195 are 5, 7, 13 and 29.
//
// What is the largest prime factor of the number 600851475143 ?

fn prime_factors(mut n: i64) -> Vec<i64> {
    let mut v = vec![];

    if n < 2 {
        v
    } else {
        let mut i = 2;
        while i <= n {
            while n % i == 0 {
                n = n / i;
                v.push(i);
            }
            i = i + 1;
        }
        v
    }
}

pub fn max_prime_factors(n: i64) -> i64 {
    let v = prime_factors(n);
    *v.iter().max().unwrap()
}

// Problem 4: Largest palindrome product 最大回文数字积

// A palindromic number reads the same both ways. The largest palindrome made from the product of two 2-digit numbers is 9009 = 91 × 99.
//
// Find the largest palindrome made from the product of two 3-digit numbers.

fn is_palindrome(num: u64) -> bool {
    let s: String = num.to_string();
    let rev_s: String = s.chars().rev().collect::<String>();
    s == rev_s
}

pub fn max_palindrome() -> u64 {
    let mut v = vec![];
    for i in 100..1000 {
        for j in 100..1000 {
            let product = i * j;
            if is_palindrome(product) {
                v.push(product);
            }
        }
    }
    *v.iter().max().unwrap()
}

// Problem 5: Smallest multiple

// 2520 is the smallest number that can be divided by each of the numbers from 1 to 10 without any remainder.
//
// What is the smallest positive number that is evenly divisible by all of the numbers from 1 to 20?

// 纯遍历性能很差
// pub fn smallest_multiple() -> usize {
//     let mut n: usize = 2520;
//
//     let mut min = 0;
//     while min == 0 {
//         let mut v: [bool; 20] = [true; 20];
//         for i in 1..21 {
//             if n % i != 0 {
//                 v[i-1] = false;
//             }
//         }
//         if v.contains(&false) {
//             println!("n: {}", n);
//             n = n+1
//         }else{
//             min = n;
//             println!("min: {}", min);
//         }
//     }
//     min
// }

// 优化：求几个数的最小公倍数。
// 一个整数要能被1-10的所有整数整除，那么就等同于他能被1-10之间的所有素数整除

// 欧几里德算法求两个数的最大公约数
fn gcd(a: u32, b: u32) -> u32 {
    if b == 0 { a } else { gcd(b, a % b) }
}

// 求多个数的最大公约数
fn mult_gcd(v: Vec<u32>) -> u32 {
    let a = v[0];
    let b = v[1];
    let mut c = gcd(a, b);
    for i in 0..v.len() {
        c = gcd(c, v[i]);
    }
    c
}

// 最小公倍数
pub fn lcm(v: Vec<u32>) -> u32 {
    let mut x = v[0];
    let mut y = v[1];
    let mut num = gcd(x, y);

    for i in 0..v.len() {
        x = num;
        y = v[i];
        let _gcd = gcd(x, y);
        num = x / _gcd * y / _gcd * _gcd;
    }
    num

}

// Problem6: Sum square difference

// The sum of the squares of the first ten natural numbers is,
//
// 1^2 + 2^2 + ... + 10^2 = 385
// The square of the sum of the first ten natural numbers is,
//
// (1 + 2 + ... + 10)^2 = 55^2 = 3025
// Hence the difference between the sum of the squares of the first ten natural numbers and the square of the sum is 3025 − 385 = 2640.
//
// Find the difference between the sum of the squares of the first one hundred natural numbers and the square of the sum.

fn square(n: u64) -> u64 {
    n * n
}

pub fn sum_square_diff(n: u64) -> u64 {
    let sum_square = square((1..n + 1).sum());
    println!("{:?}", sum_square);
    let square_sum: u64 = (1..n + 1).map(|i| square(i)).sum();
    println!("{:?}", square_sum);
    sum_square - square_sum
}

// Problem7 : 10001st prime

// By listing the first six prime numbers: 2, 3, 5, 7, 11, and 13,
// we can see that the 6th prime is 13.
//
// What is the 10001st prime number?

// 估算素数分布，素数定理： x范围内存在x/ln(x) 个素数，误差范围在0.922087
fn prime_distribution(x: f64) -> u64{
    // 1_000_000范围内有 78498个素数
    // 求第10_001个素数，大概在自然数127405内
    let num = x/x.ln();
    (num/0.922087) as u64
}

// 判断是否为素数：
// 判断小于等于一个数的平方的所有大于1的整数是不是能整除这个数，
//              如果能，则表明这个数不是素数；反之，则是素数
fn is_prime(x: usize) -> bool {
    let m = (x as f64).sqrt() as usize;
    let mut r = true;
    if x < 2{
        r = false
    }else{
        for i in 2..m+1{
            if x % i == 0{
                r = false
            }
        }
    }
    r
}
// 试除法
pub fn primes(count: usize, nth: usize) -> usize{

    let mut primes = vec![];
    primes.push(2);
    let mut pc = 1;
    let mut m = 3;
    while pc < count {
        let mut k = 0;
        while primes[k] * primes[k] <= m {
            if m % primes[k] == 0 {
                m += 2;
                k = 1;
            }else{
                k += 1;
            }
        }
        pc += 1;
        primes.push(m);
        m += 2;
    }
    primes[nth-1]
}

// 筛选法：
// 找出小于等于n的开方的素数，然后将n内所有这些素数的倍数统统去掉，剩下的数就都是素数
pub fn primes_opt(n: usize, nth: usize) -> usize {
    let mut vec = vec![true; n];
    let mut result = vec![];
    let sqrt_n = (n as f64).sqrt() as usize;
    vec[0] = false;
    vec[1] = false;

    for i in 2..sqrt_n+1{
        if vec[i] && is_prime(i) {
            let mut c = 2;
            let mut j = i*c;
            while j < n {
                vec[j] = false;
                c += 1;
                j = i*c;
            }
        }
    }

    for i in 0..vec.len(){
        if vec[i] { result.push(i) }
    }
    result[nth-1]
}

pub fn primes_opt_all(n: usize) -> Vec<usize> {
    let mut vec = vec![true; n];
    let mut result = vec![];
    let sqrt_n = (n as f64).sqrt() as usize;
    vec[0] = false;
    vec[1] = false;

    for i in 2..sqrt_n+1{
        if vec[i] && is_prime(i) {
            let mut c = 2;
            let mut j = i*c;
            while j < n {
                vec[j] = false;
                c += 1;
                j = i*c;
            }
        }
    }

    for i in 0..vec.len(){
        if vec[i] { result.push(i) }
    }
    result
}

// Problem 8 : Largest product in a series
// The four adjacent digits in the 1000-digit number that have the greatest product are 9 × 9 × 8 × 9 = 5832.


//                        73167176531330624919225119674426574742355349194934
//                        96983520312774506326239578318016984801869478851843
//                        85861560789112949495459501737958331952853208805511
//                        12540698747158523863050715693290963295227443043557
//                        66896648950445244523161731856403098711121722383113
//                        62229893423380308135336276614282806444486645238749
//                        30358907296290491560440772390713810515859307960866
//                        70172427121883998797908792274921901699720888093776
//                        65727333001053367881220235421809751254540594752243
//                        52584907711670556013604839586446706324415722155397
//                        53697817977846174064955149290862569321978468622482
//                        83972241375657056057490261407972968652414535100474
//                        82166370484403199890008895243450658541227588666881
//                        16427171479924442928230863465674813919123162824586
//                        17866458359124566529476545682848912883142607690042
//                        24219022671055626321111109370544217506941658960408
//                        07198403850962455444362981230987879927244284909188
//                        84580156166097919133875499200524063689912560717606
//                        05886116467109405077541002256983155200055935729725
//                        71636269561882670428252483600823257530420752963450

// Find the thirteen adjacent digits in the 1000-digit number that have the greatest product.
// What is the value of this product?
static DIGIT1000_NUMBER: &'static str = r#"7316717653133062491922511967442657474235534919493496983520312774506326239578318016984801869478851843858615607891129494954595017379583319528532088055111254069874715852386305071569329096329522744304355766896648950445244523161731856403098711121722383113622298934233803081353362766142828064444866452387493035890729629049156044077239071381051585930796086670172427121883998797908792274921901699720888093776657273330010533678812202354218097512545405947522435258490771167055601360483958644670632441572215539753697817977846174064955149290862569321978468622482839722413756570560574902614079729686524145351004748216637048440319989000889524345065854122758866688116427171479924442928230863465674813919123162824586178664583591245665294765456828489128831426076900422421902267105562632111110937054421750694165896040807198403850962455444362981230987879927244284909188845801561660979191338754992005240636899125607176060588611646710940507754100225698315520005593572972571636269561882670428252483600823257530420752963450"#;

pub fn largest_product(top: usize) -> u64 {
    let letters = DIGIT1000_NUMBER.chars().collect::<Vec<_>>();
    let mut max = 0;
    for i in 0..988 {
        let temp = &letters[i..i+top];
        let r:u64 = temp.iter().map(|&i| i.to_digit(10).unwrap() ).fold(1, |s, i| s* (i as u64));
        if r > max { max = r}
    }
    max
}


// Problem 9 : Special Pythagorean triplet 毕达哥拉斯三元组

// A Pythagorean triplet is a set of three natural numbers, a < b < c, for which,
//
// a^2 + b^2 = c^2
// For example, 32 + 42 = 9 + 16 = 25 = 52.
//
// There exists exactly one Pythagorean triplet for which a + b + c = 1000.
// Find the product abc.

// 公约数
fn gcd_other(mut a: u64, mut b: u64) -> u64 {
    if a < b {
        a = b;
        b = a;
    }

    while a % b != 0 {
        let temp = a;
        a = b;
        b = temp % a;
    }
    b
}

pub fn pythagorean_triplet() -> (u64, u64, u64) {
    let mut a = 0;
    let mut b = 0;
    let mut c = 0;

    let s = 1000;
    let mut k : u64;
    let mut n : u64;
    let mut d : u64;

    let mlimit: u64 = ((s/2) as f64).sqrt() as u64;

    for m in 2..mlimit+1{
        let half_s = s/2.0 as u64;
        if half_s % m == 0 {
            if m % 2 == 0 {
                k = m + 1;
            } else {
                k = m + 2;
            }
            let mhalf_s = s / (2 * m) as u64;
            while k < 2 * m && k <= mhalf_s{
                if mhalf_s % k == 0 && gcd_other(k, m) == 1 {
                    d = s / 2/ (k*m) as u64;
                    n = k - m ;
                    a = d * (m * m - n * n);
                    b = 2 * d * n * m;
                    c = d * (m * m + n * n);
                }
                k += 2;
            }
        }
    }

    (a, b, c)

}


pub fn pythagorean_triplet_product() -> u64 {
    let result = pythagorean_triplet();
    result.0 * result.1 * result.2
}

// Problem 10 ： Summation of primes
// The sum of the primes below 10 is 2 + 3 + 5 + 7 = 17.
//
// Find the sum of all the primes below two million.

pub fn sum_of_primes(n: usize) -> usize {
    primes_opt_all(n).iter().sum()
}

// Problem 11: Largest product in a grid

// In the 20×20 grid below, four numbers along a diagonal line have been marked in red.

// (26 63 78 14)

// The product of these numbers is 26 × 63 × 78 × 14 = 1788696.
//
// What is the greatest product of four adjacent numbers in the same direction
// (up, down, left, right, or diagonally) in the 20×20 grid?

pub fn regex_replace() {
    let str = "08 02 22 97 38 15 00 40 00 75 04 05 07 78 52 12 50 77 91 08 49 49 99 40 17 81 18 57 60 87 17 40 98 43 69 48 04 56 62 00 81 49 31 73 55 79 14 29 93 71 40 67 53 88 30 03 49 13 36 65 52 70 95 23 04 60 11 42 69 24 68 56 01 32 56 71 37 02 36 91 22 31 16 71 51 67 63 89 41 92 36 54 22 40 40 28 66 33 13 80 24 47 32 60 99 03 45 02 44 75 33 53 78 36 84 20 35 17 12 50 32 98 81 28 64 23 67 10 26 38 40 67 59 54 70 66 18 38 64 70 67 26 20 68 02 62 12 20 95 63 94 39 63 08 40 91 66 49 94 21 24 55 58 05 66 73 99 26 97 17 78 78 96 83 14 88 34 89 63 72 21 36 23 09 75 00 76 44 20 45 35 14 00 61 33 97 34 31 33 95 78 17 53 28 22 75 31 67 15 94 03 80 04 62 16 14 09 53 56 92 16 39 05 42 96 35 31 47 55 58 88 24 00 17 54 24 36 29 85 57 86 56 00 48 35 71 89 07 05 44 44 37 44 60 21 58 51 54 17 58 19 80 81 68 05 94 47 69 28 73 92 13 86 52 17 77 04 89 55 40 04 52 08 83 97 35 99 16 07 97 57 32 16 26 26 79 33 27 98 66 88 36 68 87 57 62 20 72 03 46 33 67 46 55 12 32 63 93 53 69 04 42 16 73 38 25 39 11 24 94 72 18 08 46 29 32 40 62 76 36 20 69 36 41 72 30 23 88 34 62 99 69 82 67 59 85 74 04 36 16 20 73 35 29 78 31 90 01 74 31 49 71 48 86 81 16 23 57 05 54 01 70 54 71 83 51 54 69 16 92 33 48 61 43 52 01 89 19 67 48";

    let re1 = Regex::new(r" ").unwrap();
    let result = re1.replace_all(str, ",");
    println!("{:?}", result);
}

// 可以把vector分组为一个二维数组
// let mut v = [1,2,3,4,5,6,7,8,9];
// let r = group_by(&v, 2);
fn group_by<'a> (v: &'a [u64], n: usize) -> Vec<&'a[u64]>{
    let mut new_v = vec![];
    group(new_v, v, n)
}

fn group<'a> (mut r:  Vec<&'a[u64]>, v: &'a [u64], n: usize) -> Vec<&'a[u64]>{
    if v.len() <= n  {
        r.push(v);
        r
    }
    else{
        r.push(v.split_at(n).0);
        group(r, v.split_at(n).1, n)
    }
}



pub fn largest_product_11()  {

    let str = "08,02,22,97,38,15,00,40,00,75,04,05,07,78,52,12,50,77,91,08,49,49,99,40,17,81,18,57,60,87,17,40,98,43,69,48,04,56,62,00,81,49,31,73,55,79,14,29,93,71,40,67,53,88,30,03,49,13,36,65,52,70,95,23,04,60,11,42,69,24,68,56,01,32,56,71,37,02,36,91,22,31,16,71,51,67,63,89,41,92,36,54,22,40,40,28,66,33,13,80,24,47,32,60,99,03,45,02,44,75,33,53,78,36,84,20,35,17,12,50,32,98,81,28,64,23,67,10,26,38,40,67,59,54,70,66,18,38,64,70,67,26,20,68,02,62,12,20,95,63,94,39,63,08,40,91,66,49,94,21,24,55,58,05,66,73,99,26,97,17,78,78,96,83,14,88,34,89,63,72,21,36,23,09,75,00,76,44,20,45,35,14,00,61,33,97,34,31,33,95,78,17,53,28,22,75,31,67,15,94,03,80,04,62,16,14,09,53,56,92,16,39,05,42,96,35,31,47,55,58,88,24,00,17,54,24,36,29,85,57,86,56,00,48,35,71,89,07,05,44,44,37,44,60,21,58,51,54,17,58,19,80,81,68,05,94,47,69,28,73,92,13,86,52,17,77,04,89,55,40,04,52,08,83,97,35,99,16,07,97,57,32,16,26,26,79,33,27,98,66,88,36,68,87,57,62,20,72,03,46,33,67,46,55,12,32,63,93,53,69,04,42,16,73,38,25,39,11,24,94,72,18,08,46,29,32,40,62,76,36,20,69,36,41,72,30,23,88,34,62,99,69,82,67,59,85,74,04,36,16,20,73,35,29,78,31,90,01,74,31,49,71,48,86,81,16,23,57,05,54,01,70,54,71,83,51,54,69,16,92,33,48,61,43,52,01,89,19,67,48";

    let mut v = str.split(",").map(|i| i.parse::<u64>().unwrap()).collect::<Vec<_>>();
    let r = group_by(&v, 20);

    println!("r: {:?}",  r);

}
