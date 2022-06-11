/*
 * n! means n × (n − 1) × ... × 3 × 2 × 1
 * 
 * For example, 10! = 10 × 9 × ... × 3 × 2 × 1 = 3628800,
 * and the sum of the digits in the number 10! is 3 + 6 + 2 + 8 + 8 + 0 + 0 = 27.
 * 
 * Find the sum of the digits in the number 100!
 */

struct BigInt {
    array: Vec<usize>,
}

impl BigInt {
    fn from_usize(n: usize) -> Self {
        Self {
            array: vec![n; 1],
        }
    }

    // multiplies the fact in place
    fn multiply(&mut self, factor: usize) {
        let mut carry = 0;
        for i in 0..self.array.len() {
            carry = self.multiply_place(i, carry, factor);
        }
        if carry != 0 {
            if carry/10 < 1 {
                self.array.push(carry);
            } else {
                self.array.push(carry%10);
                self.array.push(carry/10);
            }
        }
    }

    fn multiply_place(&mut self, place: usize, carry: usize, factor: usize) -> usize {
        let product = (self.array.get(place).unwrap() * factor) + carry;
        self.array[place] = product % 10;
        return product / 10;
    }

    fn sum_array(&self) -> usize {
        let mut sum = 0;
        for i in self.array.iter() {
            sum += i;
        }
        return sum;
    }
}

fn main() {
    assert_eq!(27, sum_factorial(10));
    println!("{}", sum_factorial(100));
}

fn sum_factorial(n: usize) -> usize {
    let factorial: Vec<usize> = from_factorial(n);
    let mut result: BigInt = BigInt::from_usize(1);
    for factor in factorial.iter() {
        result.multiply(*factor);
    }
    return result.sum_array();
}

// returns a new fact with a starting value
fn from_factorial(n: usize) -> Vec<usize> {
    let mut arr: Vec<usize> = Vec::new();
    for i in 1..=n {
        arr.push(i);
    }
    return arr;
}
