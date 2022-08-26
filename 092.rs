/*

A number chain is created by continuously adding the square of the
digits in a number to form a new number until it has been seen before.

For example,

44 → 32 → 13 → 10 → 1 → 1
85 → 89 → 145 → 42 → 20 → 4 → 16 → 37 → 58 → 89

Therefore any chain that arrives at 1 or 89 will become stuck in an endless loop.
What is most amazing is that EVERY starting number will eventually arrive at 1 or 89.

How many starting numbers below ten million will arrive at 89?

*/

use math::digits::LEndian;

pub fn pe92() {
    let limit = 10_000_000;
    let mut chains = (0..=limit).map(|i| next(i)).collect::<Vec<usize>>();

    untangle(&mut chains, limit);

    let sol = chains.iter().filter(|&n| *n == 89).count();
    println!("{sol}");
}

fn untangle(chains: &mut Vec<usize>, limit: usize) {
    (1..=limit).for_each(|i| loop {
        match chains[i] {
            1 | 89 => break,
            pointer => chains[i] = chains[pointer],
        }
    });
}

fn next(n: usize) -> usize {
    LEndian::new(n as u64, 10)
        .map(|x| x as usize * x as usize)
        .sum()
}
