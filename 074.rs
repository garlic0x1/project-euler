/*

The number 145 is well known for the property that the sum of the factorial of its digits is equal to 145:

1! + 4! + 5! = 1 + 24 + 120 = 145

Perhaps less well known is 169, in that it produces the longest chain of numbers that link back to 169;
it turns out that there are only three such loops that exist:

169 → 363601 → 1454 → 169
871 → 45361 → 871
872 → 45362 → 872

It is not difficult to prove that EVERY starting number will eventually get stuck in a loop. For example,

69 → 363600 → 1454 → 169 → 363601 (→ 1454)
78 → 45360 → 871 → 45361 (→ 871)
540 → 145 (→ 145)

Starting with 69 produces a chain of five non-repeating terms, but the longest non-repeating chain with
a starting number below one million is sixty terms.

How many chains, with a starting number below one million, contain exactly sixty non-repeating terms?

*/

use std::collections::HashSet;

use math::digits::*;
use math::functions::*;

pub fn pe74() {
    assert_eq!(true, check_len(169, 3));
    println!("{}", chains(60, 1_000_000));
}

fn chains(len: u32, limit: u64) -> usize {
    (0..limit).filter(|&x| check_len(x, len)).count()
}

fn check_len(n: u64, len: u32) -> bool {
    let mut set = HashSet::new();
    let mut x = n;
    for _ in 1..len {
        if !set.insert(x) {
            return false;
        }
        x = digit_factorial(x);
    }

    if set.insert(x) {
        true
    } else {
        false
    }
}

fn digit_factorial(n: u64) -> u64 {
    LEndian::new(n, 10)
        .map(|x| factorial(x as u32) as u64)
        .sum()
}
