use std::{collections::HashMap, fmt::Debug};

use serde::{Deserialize, Serialize};

use crate::shared::gamestate::{Gamestate, Player};

#[derive(Debug, Serialize, Deserialize)]
pub struct NewGameRequest {
    pub public: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NewGameResponse {
    pub game_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JoinGameRequest {
    pub game_id: String,
    pub player_name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JoinGameResponse {
    pub player_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateRequest {
    pub game_id: String,
    pub player_id: String,
    pub position: (f32, f32),
    pub new_rooms: Vec<(i32, i32)>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateResponse {
    players: HashMap<String, Player>,
}

impl UpdateResponse {
    pub fn new(gamestate: &Gamestate) -> UpdateResponse {
        UpdateResponse {
            players: gamestate.players.clone(),
        }
    }
}
