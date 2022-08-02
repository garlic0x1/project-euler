/*

The primes 3, 7, 109, and 673, are quite remarkable. By taking any two primes
and concatenating them in any order the result will always be prime. For example,
taking 7 and 109, both 7109 and 1097 are prime. The sum of these four primes, 792,
represents the lowest sum for a set of four primes with this property.

Find the lowest sum for a set of five primes for which any two primes concatenate to produce another prime.

*/

use math::{factor::is_prime, sieve::primes};

fn rec(stack: &mut Vec<u64>, limit: usize, primes: &Vec<u64>) -> Option<Vec<u64>> {
    if stack.len() >= limit {
        return Some(stack.clone());
    }

    let max = match stack.last() {
        Some(val) => *val,
        None => u64::MAX,
    };

    'out: for &p in primes.iter().take_while(|&x| *x < max) {
        for &n in stack.iter() {
            if !(is_prime(fast_cat(p, n)) && is_prime(fast_cat(n, p))) {
                continue 'out;
            }
        }

        stack.push(p);

        if let Some(answer) = rec(stack, limit, primes) {
            return Some(answer);
        }

        stack.pop();
    }

    None
}

pub fn pe60() {
    let primes = primes(1_000_000);

    println!(
        "{:?}",
        rec(&mut Vec::new(), 5, &primes)
            .unwrap()
            .iter()
            .sum::<u64>()
    );
}

fn fast_cat(a: u64, b: u64) -> u64 {
    let mut x = a;
    let mut dec = b;
    while dec > 0 {
        dec /= 10;
        x *= 10;
    }
    x + b
}
