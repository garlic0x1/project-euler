use math::big_integer::*;
use math::factor::*;
use std::collections::HashMap;

pub fn pe49() {
    for i in 1000..=7000 {
        if is_prime(i) {
            for step in 1..3335 {
                let mut x = i + step;
                let mut nums = vec![i];
                let mut counter = 1;
                while x < 10000 && counter < 4 {
                    if is_prime(x) && is_permutation(i as u32, x as u32) {
                        nums.push(x)
                    } else {
                        break;
                    }
                    x += step;
                    counter += 1;
                }

                if nums.len() == 3 {
                    println!("{:?}", nums)
                }
            }
        }
    }
}

fn is_permutation(num1: u32, num2: u32) -> bool {
    let big1 = BigInteger::from_u32(num1);
    let big2 = BigInteger::from_u32(num2);
    let vec1 = big1.raw_vec();
    let vec2 = big2.raw_vec();
    if vec1.len() != vec2.len() {
        return false;
    }
    let mut set1 = HashMap::new();
    for x in vec1 {
        if let Some(count) = set1.get_mut(x) {
            *count += 1;
        } else {
            set1.insert(x, 1);
        }
    }
    let mut set2 = HashMap::new();
    for x in vec2 {
        if let Some(count) = set2.get_mut(x) {
            *count += 1;
        } else {
            set2.insert(x, 1);
        }
    }
    set1 == set2
}
