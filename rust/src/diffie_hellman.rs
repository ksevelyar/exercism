pub fn modular_exponentiation(num: u64, pow: u64, div: u64) -> u64 {
    (1..=pow).fold(1, |acc, _| (num * acc) % div)
}
