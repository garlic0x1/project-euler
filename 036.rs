/*

The decimal number, 585 = 10010010012 (binary), is palindromic in both bases.

Find the sum of all numbers, less than one million, which are palindromic in base 10 and base 2.

(Please note that the palindromic number, in either base, may not include leading zeros.)

*/

fn main() {
    println!("{}", sum_palindromes(1_000_000));
}

fn sum_palindromes(limit: u32) -> u32 {
    let mut sum = 1;
    let mut i = 1;

    loop {
        let bint = BinaryInteger::from_decimal(i, None);
        let n = bint.calculate();
        if n > limit {
            break;
        }
        if bint.decimal_palindrome(n) {
            sum += n;
        }

        let bint = BinaryInteger::from_decimal(i, Some(true));
        let n = bint.calculate();
        if n < limit {
            if bint.decimal_palindrome(n) {
                sum += n;
            }
        }

        let bint = BinaryInteger::from_decimal(i, Some(false));
        let n = bint.calculate();
        if n < limit {
            if bint.decimal_palindrome(n) {
                sum += n;
            }
        }
        i += 1;
    }

    sum
}

struct BinaryInteger {
    digits: Vec<bool>,
}

impl BinaryInteger {
    fn from_decimal(n: u32, middle: Option<bool>) -> Self {
        let mut digits = Vec::new();
        let mut n = n;
        loop {
            let rem = n % 2;
            if rem == 1 {
                digits.push(true);
            } else if rem == 0 {
                digits.push(false);
            }
            n /= 2;
            if n < 1 {
                break;
            }
        }
        let mut mirrored = Vec::new();
        for b in digits.iter().rev() {
            mirrored.push(*b);
        }
        if let Some(b) = middle {
            mirrored.push(b);
        }
        for b in digits.iter() {
            mirrored.push(*b);
        }
        Self { digits: mirrored }
    }
    fn new_mirror(digits: Vec<bool>) -> Self {
        let mut mirrored = Vec::new();
        for b in digits.iter() {
            mirrored.push(*b);
        }
        for b in digits.iter().rev() {
            mirrored.push(*b);
        }
        Self { digits: mirrored }
    }

    fn calculate(&self) -> u32 {
        let mut place = 0;
        let mut sum = 0;
        for b in self.digits.iter() {
            if *b {
                sum += (2 as u32).pow(place);
            }
            place += 1;
        }
        sum
    }

    fn decimal_palindrome(&self, n: u32) -> bool {
        let nf = n as f64;
        let max = nf.log10().floor() as i64 + 1;

        let mut i = 0;
        while i < max / 2 {
            let place1 = u32::pow(10, i as u32);
            let place2 = u32::pow(10, (max - (i + 1)) as u32);
            let digit1 = (n % (place1 * 10)) / place1;
            let digit2 = (n % (place2 * 10)) / place2;
            if digit1 != digit2 {
                return false;
            }
            i += 1;
        }
        return true;
    }
}
