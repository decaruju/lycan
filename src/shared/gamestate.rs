use std::fmt::Debug;

#[derive(Debug)]
pub struct Gamestate {
    pub players: Vec<Player>,
    pub test: String,
}

#[derive(Debug)]
pub struct Player {
    pub name: String,
}

impl Gamestate {
    pub fn add_player(&mut self, player_name: String) {
        self.players.push(Player{name: player_name});
    }
}
