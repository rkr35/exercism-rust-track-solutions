#![warn(clippy::pedantic)]
#![allow(clippy::must_use_candidate)]

pub struct HighScores<'sc> {
    scores: &'sc [u32],
}

impl<'sc> HighScores<'sc> {
    pub fn new(scores: &[u32]) -> HighScores {
        HighScores { scores }
    }

    pub fn scores(&self) -> &[u32] {
        self.scores
    }

    pub fn latest(&self) -> Option<u32> {
        self.scores.last().copied()
    }

    pub fn personal_best(&self) -> Option<u32> {
        self.personal_top_three().first().copied()
    }

    pub fn personal_top_three(&self) -> Vec<u32> {
        let mut sorted = self.scores().to_vec();
        sorted.sort_unstable_by(|a, b| b.cmp(a));
        sorted.truncate(3);
        sorted
    }
}
