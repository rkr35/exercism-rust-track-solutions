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
        self.scores().iter().max().copied()
    }

    pub fn personal_top_three(&self) -> Vec<u32> {
        const N: usize = 3;

        // Our Vec will hold at most N + 1 elements at a time.
        // The `+1` is for when we have a full Vec of N elements, and we insert
        // into the collection to get N + 1 elements before truncating back to N
        // elements.
        let mut top = Vec::with_capacity(N + 1);

        // Iterate over all our scores.
        for &score in self.scores {

            // Iterate over our top scores.
            for i in 0..N {

                // Is the current score greater than this top score?
                if score >= top.get(i).copied().unwrap_or(0) {

                    // Place the current score into the position formerly occupied
                    // by that top score.
                    top.insert(i, score);

                    // Snip off any excess element that got pushed right.
                    // We want to truncate per update because the cost of inserting into the collection
                    // is O(top.len()). If we truncated only once outside the loops before returning, 
                    // then we're needlessly paying the cost of shifting to the right elements we don't care about.
                    top.truncate(N);

                    // Since we just added the current score into `top`, there's nothing more for us to do with this
                    // current score.
                    break;
                }
            }
        }

        top
    }
}
