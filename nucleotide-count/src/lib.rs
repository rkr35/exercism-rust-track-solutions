#![warn(clippy::pedantic)]
use std::collections::HashMap;

fn is_nucleotide(c: char) -> bool {
    match c {
        'A' | 'C' | 'G' | 'T' => true,
        _ => false,
    }
}

pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {
    if !is_nucleotide(nucleotide) { return Err(nucleotide); }

    dna.chars().try_fold(0, |count, c| {
        if is_nucleotide(c) { Ok( count + usize::from(c == nucleotide) ) }
        else { Err(c) }
    })
}

pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {
    unimplemented!(
        "How much of every nucleotide type is contained inside DNA string '{}'?",
        dna
    );
}
