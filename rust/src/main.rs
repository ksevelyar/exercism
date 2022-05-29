mod matching_brackets;

fn main() {
    dbg!(matching_brackets::brackets_are_balanced("([][])"));
    dbg!(matching_brackets::brackets_are_balanced("(}{)"));
}
