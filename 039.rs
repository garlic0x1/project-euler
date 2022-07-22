/*

If p is the perimeter of a right angle triangle with integral length sides, {a,b,c},
 there are exactly three solutions for p = 120.

{20,48,52}, {24,45,51}, {30,40,50}

For which value of p ≤ 1000, is the number of solutions maximised?

*/

fn main() {
    println!("{}", max_solutions(1000));
}

fn max_solutions(n: u32) -> u32 {
    let mut max = 0;
    let mut ind = 0;

    for i in 0..=n {
        let val = solutions(i);
        if val > max {
            max = val;
            ind = i;
        }
    }

    ind
}

fn solutions(p: u32) -> u32 {
    let mut solutions = 0;

    for a in 1..p {
        for b in a..p - a {
            let sqared = (a * a) + (b * b);
            let c = (sqared as f32).sqrt();
            if c != c.floor() {
                continue;
            }
            if a + b + c as u32 == p {
                solutions += 1;
            }
        }
    }

    solutions
}
