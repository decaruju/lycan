use std::collections::HashMap;
use std::fmt::Debug;

use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Gamestate {
    pub players: HashMap<String, Player>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Player {
    pub name: String,
    pub position: (f32, f32),
}

impl Player {
    pub fn move_player(&mut self, direction: (f32, f32)) {
        self.position.0 += direction.0;
        self.position.1 += direction.1;
    }
}

#[derive(Debug)]
pub struct Position {
    x: u32,
    y: u32,
}

impl Default for Gamestate {
    fn default() -> Self {
        Gamestate{players: HashMap::new()}
    }
}

struct NewResponse {
    pub game_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateResponse {
    pub players: HashMap<String, SharedPlayer>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SharedPlayer {
    pub name: String,
    pub position: (f32, f32),
}
