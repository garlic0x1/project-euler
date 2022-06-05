fn main() {
    for i in (1..=998001).rev() {
        if is_palindromic(i) && factors_3_digit(i) {
            println!("{}", i);
            return;
        }
    }
}

fn factors_3_digit(n: i64) -> bool {
    let mut i = 999;
    while i > 100 {
        if n % i == 0 && n / i > 100 && n / i < 999 {
            return true;
        }
        i -= 1;
    }
    return false;
}

fn is_palindromic(n: i64) -> bool {
    let nf = n as f64;
    let max = nf.log10().floor() as i64 + 1;

    let mut i = 0;
    while i < max/2 {
        let place1 = i64::pow(10, i as u32);
        let place2 = i64::pow(10, (max-(i+1)) as u32);
        let digit1 = (n % (place1 * 10)) / place1;
        let digit2 = (n % (place2 * 10)) / place2;
        if digit1 != digit2 {
            return false;
        }
        i += 1;
    }
    return true;
}
