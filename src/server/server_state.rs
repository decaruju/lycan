use std::collections::HashMap;
use std::fmt::Debug;

use lycan::shared::gamestate::Gamestate;

use uuid::Uuid;

#[derive(Debug)]
pub struct ServerGamestate {
    pub gamestate: Gamestate,
}

impl ServerGamestate {
    pub fn dump(&self) -> String {
        self.gamestate.test.clone()
    }

    pub fn new() -> ServerGamestate {
        ServerGamestate{ gamestate: Gamestate{ players: vec![], test: String::from("test") } }
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
        println!("{:?}", self.games);
        uuid
    }

    pub fn join_game(&mut self, game_id: String, player_name: String) -> Option<String> {
        let game = self.games.get_mut(&game_id)?;
        println!("im in");
        game.add_player(player_name);
        Some(game.dump_players())
    }
}
