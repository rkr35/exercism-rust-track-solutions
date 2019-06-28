#![warn(clippy::pedantic)]

#[derive(Debug, PartialEq)]
pub struct DNA(String);

#[derive(Debug, PartialEq)]
pub struct RNA(String);

const DNA_NUCLEOTIDES: [char; 4] = ['G', 'C', 'T', 'A'];
const RNA_NUCLEOTIDES: [char; 4] = ['C', 'G', 'A', 'U'];

impl DNA {
    pub fn new(dna: &str) -> Result<Self, usize> {
        dna.chars().enumerate().try_fold(String::new(), |mut dna_string, (index, current_char)| {
            if "GCTA".contains(current_char) { dna_string.push(current_char); Ok(dna_string) }
            else { Err(index) }
        }).map(Self)
    }

    pub fn into_rna(self) -> RNA {
        let index = |dna_nucleotide| DNA_NUCLEOTIDES.iter().position(|&c| c == dna_nucleotide).unwrap();
        RNA(self.0.chars().map(|c| RNA_NUCLEOTIDES[index(c)]).collect())
    }
}

impl RNA {
    pub fn new(rna: &str) -> Result<Self, usize> {
        rna.chars().enumerate().try_fold(String::new(), |mut rna_string, (index, current_char)| {
            if "CGAU".contains(current_char) { rna_string.push(current_char); Ok(rna_string) }
            else { Err(index) }
        }).map(Self)
    }
}
