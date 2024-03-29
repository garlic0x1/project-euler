/*

Pentagonal numbers are generated by the formula, Pn=n(3n−1)/2.
The first ten pentagonal numbers are:

1, 5, 12, 22, 35, 51, 70, 92, 117, 145, ...

It can be seen that P4 + P7 = 22 + 70 = 92 = P8.
However, their difference, 70 − 22 = 48, is not pentagonal.

Find the pair of pentagonal numbers, Pj and Pk, for which their
sum and difference are pentagonal and D = |Pk − Pj| is minimised;

What is the value of D?

*/

pub fn pe44() {
    let pents = pentagonals(1_000_000);

    for &diff in pents.iter() {
        for &p in pents.iter() {
            if let Ok(p2) = pents.binary_search(&(p + diff)) {
                let p2 = pents[p2];

                if let Ok(_) = pents.binary_search(&(p + p2)) {
                    println!("{diff}");
                    return;
                }
            }
        }
    }
}

fn pentagonals(limit: u32) -> Vec<u64> {
    let mut vec = Vec::new();

    for n in 1..=limit {
        let n = n as u64;
        vec.push(n * (3 * n - 1) / 2);
    }

    vec
}
