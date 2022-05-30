mod matching_brackets;
mod proverb;

fn main() {
    dbg!(matching_brackets::brackets_are_balanced("([][])"));
    dbg!(matching_brackets::brackets_are_balanced("(}{)"));

    dbg!(proverb::build_proverb(&[
        "nail", "shoe", "horse", "rider", "message", "battle", "kingdom"
    ]));
}
