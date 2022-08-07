/*

The cube, 41063625 (3453), can be permuted to produce two other cubes:
56623104 (3843) and 66430125 (4053). In fact, 41063625 is the smallest
cube which has exactly three permutations of its digits which are also cube.

Find the smallest cube for which exactly five permutations of its digits are cube.

*/

use std::{
    collections::{BTreeMap, HashMap},
    hash::{Hash, Hasher},
};

use math::big_integer::BigInteger;

fn digit_map(num: &BigInteger) -> DigitMap {
    let mut map = BTreeMap::new();
    for &digit in num.raw_vec() {
        if let Some(c) = map.get_mut(&digit) {
            *c += 1;
        } else {
            map.insert(digit, 1);
        }
    }
    DigitMap { digits: map }
}

#[derive(Clone, Eq)]
struct DigitMap {
    pub digits: BTreeMap<u8, u32>,
}

impl Hash for DigitMap {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let s = format!("{:?}", self.digits);
        s.hash(state);
    }
}

impl PartialEq for DigitMap {
    fn eq(&self, other: &Self) -> bool {
        let s = format!("{:?}", self.digits);
        let os = format!("{:?}", other.digits);

        s == os
    }
}

pub fn pe62() {
    let mut map: HashMap<DigitMap, (u32, u32)> = HashMap::new();

    for (i, cube) in (1..)
        .map(|x| {
            let mut big = BigInteger::from_u32(x, None);
            big.power(3);
            big
        })
        .enumerate()
    {
        let digits = digit_map(&cube);
        if let Some((count, smallest)) = map.get_mut(&digits) {
            *count += 1;
            if i < *smallest as usize {
                *smallest = i as u32;
            }
            if *count >= 5 {
                println!("{} ^ 3", *smallest + 1);
                break;
            }
        } else {
            map.insert(digits, (1, i as u32));
        }
    }
}
