#![allow(dead_code, unused_imports)]

mod medium;

fn main() {
    let string = "AND + A + STRONG + OFFENSE + AS + A + GOOD == DEFENSE";

    println!("{:?}", medium::alphametics::solve(string));
}
