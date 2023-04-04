use crate::player::Player;

#[derive(Debug, Clone)]
pub struct Team {
    pub name: String,
    pub players: Vec<Player>,
    pub rating: i32,
}
impl Team {
    pub fn new(name: String) -> Team {
        Team {
            name,
            players: Vec::new(),
            rating: 1500,
        }
    }

    pub fn find_player(&self, name: String) -> Option<&Player> {
        self.players
            .iter()
            .find(|p| p.name == name)
    }
}
impl PartialEq<Vec<Player>> for Team {
    fn eq(&self, other: &Vec<Player>) -> bool {
        self.players.len() == other.len() && self.players.iter()
            .zip(other.iter()).all(|(a, b)| a.name == b.name)
    }
}
impl PartialEq<Vec<Player>> for &mut Team {
    fn eq(&self, other: &Vec<Player>) -> bool {
        self.players.len() == other.len() && self.players.iter()
            .zip(other.iter()).all(|(a, b)| a.name == b.name)
    }
}