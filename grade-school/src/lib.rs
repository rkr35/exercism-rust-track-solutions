#![warn(clippy::pedantic)]

type Grade = u32;
type Students = Vec<String>;

#[derive(Default)]
pub struct School(std::collections::HashMap<Grade, Students>);

impl School {
    pub fn new() -> Self { Self::default() }

    pub fn add(&mut self, grade: Grade, student: &str) {
        self.0.entry(grade).or_insert_with(Students::new).push(student.to_string());
    }

    pub fn grades(&self) -> Vec<Grade> {
        let mut g: Vec<_> = self.0.keys().cloned().collect(); g.sort_unstable();
        g
    }

    pub fn grade(&self, grade: Grade) -> Option<Students> {
        self.0.get(&grade).cloned().map(|mut s| { s.sort_unstable(); s })
    }
}
