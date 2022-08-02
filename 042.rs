/*

The nth term of the sequence of triangle numbers is given by, tn = ½n(n+1); so the first ten triangle numbers are:

1, 3, 6, 10, 15, 21, 28, 36, 45, 55, ...

By converting each letter in a word to a number corresponding to its alphabetical
position and adding these values we form a word value. For example, the word value
for SKY is 19 + 11 + 25 = 55 = t10. If the word value is a
triangle number then we shall call the word a triangle word.

Using words.txt (right click and 'Save Link/Target As...'),
a 16K text file containing nearly two-thousand common English words, how many are triangle words?

*/

use std::{collections::HashSet, fs::read_to_string};

pub fn pe42() {
    let text = read_to_string("p042_words.txt").unwrap();
    let set = triangle_set(20);

    let c = text
        .split(',')
        .map(|x| x.replace('"', ""))
        .map(|x| word_value(&x))
        .filter(|x| set.contains(x))
        .count();

    println!("{}", c);
}

fn word_value(word: &str) -> u32 {
    let mut sum = 0;
    for c in word.chars() {
        sum += c as u32 - 64;
    }
    sum
}

fn triangle_set(limit: u32) -> HashSet<u32> {
    let mut set = HashSet::new();

    for n in 1..limit {
        set.insert((n * (n + 1)) / 2);
    }

    set
}
