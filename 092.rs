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
    let mut chains = vec![0usize; limit + 1];

    chains
        .iter_mut()
        .enumerate()
        .for_each(|(i, n)| *n = next(i));

    untangle(&mut chains, limit);

    let sol = &chains[1..limit].iter().filter(|&n| *n == 89).count();
    println!("{sol}");
}

fn untangle(chains: &mut Vec<usize>, limit: usize) {
    if (1..=limit).fold(false, |tangled, i| {
        let pointer = chains[i];
        if pointer != 1 && pointer != 89 {
            if let Some(&val) = chains.get(pointer) {
                chains[i] = val;
                return true;
            }
        }
        tangled
    }) {
        untangle(chains, limit);
    }
}

fn next(n: usize) -> usize {
    LEndian::new(n as u64, 10)
        .map(|x| x as usize * x as usize)
        .sum()
}
