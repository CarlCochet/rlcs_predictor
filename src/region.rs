use crate::team::Team;
use crate::player::Player;
// use crate::series::Series;

#[derive(Debug, Clone)]
pub struct Region {
    pub name: String,
    pub teams: Vec<Team>,
    pub rating: i32,
}
impl Region {
    pub fn new(name: String) -> Region {
        Region {
            name,
            teams: Vec::new(),
            rating: 1500,
        }
    }

    pub fn get_team(&self, name: String, players: &Vec<Player>) -> Option<usize> {
        if let Some(index) = self.teams.iter().position(|t| t == players) {
            return Some(index);
        }
        None
    }

    pub fn get_teams_mut(
        &mut self,
        players1: &Vec<Player>,
        players2: &Vec<Player>
    ) -> Option<(&mut Team, &mut Team)> {
        let idx1 = self.teams.iter().position(|t| t == players1)?;
        let (left, right) = self.teams.split_at_mut(idx1);
        let (team1, right) = right.split_first_mut()?;
        if let Some(team2) = right.iter_mut().find(|t| t == players2) {
            Some((team1, team2))
        } else {
            let team2 = left.iter_mut().find(|t| t == players2)?;
            Some((team2, team1))
        }
    }

    pub fn fill_team(&mut self, name: String, players: &Vec<Player>) {
        if self.teams.iter().position(|t| t == players).is_none() {
            let mut team = Team::new(name.clone());
            for player in players.iter() {
                team.players.push(player.clone());
            }
            team.update_rating();
            self.teams.push(team);
        }
    }

    pub fn find_player(&self, name: String) -> Option<&Player> {
        for team in self.teams.iter() {
            if let Some(p) = team.find_player(name.clone()) {
                return Some(p);
            }
        }
        None
    }
}