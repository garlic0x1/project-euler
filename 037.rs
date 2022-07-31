/*

The number 3797 has an interesting property. Being prime itself,
it is possible to continuously remove digits from left to right,
and remain prime at each stage: 3797, 797, 97, and 7.
Similarly we can work from right to left: 3797, 379, 37, and 3.

Find the sum of the only eleven primes that are both truncatable from left to right and right to left.

NOTE: 2, 3, 5, and 7 are not considered to be truncatable primes.

*/

use math::big_integer::*;
use math::factor::*;
use math::sieve::*;

pub fn pe37() {
    let mut sum = 0;
    for prime in primes(1_000_000).iter() {
        if *prime < 10 {
            continue;
        }
        let big = BigInteger::from_u32(prime.clone() as u32);
        let vec = big.raw_vec();
        if is_prime_truncatable(vec.clone()) && is_prime_truncatable_rev(vec.clone()) {
            sum += prime;
        }
    }
    println!("{sum}");
}

fn is_prime_truncatable(big: Vec<u8>) -> bool {
    let mut big = big;
    while big.len() > 1 {
        big.pop();
        let new = BigInteger::from_vec(big.clone());
        let int = new.to_u32().unwrap();
        if !is_prime(int as u64) {
            return false;
        }
    }
    true
}

fn is_prime_truncatable_rev(big: Vec<u8>) -> bool {
    let mut big = big;
    while big.len() > 1 {
        big.remove(0);
        let new = BigInteger::from_vec(big.clone());
        let int = new.to_u32().unwrap();
        if !is_prime(int as u64) {
            return false;
        }
    }

    true
}
