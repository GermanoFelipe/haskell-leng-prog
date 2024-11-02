pub fn collatz(mut n: u64) -> Option<u64> {
    result_collatz(n, 0)
}

pub fn result_collatz(mut n: u64, mut counter: u64) -> Option<u64> {
    if n <= 0 {
        return None;
    }
    if n == 1 {
        return Some(counter)
    }
    let num = if n % 2 == 0 {n / 2} else { n*3 + 1 };
    result_collatz(num, counter + 1)
}

