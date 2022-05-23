pub fn square(s: u32) -> u64 {
    match s {
        1..=64 => (1..s).fold(1, |grains, _| grains * 2),
        _ => panic!("Square must be between 1 and 64"),
    }
}

pub fn total() -> u64 {
    (1..=64).fold(0, |total_grains, current_square| total_grains + square(current_square))
}
