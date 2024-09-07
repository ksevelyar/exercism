pub fn egg_count(display_value: u32) -> usize {
    (0..31)
        .map(|shift| ((display_value >> shift & 1) as usize))
        .sum()
}
