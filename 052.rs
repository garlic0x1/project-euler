/*

It can be seen that the number, 125874, and its double, 251748, contain exactly the same digits, but in a different order.

Find the smallest positive integer, x, such that 2x, 3x, 4x, 5x, and 6x, contain the same digits.

*/

use math::{big_integer::BigInteger, permute::Permuter};

pub fn pe52() {
    let mut i = 1;
    let solution = loop {
        if permutable_mults(i) {
            break i;
        }
        i += 1;
    };
    println!("{solution}");
}

pub fn permutable_mults(i: u32) -> bool {
    let big = BigInteger::from_u32(i);
    let permuter = Permuter::new(big.raw_vec().clone());
    for j in (2..=6).rev() {
        let big2 = BigInteger::from_u32(j * i);
        if !permuter.is_permutation_of(big2.raw_vec().clone()) {
            return false;
        }
    }

    true
}
