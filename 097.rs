/*

The first known prime found to exceed one million
digits was discovered in 1999, and is a Mersenne
prime of the form 2^6972593−1; it contains exactly
2,098,960 digits. Subsequently other Mersenne primes,
of the form 2p−1, have been found which contain more digits.

However, in 2004 there was found a massive non-Mersenne
prime which contains 2,357,207 digits: 28433×2^7830457+1.

Find the last ten digits of this prime number.

*/

use math::bignums::big_integer::BigInteger;

pub fn pe97() {
    let start = 28433;
    let mut big = BigInteger::from_u32(start, Some(10));
    (0..7830457).for_each(|_| big.multiply_digit(2).unwrap());
    println!("{}", big.to_u64().unwrap() + 1);
}
