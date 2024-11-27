use std::collections::{HashMap, HashSet};

pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    if factors.is_empty() || limit == 0 {
        return 0;
    }
    (factors[0] .. limit)
        .filter(|&x| factors.iter().any(|&n| n != 0 && x % n == 0))
        .collect::<HashSet<_>>()
        .iter()
        .sum()
}

pub fn keys_above (numbers: HashMap<i32, i32>, threshold: i32) -> Vec<i32> {
    numbers
        .iter().filter(|(k,v)| v > &&threshold).map(|(k,v)| k)
        .collect::<Vec<i32>> ()
}