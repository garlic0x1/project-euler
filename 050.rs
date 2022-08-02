/*

The prime 41, can be written as the sum of six consecutive primes:

41 = 2 + 3 + 5 + 7 + 11 + 13
This is the longest sum of consecutive primes that adds to a prime below one-hundred.

The longest sum of consecutive primes below one-thousand that adds to a prime,
contains 21 terms, and is equal to 953.

Which prime, below one-million, can be written as the sum of the most consecutive primes?

*/

use math::sieve::*;

pub fn pe50() {
    let primes = primes(1_000_000);
    let mut max = 0;

    for prime in primes.iter().rev().flat_map(|&x| sum_chain(x, &primes)) {
        if prime.0 > max {
            max = prime.0;
            println!("{:?}", prime);
        }
    }
}

fn sum_chain(n: u64, primes: &Vec<u64>) -> Option<(u64, u64)> {
    let mut prime_iter = primes.iter();
    while let Some(&x) = prime_iter.next() {
        let mut sum = x;
        let mut c = 1;
        let mut it2 = prime_iter.clone();
        while let Some(&y) = it2.next() {
            c += 1;
            sum += y;
            if sum == n {
                return Some((c, n));
            } else if sum > n {
                break;
            }
        }
    }
    None
}
