
pub struct Player {
    pub name: String,
    pub rating: i32,
}
impl Player {
    pub fn new(name: String, rating: i32) -> Player {
        Player {
            name,
            rating,
        }
    }

    pub fn update_rating(&mut self, change: i32, score: i32, team_scores: Vec<i32>) {
        let gamma = 6;
        let team_score = team_scores.iter().sum::<i32>();
        let score_ratio = score as f32 / team_score as f32;

        if change > 0 {
            let sa = 3.0 * (score_ratio.powi(gamma) as f32 /
                ((team_scores[0] / team_score).powi(gamma) as f32 +
                (team_scores[1] / team_score).powi(gamma) as f32 +
                (team_scores[2] / team_score).powi(gamma) as f32));
            self.rating += (change as f32 * sa) as i32;
        }
        else {
            let sa = 3.0 * ((1.0 / score_ratio).powi(gamma) as f32 /
                ((1.0 / (team_scores[0] / team_score)).powi(gamma) as f32 +
                (1.0 / (team_scores[1] / team_score)).powi(gamma) as f32 +
                (1.0 / (team_scores[2] / team_score)).powi(gamma) as f32));
            self.rating -= (change.abs() as f32 * sa) as i32;
        }
    }
}