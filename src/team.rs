use crate::player::Player;

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
}