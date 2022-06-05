fn main() {
    println!("{} {}", square_sum(10), sum_square(10));
    println!("{}", diff_sums(100));
}

fn diff_sums(limit: i64) -> i64 {
    square_sum(limit) - sum_square(limit)
}

fn sum_square(limit: i64) -> i64 {
    let mut sum = 0;
    for i in 1..=limit {
        sum += i * i;
    }
    return sum;
}

fn square_sum(limit: i64) -> i64 {
    let mut sum = 0;
    for i in 1..=limit {
        sum += i;
    }
    return sum * sum;
}
