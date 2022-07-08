mod high_scores;
use high_scores::HighScores;

fn main() {
    let high_scores = HighScores::new(&[10, 30, 90, 30, 100, 20, 10, 0, 30, 40, 40, 70, 70]);

    assert_eq!(high_scores.personal_top_three(), vec![100, 90, 70]);
}
