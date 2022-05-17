use std::collections::HashMap;

pub fn can_construct_note(magazine: &[&str], note: &[&str]) -> bool {
    let mut cutout_dictionary = HashMap::new();
    for word in magazine {
        let counter = cutout_dictionary.entry(word).or_insert(0);
        *counter += 1;
    }

    let mut note_dictionary = HashMap::new();
    for word in note {
        let counter = note_dictionary.entry(word).or_insert(0);
        *counter += 1;
    }

    note_dictionary.iter().all(
        |(word, word_count_in_note)| match cutout_dictionary.get(word) {
            Some(word_count_in_cutout) => word_count_in_cutout >= word_count_in_note,
            None => false,
        },
    )
}
