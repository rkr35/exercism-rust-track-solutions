use std::collections::HashMap;

pub struct CodonsInfo<'a> {
    codon_to_protein: HashMap<&'a [u8], &'a str>,
}

impl<'a> CodonsInfo<'a> {
    fn name_for_bytes(&self, codon: &[u8]) -> Option<&'a str> {
        self.codon_to_protein.get(codon).copied()
    }
    
    pub fn name_for(&self, codon: &str) -> Option<&'a str> {
        self.name_for_bytes(codon.as_bytes())
    }

    pub fn of_rna(&self, rna: &str) -> Option<Vec<&'a str>> {
        rna
            .as_bytes()
            .chunks(3)
            .map(|codon| self.name_for_bytes(codon))
            .take_while(|name| name.unwrap_or("") != "stop codon")
            .collect()
    }
}

pub fn parse<'a>(pairs: Vec<(&'a str, &'a str)>) -> CodonsInfo<'a> {
    use core::iter::FromIterator;

    CodonsInfo {
        codon_to_protein: HashMap::from_iter(pairs
            .into_iter()
            .map(|(codon, name)| (codon.as_bytes(), name))
        ),
    }
}
