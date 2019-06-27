#![warn(clippy::pedantic)]
use std::collections::HashMap;
use std::iter::repeat;

const NUCLEOTIDES: &str = "ACGT";

fn is_nucleotide(c: char) -> bool {
    NUCLEOTIDES.contains(c)
}

pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {
    if !is_nucleotide(nucleotide) { return Err(nucleotide); }

    dna.chars().try_fold(0, |count, c| {
        if is_nucleotide(c) { Ok(count + usize::from(c == nucleotide)) } 
        else { Err(c) }
    })
}

type Counts = HashMap<char, usize>;
pub fn nucleotide_counts(dna: &str) -> Result<Counts, char> {
    let mut counts: Counts = NUCLEOTIDES.chars().zip(repeat(0)).collect();
    dna.chars()
        .try_for_each(|c| counts.get_mut(&c).map(|count| *count += 1).ok_or(c))
        .map(|()| counts)
}
