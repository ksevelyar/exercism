fn is_divisible_by_any_factor(num: &u32, factors: &[u32]) -> bool {
    factors.iter().any(|factor| match factor {
        0 => false,
        _ => num % factor == 0,
    })
}

pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    (1..limit)
        .filter(|num| is_divisible_by_any_factor(num, factors))
        .sum()
}
