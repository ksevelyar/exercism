fn sqrt(num: u64) -> u64 {
    (num as f64).sqrt() as u64 + 1
}

pub fn primes_up_to(upper_bound: u64) -> Vec<u64> {
    let marked: Vec<u64> = (2..sqrt(upper_bound))
        .flat_map(|num| {
            (num..=upper_bound)
                .step_by(num as usize)
                .filter(move |n| *n != num)
        })
        .collect();

    (2..=upper_bound)
        .filter(|num| !marked.contains(num))
        .collect()
}

#[test]
fn test10() {
    assert!(primes_up_to(10) == [2, 3, 5, 7]);
}
