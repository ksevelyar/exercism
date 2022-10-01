mod medium;

fn main() {
    dbg!(medium::luhn::is_valid("  *  "));
    dbg!(medium::luhn::is_valid("1234  5678"));
}
