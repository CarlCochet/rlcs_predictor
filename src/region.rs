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

    pub fn find_team(&mut self, name: String) -> Option<&mut Team> {
        if let Some(index) = self.teams.iter().position(|t| t.name == name) {
            Some(&mut self.teams[index])
        } else {
            self.teams.push(Team::new(name));
            self.teams.last_mut()
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