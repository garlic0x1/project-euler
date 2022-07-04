/*

The number, 197, is called a circular prime because all rotations of the digits: 197, 971, and 719, are themselves prime.

There are thirteen such primes below 100: 2, 3, 5, 7, 11, 13, 17, 31, 37, 71, 73, 79, and 97.

How many circular primes are there below one million?

*/

use std::collections::HashSet;

fn main() {
    println!("circular primes under {}: {}", 100, count_circle_primes(100));
    println!("circular primes under {}: {}", 1_000_000, count_circle_primes(1_000_000));
}

fn count_circle_primes(limit: u32) -> u32 {
    let mut sum = 1;

    for i in 2..limit {
        if is_circular_prime(rotate_digit(i)) {
            sum += 1;
        }
    }

    sum
}

fn rotate_digit(digit: u32) -> Vec<u32> {
    let mut ret = Vec::new();
    const radix: u32 = 10;

    let mut n = digit;
    let mut last_place = 0;
    while n >= 1 {
            n /= radix;
            last_place += 1;
    }
    last_place -= 1;
    n = digit;
    for i in 0..=last_place {
            ret.push(n);
            let ones = n % 10;
            n /= 10;
            n += ones * (10 as u32).pow(last_place);
    }

    ret
}

fn is_prime(n: u32) -> bool {
    for i in 2..=((n as f64).sqrt() as u32) {
        if n % i == 0 {
            return false;
        }
    }
    return true;
}

fn is_circular_prime(arr: Vec<u32>) -> bool {
    let mut tsum = 0;
    for value in arr.iter() {
        tsum += value;
        let cmp = value % 10;
        if cmp % 2 == 0 {
            return false;
        }
    }
    if tsum % 3 == 0 {
  //      return false;
    }

    for value in arr {
        if !is_prime(value) {
                return false;
        }
    }
    true
}
