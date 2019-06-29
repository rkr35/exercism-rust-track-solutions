#![warn(clippy::pedantic)]

use std::collections::{BTreeMap, BTreeSet};

type Grade = u32;
type Students = BTreeSet<String>;

#[derive(Default)]
pub struct School(BTreeMap<Grade, Students>);

impl School {
    pub fn new() -> Self { Self::default() }

    pub fn add(&mut self, grade: Grade, student: &str) {
        self.0.entry(grade).or_default().insert(student.to_string());
    }

    pub fn grades(&self) -> Vec<Grade> {
        self.0.keys().cloned().collect()
    }

    pub fn grade(&self, grade: Grade) -> Option<Vec<String>> {
        self.0.get(&grade).map(|students| students.iter().cloned().collect())
    }
}
