use lycan::shared::gamestate::Gamestate;

struct ServerGamestate{
    pub gamestate: Gamestate,
}

impl ServerGamestate {
    fn dump(&self) -> String {
        self.gamestate.test.clone()
    }
}

fn main() {
    let gamestate = ServerGamestate{gamestate: Gamestate{test:String::from("test")}};
    println!("{}", gamestate.dump())
}