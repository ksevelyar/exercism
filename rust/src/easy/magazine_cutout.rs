use std::collections::HashMap;

fn counts<'a>(words: &'a [&str]) -> HashMap<&'a str, u32> {
    words.iter().fold(HashMap::new(), |mut counts, word| {
        counts
            .entry(word)
            .and_modify(|count| *count += 1)
            .or_insert(0);
        counts
    })
}

pub fn can_construct_note(magazine: &[&str], note: &[&str]) -> bool {
    let cutout_dictionary = counts(magazine);
    let note_dictionary = counts(note);

    note_dictionary.iter().all(
        |(word, word_count_in_note)| match cutout_dictionary.get(word) {
            Some(word_count_in_cutout) => word_count_in_cutout >= word_count_in_note,
            None => false,
        },
    )
}
