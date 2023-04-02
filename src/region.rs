use crate::team::Team;
use crate::player::Player;

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

    pub fn get_team(&self, name: String, players: &Vec<Player>) -> Option<&Team> {
        if let Some(index) = self.teams.iter().position(|t| t.name == name) {
            return Some(&self.teams[index]);
        } else if let Some(index) = self.teams.iter()
            .position(|t| t.players.len() == players.len() && t.players.iter()
                .zip(players.iter()).all(|(a, b)| a.name == b.name)) {
            return Some(&self.teams[index]);
        }
        None
    }

    pub fn fill_team(&mut self, name: String, players: &Vec<Player>) {
        // if the name is not found, and the players are not found, add a new team
        if self.teams.iter().position(|t| t.name == name).is_none() &&
            self.teams.iter().position(|t| t.players.len() == players.len() && t.players.iter()
                .zip(players.iter()).all(|(a, b)| a.name == b.name)).is_none() {
            let mut team = Team::new(name.clone());
            for player in players.iter() {
                team.players.push(player.clone());
            }
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