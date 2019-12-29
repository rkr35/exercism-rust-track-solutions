#![warn(clippy::pedantic)]
#![allow(clippy::must_use_candidate)]

use once_cell::sync::OnceCell;
use rand::{rngs::StdRng, SeedableRng};
use rand::distributions::{Distribution, Uniform};
use std::collections::HashSet;
use std::sync::{Arc, Mutex};

type ArcString = Arc<String>;

#[derive(Default)]
pub struct Robot {
    name: ArcString,
}

impl Robot {
    pub fn new() -> Self {
        let mut robot = Self::default();
        robot.reset_name();
        robot
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn reset_name(&mut self) {
        static GENERATOR: OnceCell<Mutex<Generator>> = OnceCell::new();

        let mut lock = GENERATOR
            .get_or_init(|| Mutex::new(Generator::new()))
            .lock()
            .expect("Encountered a poisoned lock");

        self.name = lock.generate(&self.name);
    }
}

struct Generator {
    rng: StdRng,
    letter: Uniform<u8>,
    number: Uniform<u8>,
    names_in_use: HashSet<ArcString>,
}

impl Generator {
    fn new() -> Self {
        Self {
            rng: StdRng::from_entropy(),
            names_in_use: HashSet::new(),
            letter: (b'A' ..= b'Z').into(),
            number: (b'0' ..= b'9').into(),
        }
    }

    fn get_random_name(&mut self) -> String {
        const PREFIX_LEN: usize = 2;
        const SUFFIX_LEN: usize = 3;
        let letter = &self.letter;
        let number = &self.number;
        let mut name = String::with_capacity(PREFIX_LEN + SUFFIX_LEN);
        name.extend(letter.sample_iter(&mut self.rng).take(PREFIX_LEN).map(char::from));
        name.extend(number.sample_iter(&mut self.rng).take(SUFFIX_LEN).map(char::from));
        name
    }

    fn generate(&mut self, old_name: &ArcString) -> ArcString {
        loop {
            let name = ArcString::new(self.get_random_name());
            
            if self.names_in_use.insert(ArcString::clone(&name)) {
                self.names_in_use.remove(old_name);
                return name;
            }
            
            println!("Encountered a name already in use (\"{}\"). Generating a new name.", name);
        }
    }
}