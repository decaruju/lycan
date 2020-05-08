use std::collections::HashMap;
use std::fmt::Debug;

use lycan::shared::gamestate::Gamestate;

use uuid::Uuid;

#[derive(Debug)]
pub struct ServerGamestate {
    pub gamestate: Gamestate,
}

impl ServerGamestate {
    pub fn new() -> ServerGamestate {
        ServerGamestate{ gamestate: Gamestate::default() }
    }

    pub fn add_player(&mut self, player_name: String) {
        self.gamestate.add_player(player_name)
    }

    pub fn dump_players(&self) -> String {
        format!("{:?}", self.gamestate.players.iter().map(|player| player.name.clone()))
    }
}

pub struct ServerState {
    games: HashMap<String, ServerGamestate>
}

impl ServerState {
    pub fn new() -> ServerState {
        ServerState{
            games: HashMap::new(),
        }
    }

    pub fn new_game(&mut self) -> String {
        let uuid = Uuid::new_v4().to_string();
        self.games.insert(uuid.clone(), ServerGamestate::new());
        uuid
    }

    pub fn join_game(&mut self, game_id: String, player_name: String) -> Option<String> {
        let game = self.games.get_mut(&game_id)?;
        game.add_player(player_name);
        Some(game.dump_players())
    }
}
