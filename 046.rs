/*

Goldbach's Other Conjecture:

It was proposed by Christian Goldbach that every odd composite number
can be written as the sum of a prime and twice a square.

9 = 7 + 2×12
15 = 7 + 2×22
21 = 3 + 2×32
25 = 7 + 2×32
27 = 19 + 2×22
33 = 31 + 2×12

It turns out that the conjecture was false.

What is the smallest odd composite that cannot be written as the
sum of a prime and twice a square?

*/

use math::factor::*;
use math::sieve::*;

pub fn pe46() {
    for odd_composite in composites(1_000_000).iter().filter(|x| *x % 2 != 0) {
        let mut follows_conjecture = false;
        for prime in primes(odd_composite - 2) {
            let mut i = 1;
            while (2 * (i * i)) + prime < *odd_composite {
                i += 1;
            }

            if (2 * (i * i)) + prime == *odd_composite {
                follows_conjecture = true;
                break;
            }
        }

        if !follows_conjecture {
            println!("{odd_composite}");
        }
    }
}
