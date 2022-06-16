fn main() {
    let coins = vec![1, 2, 5, 10, 20, 50, 100, 200];
    println!("{}", paths_to_sum(&coins, 200));
}

fn paths_to_sum(coins: &Vec<u32>, sum: u32) -> u32 {
    let mut paths = 0;
    if let Some(coin) = coins.last() {
        let mut collected = 0;
        while collected < sum {
            let slice_vec = &coins[..(coins.len()-1)].to_vec();
            paths += paths_to_sum(slice_vec, sum - collected);
            collected += coin;
        }
        if collected == sum {
            return paths + 1;
        }
        if collected > sum {
            return paths;
        }
    }
    return 0;
}
