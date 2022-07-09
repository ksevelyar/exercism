fn transform(num: u64, count: u64) -> Option<u64> {
    match num {
        1 => return Some(count),

        num if num % 2 == 0 => transform(num / 2, count + 1),

        num => match num.checked_mul(3)?.checked_add(1) {
            None => None,
            Some(int) => transform(int, count + 1),
        },
    }
}

pub fn collatz(n: u64) -> Option<u64> {
    transform(n, 0)
}
