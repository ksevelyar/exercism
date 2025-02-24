use std::collections::HashMap;

pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    if input.is_empty() {
        return HashMap::new();
    }

    let batch_size = calc_batch_size(input.len(), worker_count);

    let handles = input
        .chunks(batch_size)
        .map(|chunk| {
            let chunk: Vec<String> = chunk.iter().map(|s| s.to_string()).collect();
            std::thread::spawn(move || calc_frequencies(chunk))
        })
        .map(|handle| handle.join().unwrap());

    handles.fold(HashMap::new(), |mut map, item| {
        item.iter().for_each(|(key, count)| {
            *map.entry(*key).or_insert(0) += count;
        });

        map
    })
}

fn calc_batch_size(len: usize, worker_count: usize) -> usize {
    match len / worker_count {
        0 => len,
        n => n,
    }
}

fn calc_frequencies(chunk: Vec<String>) -> HashMap<char, usize> {
    chunk.iter().fold(HashMap::new(), |mut map, item| {
        item.chars()
            .filter(|ch| ch.is_alphabetic())
            .map(|ch| ch.to_ascii_lowercase())
            .for_each(|ch| {
                map.entry(ch)
                    .and_modify(|counter| *counter += 1)
                    .or_insert(1);
            });

        map
    })
}

const STAR_SPANGLED_BANNER: [&str; 8] = [
    "O say can you see by the dawn's early light,",
    "What so proudly we hailed at the twilight's last gleaming,",
    "Whose broad stripes and bright stars through the perilous fight,",
    "O'er the ramparts we watched, were so gallantly streaming?",
    "And the rockets' red glare, the bombs bursting in air,",
    "Gave proof through the night that our flag was still there;",
    "O say does that star-spangled banner yet wave,",
    "O'er the land of the free and the home of the brave?",
];

#[test]
fn two_threads() {
    let freqs = frequency(&STAR_SPANGLED_BANNER, 2);

    assert_eq!(freqs.get(&'a'), Some(&34));

    assert_eq!(freqs.get(&'t'), Some(&38));

    assert_eq!(freqs.get(&'?'), None);
}
