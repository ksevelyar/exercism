pub fn build_proverb(list: &[&str]) -> String {
    list.iter()
        .enumerate()
        .map(|(index, item)| match list.get(index + 1) {
            Some(next_item) => format!("For want of a {item} the {next_item} was lost."),
            None => format!("And all for the want of a {}.", list[0]),
        })
        .collect::<Vec<String>>()
        .join("\n")
}
