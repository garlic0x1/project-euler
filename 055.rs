/*

If we take 47, reverse and add, 47 + 74 = 121, which is palindromic.

Not all numbers produce palindromes so quickly. For example,

349 + 943 = 1292,
1292 + 2921 = 4213
4213 + 3124 = 7337

That is, 349 took three iterations to arrive at a palindrome.

Although no one has proved it yet, it is thought that some numbers, like 196,
never produce a palindrome. A number that never forms a palindrome through the
reverse and add process is called a Lychrel number. Due to the theoretical
nature of these numbers, and for the purpose of this problem, we shall assume
that a number is Lychrel until proven otherwise. In addition you are given
that for every number below ten-thousand, it will either (i) become a palindrome
in less than fifty iterations, or, (ii) no one, with all the computing power
that exists, has managed so far to map it to a palindrome. In fact, 10677 is the
first number to be shown to require over fifty iterations before producing
a palindrome: 4668731596684224866951378664 (53 iterations, 28-digits).

Surprisingly, there are palindromic numbers that are themselves Lychrel numbers; the first example is 4994.

How many Lychrel numbers are there below ten-thousand?

NOTE: Wording was modified slightly on 24 April 2007 to emphasise the theoretical nature of Lychrel numbers.

*/

use math::digits::BEndian;

pub fn pe55() {
    let sol = (1u128..10_000u128).filter(|x| lychrel(*x) == None).count();
    println!("{sol}");
}

fn lychrel(n: u128) -> Option<u128> {
    let mut n = n;
    for i in 1..50 {
        n = rev_add(n);
        if is_palindromic(n) {
            return Some(i as u128);
        }
    }
    None
}

fn rev_add(n: u128) -> u128 {
    BEndian::new(n, 10)
        .fold((1u128, 0u128), |(place, sum), d| {
            (place * 10, sum + d as u128 * place)
        })
        .1
        + n
}

fn is_palindromic(n: u128) -> bool {
    let digits: Vec<u8> = BEndian::new(n, 10).collect();
    for i in 0..digits.len() / 2 {
        if digits[i] != digits[digits.len() - (i + 1)] {
            return false;
        }
    }
    true
}
