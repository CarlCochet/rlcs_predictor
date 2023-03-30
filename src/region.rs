use crate::team::Team;
use crate::player::Player;

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
        self.teams
            .iter_mut()
            .find(|t| t.name == name)
            .or_else(|| {
                self.teams.push(Team::new(name.clone()));
                self.teams.last_mut()
            })
    }

    pub fn find_player(&mut self, name: String) -> Option<&mut Player> {
        for team in self.teams.iter_mut() {
            if let Some(p) = team.find_player(name.clone()) {
                return Some(p);
            }
        }
        None
    }
}