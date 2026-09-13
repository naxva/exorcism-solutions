#[derive(Debug)]
pub struct HighScores<'slice> {
    scores: &'slice [u32],
}

impl<'slice> HighScores<'slice> {
    pub fn new(scores: &'slice [u32]) -> Self {
        Self { scores }
    }

    pub fn scores(&self) -> &[u32] {
        self.scores
    }

    pub fn latest(&self) -> Option<u32> {
        self.scores.last().copied()
    }

    pub fn personal_best(&self) -> Option<u32> {
        self.scores.iter().max().copied()
    }

    pub fn personal_top_three(&self) -> Vec<u32> {
        let mut v = self.scores.to_vec();
        v.sort_unstable_by(|a, b| b.cmp(a));
        v.truncate(3);
        v
    }
}
