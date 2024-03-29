/*

A googol (10100) is a massive number: one followed by one-hundred zeros;
100100 is almost unimaginably large: one followed by two-hundred zeros.
Despite their size, the sum of the digits in each number is only 1.

Considering natural numbers of the form, ab, where a, b < 100,
what is the maximum digital sum?

*/

use math::big_integer::BigInteger;

pub fn pe56() {
    let mut max = 0;
    for a in 10..100 {
        for b in 10..100 {
            let mut n = BigInteger::from_u32(a, None);
            n.power(b);
            let ds = digit_sum(&n);
            if ds > max {
                max = ds;
            }
        }
    }
    println!("{max}");
}

fn digit_sum(num: &BigInteger) -> u32 {
    num.raw_vec().iter().map(|&x| x as u32).sum::<u32>()
}
