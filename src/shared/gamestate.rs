use std::collections::HashMap;
use std::fmt::Debug;

use serde::{Serialize, Deserialize};

#[derive(Debug)]
pub struct Gamestate {
    pub players: HashMap<String, Player>,
}

#[derive(Debug)]
pub struct Player {
    pub name: String,
    pub position: (u64, u64),
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
