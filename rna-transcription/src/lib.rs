#![warn(clippy::pedantic)]

#[derive(Debug, PartialEq)]
pub struct DNA(String);

#[derive(Debug, PartialEq)]
pub struct RNA(String);

type Nucleotides = [char; 4];
const DNA_NUCLEOTIDES: Nucleotides = ['G', 'C', 'T', 'A'];
const RNA_NUCLEOTIDES: Nucleotides = ['C', 'G', 'A', 'U'];

fn new(s: &str, nucleotides: Nucleotides) -> Result<String, usize> {
    s.chars().enumerate().try_fold(String::new(), |mut string, (index, current_char)| {
        if nucleotides.contains(&current_char) { string.push(current_char); Ok(string) }
        else { Err(index) }
    })
}

impl DNA {
    pub fn new(dna: &str) -> Result<Self, usize> {
        new(dna, DNA_NUCLEOTIDES).map(Self)
    }

    pub fn into_rna(self) -> RNA {
        let index = |dna_nucleotide| DNA_NUCLEOTIDES.iter().position(|&c| c == dna_nucleotide).unwrap();
        RNA(self.0.chars().map(|c| RNA_NUCLEOTIDES[index(c)]).collect())
    }
}

impl RNA {
    pub fn new(rna: &str) -> Result<Self, usize> {
        new(rna, RNA_NUCLEOTIDES).map(Self)
    }
}
