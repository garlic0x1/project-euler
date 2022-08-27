/*

The proper divisors of a number are all the divisors excluding the number
itself. For example, the proper divisors of 28 are 1, 2, 4, 7, and 14.
As the sum of these divisors is equal to 28, we call it a perfect number.

Interestingly the sum of the proper divisors of 220 is 284 and the sum
of the proper divisors of 284 is 220, forming a chain of two numbers.
For this reason, 220 and 284 are called an amicable pair.

Perhaps less well known are longer chains. For example, starting with 12496,
we form a chain of five numbers:

12496 → 14288 → 15472 → 14536 → 14264 (→ 12496 → ...)

Since this chain returns to its starting point, it is called an amicable chain.

Find the smallest member of the longest amicable chain with no element exceeding one million.

*/

use std::collections::HashSet;

pub fn pe95() {
    println!("{}", untangle(&sum_divisor_sieve(1_000_000)));
}

fn sum_divisor_sieve(limit: usize) -> Vec<usize> {
    let mut sieve = vec![1; limit + 1];

    (2..).take_while(|i| i * 2 <= limit).for_each(|i| {
        (2..).take_while(|j| i * j <= limit).for_each(|j| {
            sieve[(i * j)] += i;
        });
    });

    sieve
}

fn untangle(chains: &Vec<usize>) -> usize {
    let mut longest = 0;
    let mut sol = 0;
    let mut seen = vec![false; chains.len()];
    (1..chains.len()).for_each(|i| {
        if !seen[i] {
            if let Some(amic_loop) = check_loop(chains, &seen, i) {
                if amic_loop.len() > longest {
                    longest = amic_loop.len();
                    sol = i;
                }
                amic_loop.iter().for_each(|i| seen[*i] = true);
            }
        }
    });
    sol
}

fn check_loop(chains: &Vec<usize>, seen: &Vec<bool>, index: usize) -> Option<HashSet<usize>> {
    let mut seen_local = HashSet::new();
    let mut i = index;
    while let Some(&next) = chains.get(i) {
        if next == index {
            seen_local.insert(next);
            return Some(seen_local);
        } else if next >= seen.len() || seen[next] || seen_local.contains(&next) {
            return None;
        }
        seen_local.insert(next);
        i = next;
    }
    None
}
