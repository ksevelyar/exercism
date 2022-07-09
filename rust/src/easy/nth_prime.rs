fn is_prime(n: u32) -> bool {
    match n {
        0..=1 => false,
        2.. => (2..n).all(|smaller_number| n % smaller_number != 0),
    }
}

fn prime_with_count(target_primes_nth: u32, primes_count: u32, current_number: u32) -> u32 {
    let last_confirmation = primes_count + 1 == target_primes_nth;

    match (is_prime(current_number), last_confirmation) {
        (true, true) => current_number,
        (true, _) => prime_with_count(target_primes_nth, primes_count + 1, current_number + 1),
        (false, _) => prime_with_count(target_primes_nth, primes_count, current_number + 1),
    }
}

pub fn nth(n: u32) -> u32 {
    prime_with_count(n + 1, 0, 0)
}
