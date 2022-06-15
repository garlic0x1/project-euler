fn main() {
    println!("{}", sum_amicables(10_000));

    assert_eq!(d(284), 220);
    assert_eq!(d(220), 284);
    assert_eq!(is_amicable(220, 284), true);
}

fn sum_amicables(limit: u32) -> u32 {
    let mut sum = 0;
    for i in 1..limit {
        if is_amicable(i, d(i)) {
            sum += i;
        }
    }
    sum
}

fn is_amicable(a: u32, b: u32) -> bool {
    a != b && d(a) == b && d(b) == a
}

fn d(n: u32) -> u32 {
    let mut sum = 0;
    for i in proper_divisors(n).iter() {
        sum += i;
    }
    sum
}

fn proper_divisors(n: u32) -> Vec<u32> {
    let mut vec = vec![1];
    let mut i = 2;
    loop {
        if n % i == 0 && i < n / i {
            vec.push(i);
            vec.push(n/i);
        } else if n % i == 0 && i == n / i {
            vec.push(i);
        }
        if i >= n / i {
            break vec;
        }
        i += 1;
    }
}
