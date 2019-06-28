#![warn(clippy::pedantic)]

#[derive(Debug, PartialEq)]
pub struct DNA(String);

#[derive(Debug, PartialEq)]
pub struct RNA;

impl DNA {
    pub fn new(dna: &str) -> Result<Self, usize> {
        dna.chars().enumerate().try_fold(String::new(), |mut dna_string, (index, current_char)| {
            if "ACGT".contains(current_char) { dna_string.push(current_char); Ok(dna_string) }
            else { Err(index) }
        }).map(Self)
    }

    pub fn into_rna(self) -> RNA {
        unimplemented!("Transform DNA {:?} into corresponding RNA", self);
    }
}

impl RNA {
    pub fn new(rna: &str) -> Result<RNA, usize> {
        unimplemented!("Construct new RNA from '{}' string. If string contains invalid nucleotides return index of first invalid nucleotide", rna);
    }
}
