/*


The number, 1406357289, is a 0 to 9 pandigital number because it is made up of each of the digits 0 to 9 in some order,
but it also has a rather interesting sub-string divisibility property.

Let d1 be the 1st digit, d2 be the 2nd digit, and so on. In this way, we note the following:

d2d3d4=406 is divisible by 2
d3d4d5=063 is divisible by 3
d4d5d6=635 is divisible by 5
d5d6d7=357 is divisible by 7
d6d7d8=572 is divisible by 11
d7d8d9=728 is divisible by 13
d8d9d10=289 is divisible by 17
Find the sum of all 0 to 9 pandigital numbers with this property.

*/

use permute::permutations_of;

fn main() {
    println!("{}", test(&vec![1, 4, 0, 6, 3, 5, 7, 2, 8, 9]));
    let mut sum = 0;
    let set: Vec<u8> = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    for pandigital in permutations_of(&set) {
        let vec: Vec<u8> = pandigital.map(|x| x.clone()).collect();
        if test(&vec) {
            sum += to_integer(&vec);
        }
    }

    println!("{}", sum);
}

fn test(array: &Vec<u8>) -> bool {
    // println!("{:?}", array);
    let primes = vec![2, 3, 5, 7, 11, 13, 17];
    let mut i = 1;
    for n in primes {
        let tri = trigram(i, array);
        // println!("{tri} mod {n}");
        if tri % n != 0 {
            return false;
        }
        i += 1;
    }
    true
}

fn trigram(index: usize, arr: &Vec<u8>) -> u16 {
    let mut sum: u16 = 0;
    let slice = &arr[index..index + 3];
    let mut place = 1;

    for num in slice.iter().rev() {
        sum += place * *num as u16;
        place *= 10;
    }
    sum
}

fn to_integer(array: &Vec<u8>) -> u64 {
    let mut sum: u64 = 0;
    let mut place = 1;
    for num in array.iter().rev() {
        if sum > sum + place * *num as u64 {
            println!("overflow");
        }
        sum += place * *num as u64;

        place *= 10;
    }
    sum
}
