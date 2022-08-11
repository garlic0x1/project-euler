/*

Euler's Totient function, φ(n) [sometimes called the phi function],
is used to determine the number of positive numbers less than or equal
to n which are relatively prime to n. For example, as 1, 2, 4, 5, 7, and 8,
are all less than nine and relatively prime to nine, φ(9)=6.
The number 1 is considered to be relatively prime to every positive number, so φ(1)=1.

Interestingly, φ(87109)=79180, and it can be seen that 87109 is a permutation of 79180.

Find the value of n, 1 < n < 107, for which φ(n) is a permutation of n and the ratio n/φ(n) produces a minimum.

*/

use std::collections::HashMap;

use math::digits::*;
use math::factor::*;

pub fn pe70() {
    let sol = (2..10_000_000)
        .map(|x| (x, totient(x)))
        .filter(|(x, t)| is_perm(*x, *t))
        .map(|(x, t)| (x, x as f32 / t as f32))
        .fold(
            (f32::INFINITY, 0),
            |a, (x, ratio)| {
                if ratio < a.0 {
                    (ratio, x)
                } else {
                    a
                }
            },
        );

    println!("{:?}", sol);
}

fn is_perm(a: u32, b: u32) -> bool {
    digit_map(a) == digit_map(b)
}

fn digit_map(n: u32) -> HashMap<u8, usize> {
    let mut map = HashMap::new();

    for x in LEndian::new(n as u64, 10) {
        if let Some(val) = map.get_mut(&x) {
            *val += 1;
        } else {
            map.insert(x, 1);
        }
    }

    map
}
