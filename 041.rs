/*

We shall say that an n-digit number is pandigital if it makes use of all the digits 1 to n exactly once.
For example, 2143 is a 4-digit pandigital and is also prime.

What is the largest n-digit pandigital prime that exists?

*/

use permute::permutations_of;

fn main() {
    let mut highest_prime = 0;
    let mut set: Vec<u64> = vec![1, 2, 3, 4, 5, 6, 7];
    while set.len() > 1 {
        if set.iter().map(|n| n.clone()).sum::<u64>() % 3 == 0 {
            set.pop();
            continue;
        }
        for pandigital in permutations_of(&set) {
            let mut vec: Vec<u64> = pandigital.map(|x| x.clone()).collect();
            vec.reverse();
            let n = to_integer(&vec);
            if n % 2 == 0 {
                continue;
            }

            if is_prime(n) {
                if n > highest_prime {
                    highest_prime = n;
                }
            }
        }
        set.pop();
    }

    println!("highest pandigital prime: {}", highest_prime);
}

fn is_prime(n: u64) -> bool {
    for i in 2..((n as f32).sqrt().ceil() as u64) {
        if n % i == 0 {
            return false;
        }
    }
    true
}

fn to_integer(array: &Vec<u64>) -> u64 {
    let mut sum = 0;
    let mut place = 1;
    for num in array {
        sum += place * num;
        place *= 10;
    }
    sum
}
