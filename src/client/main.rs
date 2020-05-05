use lycan::shared::gamestate::Gamestate;

struct ClientGamestate {
    pub gamestate: Gamestate,
}

impl ClientGamestate { 
    fn load(string: String) -> ClientGamestate {
        ClientGamestate{gamestate: Gamestate{test: string}}
    }
}

fn main() {
    let gamestate = ClientGamestate::load(String::from("test"));
    println!("{}", gamestate.gamestate.test)
}