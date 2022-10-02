#![allow(dead_code, unused_imports)]

mod medium;

fn main() {
    dbg!(medium::luhn::is_valid("  *  "));
    dbg!(medium::luhn::is_valid("1 "));
    dbg!(medium::luhn::is_valid("1🐗"));
    dbg!(medium::luhn::is_valid("4539 3195 0343 6467"));
}
