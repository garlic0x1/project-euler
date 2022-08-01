/*

An irrational decimal fraction is created by concatenating the positive integers:

0.123456789101112131415161718192021...

It can be seen that the 12th digit of the fractional part is 1.

If dn represents the nth digit of the fractional part, find the value of the following expression.

d1 × d10 × d100 × d1_000 × d10_000 × d100_000 × d1_000_000

*/

use math::big_integer::BigInteger;

pub fn pe40() {
    println!(
        "{}",
        d(1) * d(10) * d(100) * d(1_000) * d(10_000) * d(100_000) * d(1_000_000)
    );
}

fn d(n: u32) -> u32 {
    let mut step = 1;
    let mut i = 1;
    let mut num = 1;
    loop {
        if i >= n {
            let big = BigInteger::from_u32(num);
            let vec = big.raw_vec().clone();
            if let Some(ret) = vec.get((i - n) as usize) {
                break ret.clone() as u32;
            }
        }

        num += 1;
        if is_pow_10(num) {
            step += 1;
        }
        i += step;
    }
}

fn is_pow_10(n: u32) -> bool {
    if n <= 0 {
        return false;
    } else if n == 1 {
        return true;
    } else if n % 10 != 0 {
        return false;
    }
    is_pow_10(n / 10)
}
