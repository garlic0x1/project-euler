/*

By replacing the 1st digit of the 2-digit number *3, it turns out
that six of the nine possible values: 13, 23, 43, 53, 73, and 83, are all prime.

By replacing the 3rd and 4th digits of 56**3 with the same digit,
this 5-digit number is the first example having seven primes among the ten generated numbers, yielding the family: 56003, 56113, 56333, 56443, 56663, 56773, and 56993. Consequently 56003, being the first member of this family, is the smallest prime with this property.

Find the smallest prime which, by replacing part of the number
(not necessarily adjacent digits) with the same digit, is part of an eight prime value family.

*/

use math::{bignums::big_integer::BigInteger, digits::LEndian, factors::sieves::primes};

pub fn pe51() {
    let primes = primes(10_000_000);
    for p in primes.iter() {
        for setdig in (0..5) {
            for ind in set_of_digit(*p, setdig).iter() {
                // println!("{}", *p);
                if try_replace(*p, ind, 8, &primes) {
                    println!("{}", *p);
                    return;
                }
            }
        }
    }
}

fn try_replace(n: u64, indexes: &Vec<usize>, limit: usize, primes: &Vec<u64>) -> bool {
    //println!("try: {n}");
    let num: Vec<u8> = LEndian::new(n, 10).collect();
    (0..10)
        .filter(|d| {
            let mut rep = num.clone();
            indexes.iter().for_each(|i| rep[*i] = *d);
            let big = BigInteger::from_vec(rep, None).to_u32().unwrap() as u64;
            if LEndian::new(big, 10).count() == num.len() {
                let res = primes.binary_search(&big).is_ok();
                // println!("{big} = {res}");
                res
            } else {
                false
            }
        })
        .count()
        >= limit
}

fn set_of_digit(n: u64, digit: u8) -> Vec<Vec<usize>> {
    let digits: Vec<usize> = LEndian::new(n, 10)
        .enumerate()
        .filter(|d| d.1 == digit)
        .map(|d| d.0)
        .collect();

    let mut sets = vec![];

    if digits.len() > 1 {
        digits.iter().for_each(|a| {
            digits.iter().filter(|b| *b != a).for_each(|b| {
                sets.push(vec![*a, *b]);

                digits.iter().filter(|c| *c != a && *c != b).for_each(|c| {
                    sets.push(vec![*a, *b, *c]);
                    digits
                        .iter()
                        .filter(|d| *d != a && *d != b && *d != c)
                        .for_each(|d| {
                            sets.push(vec![*a, *b, *c, *d]);
                        });
                });
            });
        });
    }

    sets
}
