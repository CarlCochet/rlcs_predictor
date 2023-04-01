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