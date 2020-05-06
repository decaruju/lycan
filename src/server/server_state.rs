use std::collections::HashMap;

use lycan::shared::gamestate::Gamestate;

use uuid::Uuid;

pub struct ServerGamestate {
    pub gamestate: Gamestate,
}

impl ServerGamestate {
    pub fn dump(&self) -> String {
        self.gamestate.test.clone()
    }

    pub fn new() -> ServerGamestate {
        ServerGamestate{ gamestate: Gamestate{ test:String::from("test") } }
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

    pub fn new_game(&mut self) {
        println!("{}", Uuid::new_v4())
    }
}
