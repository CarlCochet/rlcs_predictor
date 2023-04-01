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

    pub fn find_player(&mut self, name: String) -> Option<&mut Player> {
        self.players
            .iter_mut()
            .find(|p| p.name == name)
    }
}