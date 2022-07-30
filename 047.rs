use math::factor::*;
use std::collections::{HashMap, HashSet};

pub fn pe47() {
    let mut i = 1;
    let nums = loop {
        let nums = vec![i, i + 1, i + 2, i + 3];
        if n_distinct_factors(&nums, 4) {
            break nums;
        }
        i += 1;
    };

    println!("{:?}", nums);
}

pub fn n_distinct_factors(nums: &Vec<u64>, n: usize) -> bool {
    let mut fact_list = Vec::new();
    for num in nums {
        let mut fact_map: HashMap<u64, u64> = HashMap::new();
        for fact in prime_factors(*num) {
            if let Some(counter) = fact_map.get_mut(&fact) {
                *counter += 1;
            } else {
                fact_map.insert(fact, 1);
            }
        }
        if fact_map.len() == n {
            fact_list.push(fact_map);
        } else {
            return false;
        }
    }

    let mut tuples = HashSet::new();

    for fact_map in fact_list {
        for (n, c) in fact_map.iter() {
            if tuples.contains(&(*n, *c)) {
                return false;
            } else {
                tuples.insert((*n, *c));
            }
        }
    }

    true
}
