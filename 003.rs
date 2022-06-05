fn main() {
    println!("{:?}",largest_prime(600851475143) );
}

fn largest_prime(n: i64) -> i64 {
    let mut i = 2;
    while i <= (n / 2) {
        if n % i == 0 {
            return largest_prime(n/i);
        }
        i += 1;
    }
    return n;
}
