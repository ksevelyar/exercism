pub struct CodonsInfo<'a> {
    // We fake using 'a here, so the compiler does not complain that
    // "parameter `'a` is never used". Delete when no longer needed.
    phantom: std::marker::PhantomData<&'a ()>,
}

impl<'a> CodonsInfo<'a> {
    pub fn name_for(&self, codon: &str) -> Option<&'a str> {
        unimplemented!(
            "Return the protein name for a '{codon}' codon or None, if codon string is invalid"
        );
    }

    pub fn of_rna(&self, rna: &str) -> Option<Vec<&'a str>> {
        unimplemented!("Return a list of protein names that correspond to the '{rna}' RNA string or None if the RNA string is invalid");
    }
}

pub fn parse<'a>(pairs: Vec<(&'a str, &'a str)>) -> CodonsInfo<'a> {
    unimplemented!("Construct a new CodonsInfo struct from given pairs: {pairs:?}");
}

mod tests {
    use super::*;

    fn make_pairs() -> Vec<(&'static str, &'static str)> {
        let grouped = vec![
            ("isoleucine", vec!["AUU", "AUC", "AUA"]),
            ("valine", vec!["GUU", "GUC", "GUA", "GUG"]),
            ("phenylalanine", vec!["UUU", "UUC"]),
            ("methionine", vec!["AUG"]),
            ("cysteine", vec!["UGU", "UGC"]),
            ("alanine", vec!["GCU", "GCC", "GCA", "GCG"]),
            ("glycine", vec!["GGU", "GGC", "GGA", "GGG"]),
            ("proline", vec!["CCU", "CCC", "CCA", "CCG"]),
            ("threonine", vec!["ACU", "ACC", "ACA", "ACG"]),
            ("serine", vec!["AGU", "AGC"]),
            ("tyrosine", vec!["UAU", "UAC"]),
            ("tryptophan", vec!["UGG"]),
            ("glutamine", vec!["CAA", "CAG"]),
            ("asparagine", vec!["AAU", "AAC"]),
            ("histidine", vec!["CAU", "CAC"]),
            ("glutamic acid", vec!["GAA", "GAG"]),
            ("aspartic acid", vec!["GAU", "GAC"]),
            ("lysine", vec!["AAA", "AAG"]),
            ("arginine", vec!["CGU", "CGC", "CGA", "CGG", "AGA", "AGG"]),
            ("stop codon", vec!["UAA", "UAG", "UGA"]),
        ];

        let mut pairs = Vec::<(&'static str, &'static str)>::new();

        for (name, codons) in grouped.into_iter() {
            for codon in codons {
                pairs.push((codon, name));
            }
        }

        pairs.sort_by(|&(_, a), &(_, b)| a.cmp(b));

        pairs
    }

    #[test]
    fn test_methionine() {
        let info = parse(make_pairs());
        assert_eq!(info.name_for("AUG"), Some("methionine"));
    }
}
