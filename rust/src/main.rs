mod medium;

fn main() {
    use medium::clock::Clock;

    dbg!(Clock::new(8, 0).to_string());
    dbg!(Clock::new(0, 3).add_minutes(-4).to_string());
}
