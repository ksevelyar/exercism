use std::{collections::HashMap, iter::repeat};

const NUCLEOTIDES: [char; 4] = ['A', 'C', 'G', 'T'];

fn is_nucleotide(dna: &str) -> Result<(), char> {
    if let Some(ch) = dna.chars().find(|ch| !NUCLEOTIDES.contains(ch)) {
        return Err(ch);
    }

    Ok(())
}

pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {
    is_nucleotide(dna)?;
    is_nucleotide(&nucleotide.to_string())?;

    Ok(dna.chars().filter(|&ch| ch == nucleotide).count())
}

pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {
    is_nucleotide(dna)?;

    let acc: HashMap<char, usize> = NUCLEOTIDES.iter().copied().zip(repeat(0usize)).collect();

    Ok(dna.chars().fold(acc, |mut char_counts, ch| {
        let char = char_counts.entry(ch).or_insert(0);
        *char += 1;

        char_counts
    }))
}

#[test]
fn test_strand_with_multiple_nucleotides() {
    assert_eq!(
        nucleotide_counts("AGCTTTTCATTCTGACTGCAACGGGCAATATGTCTCTGTGTGGATTAAAAAAAGAGTGTCTGATAGCAGC")
            .unwrap(),
        [('A', 20), ('T', 21), ('C', 12), ('G', 17)]
            .into_iter()
            .collect::<HashMap<_, _>>()
    );
}

#[test]
fn test_strand_with_invalid_nucleotides() {
    assert_eq!(nucleotide_counts("AGXXACT"), Err('X'));
}
