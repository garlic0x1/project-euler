/*

The fraction 49/98 is a curious fraction, as an inexperienced mathematician in attempting to simplify it may incorrectly believe that 49/98 = 4/8, which is correct, is obtained by cancelling the 9s.

We shall consider fractions like, 30/50 = 3/5, to be trivial examples.

There are exactly four non-trivial examples of this type of fraction, less than one in value, and containing two digits in the numerator and denominator.

If the product of these four fractions is given in its lowest common terms, find the value of the denominator.

*/

fn main() {
    let mut n = 1;
    let mut d = 1;
    for i in 10..100 {
        for j in 10..100 {
            let frac = Fraction::new(i , j );
            if frac.is_nontrivial() {
                n *= frac.numerator ;
                d *= frac.denominator ;
            }
        }
    }
    println!("{}/{}", n, d);
    loop {
        let mut prime = true;
        for i in 2..d/2 {
            if d % i == 0 && n % i == 0 {
                d /= i;
                n /= i;
                prime = false;
                break;
            }
        }
        if prime {
            break;
        }
    }
    println!("{}/{}", n, d);
}

#[derive(Debug)]
struct Fraction {
    numerator: u32,
    denominator: u32,
}

impl Fraction {
    fn new(numerator: u32, denominator: u32) -> Self {
        Self {
            numerator,
            denominator,
        }
    }

    fn is_cancelling(&self) -> bool {
        let na: f32 = (self.numerator / 10) as f32;
        let nb: f32 = (self.numerator % 10) as f32;
        let da: f32 = (self.denominator / 10) as f32;
        let db: f32 = (self.denominator % 10) as f32;

        if na == da {
            return db != 0.0 && self.numerator as f32 / self.denominator as f32 == nb / db;
        }
        if na == db {
            return da != 0.0 && self.numerator as f32 / self.denominator as f32 == nb / da;
        }
        if nb == da {
            return db != 0.0 && self.numerator as f32 / self.denominator as f32 == na / db;
        }
        if nb == db {
            return da != 0.0 && self.numerator as f32 / self.denominator as f32 == na / da;
        }

        false
    }

    fn is_nontrivial(&self) -> bool {
        let na: f32 = (self.numerator as u32 / 10) as f32;
        let nb: f32 = (self.numerator as u32 % 10) as f32;
        let da: f32 = (self.denominator as u32 / 10) as f32;
        let db: f32 = (self.denominator as u32 % 10) as f32;

        return  self.numerator < self.denominator && !(nb == 0.0 && db == 0.0) && self.is_cancelling();
    }
}
