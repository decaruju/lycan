use std::{collections::HashMap, fmt::Debug};

use serde::{Deserialize, Serialize};

use crate::shared::gamestate::{Gamestate, Map, Player, Message};

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
    pub position: (f32, f32),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateRequest {
    pub game_id: String,
    pub player_id: String,
    pub position: (f32, f32),
    pub new_rooms: Vec<(i32, i32)>,
    pub cleared_rooms: Vec<(i32, i32)>,
    pub ready: bool,
    pub end: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateResponse {
    pub players: HashMap<String, Player>,
    pub map: Map,
    pub started: bool,
    pub keys: u32,
    pub messages: Vec<Message>,
    pub round: u32,
}

impl UpdateResponse {
    pub fn new(gamestate: &Gamestate) -> UpdateResponse {
        UpdateResponse {
            started: gamestate.started,
            players: gamestate.players.clone(),
            map: gamestate.map.clone(),
            keys: gamestate.keys,
            messages: gamestate.messages.clone(),
            round: gamestate.round,
        }
    }
}
