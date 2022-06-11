fn main() {
    println!("{}", longest_chain(1_000_000));
}

fn longest_chain(limit: usize) -> usize {
    let mut max: usize = 0;
    let mut max_index: usize = 0;

    for i in 1..limit {
        let len = chain_length(i);
        if len > max {
            max = len;
            max_index = i;
        }
    }

    return max_index
}

fn chain_length(n: usize) -> usize {
    let mut val = n;
    let mut index = 1;
    while val != 1 {
        match (val % 2) == 0 {
            true => {
                val /= 2;
            }
            false => {
                val *= 3;
                val += 1;
            }
        }
        index += 1;
    }

    return index;
}
