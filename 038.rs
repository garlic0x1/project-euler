/*

Take the number 192 and multiply it by each of 1, 2, and 3:

192 × 1 = 192
192 × 2 = 384
192 × 3 = 576
By concatenating each product we get the 1 to 9 pandigital, 192384576.
We will call 192384576 the concatenated product of 192 and (1,2,3)

The same can be achieved by starting with 9 and multiplying by 1, 2, 3, 4, and 5,
giving the pandigital, 918273645, which is the concatenated product of 9 and (1,2,3,4,5).

What is the largest 1 to 9 pandigital 9-digit number that can be formed as the
concatenated product of an integer with (1,2, ... , n) where n > 1?

*/

use math::functions::*;

pub fn pe38() {
    let mut max = 0;
    'out: for i in (1..10_000).filter(|&n| !numrepeats(n)) {
        let mut num: u64 = i;
        let mut fact = 2;

        while numdigits(num as u64) < 9 {
            num = numcat(num, i * fact);
            if numrepeats(num) {
                continue 'out;
            }
            fact += 1;
        }

        let digits = digits(num);
        if digits.len() == 9 && !digits.contains(&0) && num > max {
            max = num;
        }
    }
    println!("{max}");
}
