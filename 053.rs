use math::combinations::*;

pub fn pe53() {
    let mut c = 0;
    for n in 23..=100 {
        for r in 1..=n {
            if let Ok(combos) = n_combinations(n, r) {
                if combos > 1_000_000 {
                    c += 1;
                }
            } else {
                c += 1;
            }
        }
    }
    println!("{c}");
}

/* using function

pub fn n_combinations(n: u64, r: u64) -> Result<u64> {
    if n < r {
        yeet!("n < r, cannot make combinations");
    }

    let numerator = factorial_map(n);
    let mut denominator = factorial_map(r);
    let den2 = factorial_map(n - r);
    multiply_map(&mut denominator, &den2);
    let mut fraction = BigFraction::from_fact_map(numerator, denominator);
    fraction.simplify();

    let combinations = fraction.numerator()? / fraction.denominator()?;
    Ok(combinations)
}
*/
