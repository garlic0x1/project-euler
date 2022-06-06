fn main() {
    sieve_of_eratosthenes();
}

fn sieve_of_eratosthenes() {
    const MAX: usize = 10_000_000;
    let mut sieve = vec![false; MAX+1];

    let limit = ((MAX-1) as f64).sqrt().floor() as usize;
    for i in 2..=limit {
        let item = sieve[i];
        if item == false {
            let mut mult: usize = i*2;
            while mult <= MAX {
                sieve[mult] = true;
                mult += i;
            }
        }
    }
    
    let mut c = 0;
    for i in 2..MAX {
        if sieve[i] == false {
        println!("{}:\t{}", i, c);
            c += 1;

            if c == 10001 {
                c = i;
                break;
            }
        }
    }

    println!("{}", c);
}
