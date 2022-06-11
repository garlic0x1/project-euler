fn main() {
    println!("{}", lattice_routes(20));
}

fn lattice_routes(n: usize) -> usize {
    let mut dividend = fact_dividend(n);
    let mut divisor = fact_divisor(n);
    simplify_factors(&mut dividend, &mut divisor);
    return calculate(dividend, divisor);
}

// calculate products of factors in a fraction
fn calculate(dividend: Vec<usize>, divisor: Vec<usize>) -> usize {
    let mut product_dividend: usize = 1;
    let mut product_divisor: usize = 1;
    for i in dividend.iter() {
        product_dividend *= i;
    }
    for i in divisor.iter() {
        product_divisor *= i;
    }
    return (product_dividend / product_divisor);
}

// simplify a fraction in place
fn simplify_factors(dividend: &mut Vec<usize>, divisor: &mut Vec<usize>) {
    for i in 0..dividend.len() {
        for j in 0..divisor.len() {
            if (divisor[j] != 1) && (dividend[i] % divisor[j] == 0) {
                //println!("Reducing {} / {} -> {} / 1", dividend[i], divisor[j], dividend[i] / divisor[j]);
                dividend[i] = dividend[i] / divisor[j];
                divisor[j] = 1;
            }
        }
    }
}

// produce a vector of factors for (2n)!
fn fact_dividend(n: usize) -> Vec<usize> {
    let mut ret: Vec<usize> = Vec::new();
    for i in (n+1)..=(n*2) {
        ret.push(i);
    }
    return ret;
}

// produce a vector of factors for (n!)^2
fn fact_divisor(n: usize) -> Vec<usize> {
    let mut ret: Vec<usize> = Vec::new();
    for i in 1..=n {
        ret.push(i);
    }
    return ret;
}
