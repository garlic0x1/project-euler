use std::{io, io::prelude::*};

struct BigInt {
    array: Vec<u8>,
}

impl BigInt {
    fn from_string(s: String) -> BigInt {
        let mut arr: Vec<u8> = Vec::new();
        for c in s.chars().rev() {
            arr.push(c.to_digit(10).unwrap() as u8);
        }
        return BigInt {
            array: arr,
        };
    }

    fn add(&mut self, add: BigInt) {
        let mut i = 0;
        let mut carry = 0;
        loop {
            if let Some((sum, car)) = self.add_place(i, &add, carry) {
                self.array[i] = sum;
                carry = car;
                i += 1;
            } else {
                if carry != 0 {
                self.array.push(carry);
                }
                break;
            }
        }
    }

    fn add_place(&mut self, place: usize, add: &BigInt, carry: u8) -> Option<(u8, u8)> {
        let a: u8;
        let b: u8;
        let mut ending = false;
        if let Some(x) = self.get(place) {
            a = *x;
        } else {
            a = 0;
            ending = true;
        }
        if let Some(x) = add.get(place) {
            b = *x;
            if ending {
                self.array.push(0);
            }
        } else {
            if ending {
                return None;
            } else {
                b = 0;
            }
        }
            let sum = carry + a + b;
            Some((sum % 10, sum / 10))
    }

    fn to_string(&self) -> String {
        let mut s = String::new();
        for i in self.array.iter().rev() {
            s = format!("{}{}", s, i);
        }
        return s;
    }

    /*
    fn size(&self) -> usize {
        return self.array.len();
    }
    */

    fn get(&self, index: usize) -> Option<&u8> {
        self.array.get(index)
    }
}

fn main() {
    let mut sum = BigInt::from_string("0".to_string());

    for line in io::stdin().lock().lines() {
        let add = BigInt::from_string(line.unwrap_or_default());
        print!("{} + {} = ", sum.to_string(), add.to_string());
        sum.add(add);
        println!("{}", sum.to_string());
    }
    
    println!("{}", sum.to_string());
}
