/*

The smallest number expressible as the sum of a prime square, prime cube,
and prime fourth power is 28. In fact, there are exactly four numbers
below fifty that can be expressed in such a way:

28 = 22 + 23 + 24
33 = 32 + 23 + 24
49 = 52 + 23 + 24
47 = 22 + 33 + 24

How many numbers below fifty million can be expressed as the sum
of a prime square, prime cube, and prime fourth power?

*/

use math::factors::sieves::primes;

pub fn pe87() {
    println!("{}", prime_power_triples(50_000_000));
}

pub fn prime_power_triples(n: u64) -> u64 {
    let primes = primes(n);
    let mut pows = vec![false; n as usize + 1];

    primes
        .iter()
        .map(|x| x.pow(2))
        .take_while(|x| *x <= n)
        .for_each(|a| {
            primes
                .iter()
                .map(|x| x.pow(3))
                .take_while(|x| *x <= n - a)
                .for_each(|b| {
                    primes
                        .iter()
                        .map(|x| x.pow(4))
                        .take_while(|x| *x <= n - (a + b))
                        .for_each(|c| {
                            pows[(a + b + c) as usize] = true;
                        });
                });
        });

    pows.iter().filter(|&pow| *pow).count() as u64
}
