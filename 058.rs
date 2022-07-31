/*

Starting with 1 and spiralling anticlockwise in the following way,
a square spiral with side length 7 is formed.

37 36 35 34 33 32 31
38 17 16 15 14 13 30
39 18  5  4  3 12 29
40 19  6  1  2 11 28
41 20  7  8  9 10 27
42 21 22 23 24 25 26
43 44 45 46 47 48 49

It is interesting to note that the odd squares lie along the bottom right diagonal,
but what is more interesting is that 8 out of the 13 numbers lying along both diagonals are prime;
that is, a ratio of 8/13 ≈ 62%.

If one complete new layer is wrapped around the spiral above, a square spiral with side length 9 will be formed.
If this process is continued, what is the side length of the square spiral for which the
ratio of primes along both diagonals first falls below 10%?

*/

use math::factor::is_prime;

pub fn pe58() {
    let mut diagonals = SpiralDiagonals::new();
    let mut i = 0;
    let mut primes = 0;
    while let Some(num) = diagonals.next() {
        if is_prime(num as u64) {
            primes += 1;
        }
        i += 1;

        if i > 20 && i > primes * 10 {
            println!("{}", diagonals.length);
            println!("{}", diagonals.step);
            println!("{}", num);
            break;
        }
    }
}

struct SpiralDiagonals {
    pub n: u64,
    pub step: u64,
    pub length: u64,
}

impl SpiralDiagonals {
    pub fn new() -> Self {
        Self {
            n: 1,
            step: 0,
            length: 1,
        }
    }
}

impl Iterator for SpiralDiagonals {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        if self.n < 1 {
            self.n += 1;
            self.length = 3;
            return Some(self.n);
        }

        self.n += self.length - 1;
        if self.step < 4 {
            self.step += 1;
        } else {
            self.step = 1;
            self.length += 2;
        }
        return Some(self.n);
    }
}
