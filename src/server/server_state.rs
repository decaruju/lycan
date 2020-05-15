use std::{collections::HashMap, fmt::Debug};

use uuid::Uuid;

use lycan::shared::gamestate::{Gamestate, Player};

#[derive(Debug)]
pub struct ServerGamestate {
    pub gamestate: Gamestate,
}

impl ServerGamestate {
    pub fn new() -> ServerGamestate {
        ServerGamestate {
            gamestate: Gamestate::default(),
        }
    }

    pub fn add_player(&mut self, uuid: String, player_name: String) {
        self.gamestate.players.insert(
            uuid,
            Player {
                name: player_name,
                position: (0.0, 0.0),
            },
        );
    }

    pub fn update_player(&mut self, player_id: String, position: (f32, f32)) -> Option<()> {
        let player = self.gamestate.players.get_mut(&player_id)?;
        player.position = position;
        Some(())
    }
}

pub struct ServerState {
    games: HashMap<String, ServerGamestate>,
}

impl ServerState {
    pub fn new() -> ServerState {
        ServerState {
            games: HashMap::new(),
        }
    }

    pub fn new_game(&mut self, _public: bool) -> String {
        let uuid = Uuid::new_v4().to_string();
        self.games.insert(uuid.clone(), ServerGamestate::new());
        uuid
    }

    pub fn join_game(&mut self, game_id: String, player_name: String) -> Option<String> {
        let game = self.games.get_mut(&game_id)?;
        let uuid = Uuid::new_v4().to_string();
        game.add_player(uuid.clone(), player_name);
        Some(uuid)
    }

    pub fn update(
        &mut self,
        game_id: String,
        player_id: String,
        position: (f32, f32),
        new_rooms: Vec<(i32, i32)>,
    ) -> Option<&Gamestate> {
        let game = self.games.get_mut(&game_id)?;
        game.update_player(player_id, position)?;
        for room_pos in new_rooms {
            game.gamestate.add_room(room_pos);
        }
        Some(&game.gamestate)
    }
}
