#[derive(PartialEq, Eq)]
pub struct Dna(String);

#[derive(PartialEq, Eq)]
pub struct Rna(String);

const ACIDS: [(char, char); 4] = [('G', 'C'), ('C', 'G'), ('T', 'A'), ('A', 'U')];

impl Dna {
    pub fn new(dna: &str) -> Result<Dna, usize> {
        let dna_acids = ACIDS.map(|(dna, _rna)| dna);
        match dna.chars().position(|nucleotide| !dna_acids.contains(&nucleotide)) {
            Some(index) => Err(index),
            None => Ok(Dna(dna.to_string())),
        }
    }

    fn rna_nucleotide(dna_nucleotide: char) -> char {
        ACIDS
            .iter()
            .find(|(dna, _)| *dna == dna_nucleotide)
            .map(|(_, rna)| *rna)
            .expect("valid dna nucleotide")
    }

    pub fn into_rna(self) -> Rna {
        let rna: String = self.0.chars().map(Self::rna_nucleotide).collect();
        Rna(rna)
    }
}

impl Rna {
    pub fn new(rna: &str) -> Result<Rna, usize> {
        let rna_acids = ACIDS.map(|(_dna, rna)| rna);
        match rna.chars().position(|nucleotide| !rna_acids.contains(&nucleotide)) {
            Some(index) => Err(index),
            None => Ok(Rna(rna.to_string())),
        }
    }
}
