/*

It is possible to write ten as the sum of
primes in exactly five different ways:

7 + 3
5 + 5
5 + 3 + 2
3 + 3 + 2 + 2
2 + 2 + 2 + 2 + 2

What is the first value which can be written as the
sum of primes in over five thousand different ways?

*/

use math::factors::sieves::primes;

pub fn pe77() {
    let primes = primes(10_000);
    let sol = (10..)
        .map(|x| (x, combos_rec(x, x - 1, &primes)))
        // fold until break
        .try_fold(0, |res, (x, combos)| {
            if combos <= 5_000 {
                Ok(res)
            } else {
                // break out of fold
                Err(x)
            }
        });

    // display solution
    if let Err(sol) = sol {
        println!("{sol}");
    }
}

fn combos_rec(n: u64, last: u64, primes: &Vec<u64>) -> u64 {
    if n == 0 {
        1
    } else {
        primes
            .iter()
            .take_while(|&x| *x <= std::cmp::min(n, last))
            .map(|&x| combos_rec(n - x, x, primes))
            .sum::<u64>()
    }
}
