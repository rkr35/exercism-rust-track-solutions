#![warn(clippy::all)]
#![warn(clippy::pedantic)]

pub struct Allergies {
    score: u32,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Allergen {
    Eggs            = 0,
    Peanuts         = 1,
    Shellfish       = 2,
    Strawberries    = 3,
    Tomatoes        = 4,
    Chocolate       = 5,
    Pollen          = 6,
    Cats            = 7,
}

impl Allergies {
    pub fn new(score: u32) -> Self {
        Self { score }
    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        let allergen = *allergen as u8;
        self.score >> allergen & 1 == 1
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        unimplemented!("Return the list of allergens contained within the score with which the Allergies struct was made.");
    }
}
