pub struct HighScores<'a>{
    scores: &'a [u32]
}

impl<'a> HighScores<'a> {
    pub fn new(scores: &'a [u32]) -> Self {
        HighScores { scores }
    }

    pub fn scores(&self) -> &[u32] {
        self.scores
    }

    pub fn latest(&self) -> Option<u32> {
        match self.scores.last() {
            Some(score) => Some(score.clone()),
            None => None
        }
    }

    pub fn personal_best(&self) -> Option<u32> {
        match self.scores.iter().max() {
            Some(score) => Some(score.clone()),
            None => None
        }
    }

    pub fn personal_top_three(&self) -> Vec<u32> {
        let mut scores = self.scores.to_vec().clone();
        scores.sort();

        scores.into_iter().rev().take(3).collect()
    }
}
