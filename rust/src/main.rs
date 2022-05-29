mod matching_brackets;
mod raindrops;

fn main() {
    dbg!(matching_brackets::brackets_are_balanced("([][])"));
    dbg!(matching_brackets::brackets_are_balanced("(}{)"));

    dbg!(raindrops::raindrops(30));
    dbg!(raindrops::raindrops(34));
}
