/*
 pandigital products

We shall say that an n-digit number is pandigital if it makes use of all the digits 1 to n exactly once; for example, the 5-digit number, 15234, is 1 through 5 pandigital.

The product 7254 is unusual, as the identity, 39 × 186 = 7254, containing multiplicand, multiplier, and product is 1 through 9 pandigital.

Find the sum of all products whose multiplicand/multiplier/product identity can be written as a 1 through 9 pandigital.

HINT: Some products can be obtained in more than one way so be sure to only include it once in your sum.

*/

use std::collections::HashSet;
use permute::*;


fn main() {
    println!("Hello, world!");
    let id = Identity::new(39, 186, 7254);
    println!("{}", id.is_pandigital());
    let id = Identity::new(33, 186, 7254);
    println!("{}", id.is_pandigital());
    println!("{:?}", identities_from_permutation(&vec![1,2,3,4,5,6,7,8,9]));

    let mut found = HashSet::new();
    for permutation in permute(vec![1,2,3,4,5,6,7,8,9]) {
        for identity in identities_from_permutation(&permutation) {
            if identity.is_true() {
                found.insert(identity.product);
            }
        }
    }

    let mut sum = 0;
    for val in found.iter() {
        println!("{} has pandigital identity", val);
        sum += val;
    }

    println!("sum: {}", sum);
}

fn permutations(list: &Vec<u32>) {
}

fn identities_from_permutation(permutation: &Vec<u32>) -> Vec<Identity> {
    let mut ret = Vec::new();

    for i in 1..5 {
        for j in 1..5 {
            let mut perm = permutation.clone();
            let mut multiplier = 0;
            let mut multiplicand = 0;
            let mut product = 0;
            let mut place = 1;
            
            for _ in 0..i {
                multiplier += place * perm.pop().unwrap();
                place *= 10;
            }

            let mut place = 1;
            for _ in 0..j {
                multiplicand += place * perm.pop().unwrap();
                place *= 10;
            }

            let mut place = 1;
            for _ in 0..9-(j+i) {
                product += place * perm.pop().unwrap();
                place *= 10;
            }

            ret.push(Identity::new(multiplicand, multiplier, product));
        }
    }

    ret
}

#[derive(Debug)]
struct Identity {
    multiplicand: u32,
    multiplier: u32,
    product: u32,
}

impl Identity {
    fn new(multiplicand: u32, multiplier: u32, product: u32) -> Self {
        Self {
            multiplicand,
            multiplier,
            product,
        }
    }

    fn is_true(&self) -> bool {
        self.multiplicand * self.multiplier == self.product
    }

    fn is_pandigital(&self) -> bool {
        let mut set = HashSet::new();
        const base: u32 = 10;

        let mut x = self.multiplicand;
        while x >= 1 {
            let value = x % base;
            x /= base;
            if set.contains(&value) {
                return false;
            } else {
                set.insert(value);
            }
        }

        let mut x = self.multiplier;
        while x >= 1 {
            let value = x % base;
            x /= base;
            if set.contains(&value) {
                return false;
            } else {
                set.insert(value);
            }
        }

        let mut x = self.product;
        while x >= 1 {
            let value = x % base;
            x /= base;
            if set.contains(&value) {
                return false;
            } else {
                set.insert(value);
            }
        }

        true
    }
}
