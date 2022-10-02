fn extract(divisor: u64, remainder: u64, mut factors: Vec<u64>) -> Vec<u64> {
    match remainder {
        1 => factors,
        remainder if remainder % divisor == 0 => {
            factors.push(divisor);
            extract(divisor, remainder / divisor, factors)
        }
        _ => extract(divisor + 1, remainder, factors),
    }
}

pub fn factors(n: u64) -> Vec<u64> {
    extract(2, n, Vec::new())
}
