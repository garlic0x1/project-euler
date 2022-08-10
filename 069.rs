/*

Euler's Totient function, φ(n) [sometimes called the phi function],
is used to determine the number of numbers less than n which are relatively
prime to n. For example, as 1, 2, 4, 5, 7, and 8, are all less than nine
and relatively prime to nine, φ(9)=6.

n    Relatively Prime    φ(n)    n/φ(n)
2    1            1    2
3    1,2          2    1.5
4    1,3          2    2
5    1,2,3,4      4    1.25
6    1,5          2    3
7    1,2,3,4,5,6  6    1.1666...
8    1,3,5,7      4    2
9    1,2,4,5,7,8  6    1.5
10   1,3,7,9      4    2.5

It can be seen that n=6 produces a maximum n/φ(n) for n ≤ 10.

Find the value of n ≤ 1,000,000 for which n/φ(n) is a maximum.

*/

use std::cell::Cell;

fn totient_traditional(n: u32) -> u32 {
    let mut n = n;
    let mut result = n;
    let mut i = 2;
    while i * i <= n {
        if n % i == 0 {
            while n % i == 0 {
                n /= i;
            }
            result -= result / i;
        }

        i += 1;
    }
    if n > 1 {
        result -= result / n;
    }
    return result;
}

fn totient_iterative(num: u32) -> u32 {
    let n = Cell::new(num);
    let mut result = (2..)
        .take_while(|i| i * i <= n.get())
        .filter(|i| n.get() % i == 0)
        .fold(num, |res, i| {
            let mut nn = n.get();
            while nn % i == 0 {
                nn /= i;
            }
            n.set(nn);
            res - res / i
        });

    if n.get() > 1 {
        result -= result / n.get();
    }

    result
}

fn totient(n: u32) -> u32 {
    totient_recursive(n, 2, n)
}

fn totient_recursive(n: u32, i: u32, res: u32) -> u32 {
    let mut res = res;
    if n > i * i {
        let mut f = n;
        if n % i == 0 {
            while f % i == 0 {
                f /= i;
            }
            res -= res / i;
        }

        totient_recursive(f, i + 1, res)
    } else if n > 1 {
        res - res / n
    } else {
        res
    }
}

pub fn pe69() {
    let sol = (0..1_000_000)
        .enumerate()
        .map(|x| (x.0, x.1 as f32 / totient(x.1) as f32))
        .fold((0, 0.0), |max, (i, x)| if x > max.1 { (i, x) } else { max });

    println!("{:?}", sol);
}
