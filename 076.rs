/*

It is possible to write five as a sum in exactly six different ways:

4 + 1
3 + 2
3 + 1 + 1
2 + 2 + 1
2 + 1 + 1 + 1
1 + 1 + 1 + 1 + 1

How many different ways can one hundred be written as a sum of at least two positive integers?

*/

pub fn pe76() {
    println!("{}", combos_rec(100, 99))
}

fn combos_rec(n: u64, last: u64) -> u64 {
    if n == 0 {
        1
    } else {
        (1..=std::cmp::min(n, last))
            .map(|x| combos_rec(n - x, x))
            .sum::<u64>()
    }
}
