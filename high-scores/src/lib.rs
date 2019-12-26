#[derive(Debug)]
pub struct HighScores<'sc> {
    scores: &'sc [u32],
}

impl<'sc> HighScores<'sc> {
    pub fn new(scores: &[u32]) -> HighScores {
        HighScores {
            scores,
        }
    }

    pub fn scores(&self) -> &[u32] {
        self.scores
    }

    pub fn latest(&self) -> Option<u32> {
        self.scores.last().copied()
    }

    pub fn personal_best(&self) -> Option<u32> {
        unimplemented!("Return the highest score")
    }

    pub fn personal_top_three(&self) -> Vec<u32> {
        unimplemented!("Return 3 highest scores")
    }
}
