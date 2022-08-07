/*

A spider, S, sits in one corner of a cuboid room, measuring 6 by 5 by 3, and a fly,
F, sits in the opposite corner. By travelling on the surfaces of the room the shortest
"straight line" distance from S to F is 10 and the path is shown on the diagram.


However, there are up to three "shortest" path candidates for any given cuboid and the
shortest route doesn't always have integer length.

It can be shown that there are exactly 2060 distinct cuboids, ignoring rotations, with
integer dimensions, up to a maximum size of M by M by M, for which the shortest route
has integer length when M = 100. This is the least value of M for which the number of
solutions first exceeds two thousand; the number of solutions when M = 99 is 1975.

Find the least value of M such that the number of solutions first exceeds one million.

*/

pub fn pe86() {
    for i in 1800.. {
        let nints = integers(i);
        println!("{i} : {nints}");
        if nints > 1_000_000 {
            println!("{i}");
            break;
        }
    }
}

fn integers(max: u32) -> u32 {
    let mut sum = 0;
    for a in 1..=max {
        for b in a..=max {
            for c in b..=max {
                if is_integer_solution(a, b, c) {
                    sum += 1;
                }
            }
        }
    }
    sum
}

fn is_integer_solution(a: u32, b: u32, c: u32) -> bool {
    let mut shortest = (a + c) * (a + c) + b * b;
    let candidate2 = (a + b) * (a + b) + c * c;
    if candidate2 < shortest {
        shortest = candidate2;
    }
    let candidate3 = (b + c) * (b + c) + a * a;
    if candidate3 < shortest {
        shortest = candidate3;
    }

    is_perfect_square(shortest)
}

fn is_perfect_square(n: u32) -> bool {
    let f = n as f32;
    let sqrt = f.sqrt().floor() as u32;
    sqrt * sqrt == n
}
