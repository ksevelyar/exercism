fn square_of_sum(n: u32) -> u32 {
    (0..n)
        .enumerate()
        .map(|(i, _)| i as u32 + 1)
        .sum::<u32>()
        .pow(2)
}

fn sum_of_squares(n: u32) -> u32 {
    (0..n)
        .enumerate()
        .map(|(i, _)| (i as u32 + 1).pow(2))
        .sum::<u32>()
}

pub fn difference(n: u32) -> u32 {
    square_of_sum(n) - sum_of_squares(n)
}
