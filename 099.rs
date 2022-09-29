/*

Comparing two numbers written in index form
like 211 and 37 is not difficult, as any calculator
would confirm that 211 = 2048 < 37 = 2187.

However, confirming that 632382518061 > 519432525806
would be much more difficult, as both numbers contain
over three million digits.

Using base_exp.txt (right click and 'Save Link/Target As...'),
a 22K text file containing one thousand lines with a
base/exponent pair on each line, determine which line
number has the greatest numerical value.

NOTE: The first two lines in the file represent the
numbers in the example given above.

*/

pub fn pe99() {
    let exps = std::fs::read_to_string("p099_base_exp.txt").unwrap();

    let sol = exps
        .lines()
        // map strings to exponents
        .map(|line| {
            let mut split = line.split(',');
            (
                split.next().unwrap().parse::<f64>().unwrap(),
                split.next().unwrap().parse::<f64>().unwrap(),
            )
        })
        .enumerate()
        .fold((0, (1f64, 1f64)), |(sol, max), (index, exp)| {
            if exp.1 * exp.0.log10() > max.1 * max.0.log10() {
                (index + 1, exp)
            } else {
                (sol, max)
            }
        });

    println!("{}", sol.0);
}
