/*

It is possible to show that the square root of two can be expressed as an infinite continued fraction.
By expanding this for the first four iterations, we get:
The next three expansions are

, but the eighth expansion,

, is the first example where the number of digits in the numerator exceeds the number of digits in the denominator.

In the first one-thousand expansions, how many fractions contain a numerator with more digits than the denominator?

*/

pub fn pe57() {
    let mut c = 0;
    println!("{:?}", sqrt_cont_rec(1000, (2.0, 1.0), &mut c));
    println!("{c}");
}

/// start with 2 / 1
fn sqrt_cont_rec(n: u64, frac: (f64, f64), counter: &mut u64) -> (f64, f64) {
    let resnum = frac.0 + frac.1;
    let resden = frac.0;

    let cnum = resnum.log10().ceil();
    let cres = resden.log10().ceil();

    if cnum > cres {
        *counter += 1;
    }

    if n > 0 {
        let mut newnum = (frac.0 * 2.0) + frac.1;
        let mut newden = frac.0;

        while newnum.log10().ceil() > 20.0 && newden.log10().ceil() > 20.0 {
            newnum /= 10.0;
            newden /= 10.0;
        }

        sqrt_cont_rec(n - 1, (newnum, newden), counter)
    } else {
        (resnum, resden)
    }
}
