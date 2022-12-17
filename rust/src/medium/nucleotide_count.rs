use std::collections::HashMap;

const NUCLEOTIDES: [char; 4] = ['A', 'C', 'G', 'T'];

pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {
    unimplemented!(
        "How much of nucleotide type '{}' is contained inside DNA string '{}'?",
        nucleotide,
        dna
    );
}

pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {
    Ok(dna.chars().fold(HashMap::new(), |mut char_counts, ch| {
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
