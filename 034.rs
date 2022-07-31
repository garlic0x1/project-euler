/*

145 is a curious number, as 1! + 4! + 5! = 1 + 24 + 120 = 145.

Find the sum of all numbers which are equal to the sum of the factorial of their digits.

Note: As 1! = 1 and 2! = 2 are not sums they are not included.

*/

use math::big_integer::BigInteger;

pub fn pe34() {
    let mut sum = 0;
    for n in 3..2540161 {
        let big = BigInteger::from_u32(n);
        let arr = big.raw_vec();

        if n == sum_factorial(&arr) {
            sum += n;
        }
    }
    println!("{sum}");
}

fn sum_factorial(arr: &Vec<u8>) -> u32 {
    let mut sum = 0;
    for num in arr {
        sum += factorial(*num as u32);
    }
    sum
}

fn factorial(x: u32) -> u32 {
    if x > 1 {
        x * factorial(x - 1)
    } else {
        1
    }
}
