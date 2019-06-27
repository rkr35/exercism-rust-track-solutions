#![warn(clippy::pedantic)]
use std::collections::HashMap;
use std::iter::repeat;

fn is_nucleotide(c: char) -> bool {
    ['A', 'C', 'G', 'T'].contains(&c)
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
    let counts = ['A', 'C', 'G', 'T'].iter().cloned().zip(repeat(0)).collect();

    dna.chars().try_fold(counts, |mut counts : Counts, c| { 
        if is_nucleotide(c) { Ok({ *counts.get_mut(&c).unwrap() += 1; counts }) }
        else { Err(c) }
    })
}
