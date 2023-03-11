pub struct CodonsInfo<'a> {
    pairs: Vec<(&'a str, &'a str)>,
}

impl<'a> CodonsInfo<'a> {
    pub fn name_for(&self, codon: &str) -> Option<&'a str> {
        self.pairs
            .iter()
            .find(|(stored_codon, _protein)| codon == *stored_codon)
            .map(|(_, protein)| *protein)
    }

    pub fn of_rna(&self, rna: &str) -> Option<Vec<&'a str>> {
        let codons_count = match rna.len() % 3 {
            0 => rna.len() / 3,
            _ => rna.len() / 3 + 1,
        };

        let codons: Option<Vec<&str>> = (0..codons_count)
            .map(|ind| rna.get((ind * 3)..(ind * 3 + 3)))
            .take_while(|codon| self.name_for(codon.unwrap_or("")) != Some("stop codon"))
            .collect();

        codons?.iter().map(|codon| self.name_for(codon)).collect()
    }
}

pub fn parse<'a>(pairs: Vec<(&'a str, &'a str)>) -> CodonsInfo<'a> {
    CodonsInfo { pairs }
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

    #[test]
    fn test_translates_rna_strand_into_correct_protein() {
        let info = parse(make_pairs());

        assert_eq!(
            info.of_rna("AUGUUUUGG"),
            Some(vec!["methionine", "phenylalanine", "tryptophan"])
        );
    }

    #[test]
    fn test_stops_translation_if_stop_codon_present() {
        let info = parse(make_pairs());

        assert_eq!(
            info.of_rna("AUGUUUUAA"),
            Some(vec!["methionine", "phenylalanine"])
        );
    }

    #[test]
    fn test_invalid_length() {
        let info = parse(make_pairs());

        assert!(info.of_rna("AUGUA").is_none());
    }

    #[test]
    fn test_valid_stopped_rna() {
        let info = parse(make_pairs());

        assert_eq!(info.of_rna("AUGUAAASDF"), Some(vec!["methionine"]));
    }
}
