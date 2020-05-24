use std::{collections::HashMap, fmt::Debug};

use uuid::Uuid;
use rand;

use lycan::shared::gamestate::{Gamestate, Player, Message, Map};
use lycan::shared::room::{Item};

#[derive(Debug)]
pub struct ServerGamestate {
    pub gamestate: Gamestate,
    pub curses: HashMap<String, bool>,
}

impl ServerGamestate {
    pub fn new() -> ServerGamestate {
        ServerGamestate {
            gamestate: Gamestate::default(),
            curses: HashMap::new(),
        }
    }

    pub fn next_round(&mut self) {
        self.gamestate.map = Map::default();
        self.gamestate.keys = 0;
        self.gamestate.round += 1;
        self.gamestate.messages = vec![];
        self.curses = HashMap::new();
        for (_id, player) in self.gamestate.players.iter_mut() {
            player.position = (
                ((rand::random::<i32>() % 16 - 8) * 16 * 16 + 16*8) as f32,
                ((rand::random::<i32>() % 16 - 8) * 16 * 16 + 16*8) as f32,
            );
        }
    }

    pub fn curse(&mut self, curse: Item) -> Item {
        match self.curses.get(&curse.to_string()) {
            Some(true) => Item::Key,
            _ => {
                self.curses.insert(curse.to_string(), true);
                curse
            }
        }
    }


    pub fn add_player(&mut self, uuid: String, player_name: String) {
        self.gamestate.players.insert(
            uuid,
            Player {
                ready: false,
                name: player_name,
                position: (
                    ((rand::random::<i32>() % 16 - 8) * 16 * 16 + 16*8) as f32,
                    ((rand::random::<i32>() % 16 - 8) * 16 * 16 + 16*8) as f32,
                ),
            },
        );
    }

    pub fn update_player(&mut self, player_id: &String, position: (f32, f32), ready: bool) -> Option<()> {
        let player = self.gamestate.players.get_mut(player_id)?;
        player.position = position;
        player.ready = ready;
        Some(())
    }

    pub fn all_players_ready(&self) -> bool {
        for (id, player) in self.gamestate.players.iter() {
            if !player.ready {
                return false
            }
        }
        true
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
        let uuid = "yes".to_string();//Uuid::new_v4().to_string();
        self.games.entry(uuid.clone()).or_insert(ServerGamestate::new());
        uuid
    }

    pub fn join_game(&mut self, game_id: String, player_name: String) -> Option<(String, (f32, f32))> {
        let game = self.games.get_mut(&game_id)?;
        let uuid = Uuid::new_v4().to_string();
        game.add_player(uuid.clone(), player_name);
        Some((uuid.clone(), game.gamestate.players.get(&uuid).unwrap().position))
    }

    pub fn update(
        &mut self,
        game_id: String,
        player_id: String,
        position: (f32, f32),
        new_rooms: Vec<(i32, i32)>,
        cleared_rooms: Vec<(i32, i32)>,
        ready: bool,
        end: bool,
    ) -> Option<&Gamestate> {
        let game = self.games.get_mut(&game_id)?;
        if end {
            game.next_round();
            game.gamestate.messages.push(Message::new(format!("{} has found the exit, a new round is starting!", game.gamestate.players.get(&player_id).unwrap().name)));
            return Some(&game.gamestate);
        }
        game.update_player(&player_id, position, ready)?;
        if game.all_players_ready() {
           game.gamestate.started = true;
        }
        for room_pos in new_rooms {
            let keys = game.gamestate.keys;
            let item = match rand::random::<u32>() % 4 {
                0 => game.curse(Item::Clear),
                1 => game.curse(Item::Spin),
                2 => game.curse(Item::Bad),
                _ => game.curse(Item::Key),
            };
            if let Some(room) = game.gamestate.add_room(room_pos) {
                if keys < 8 && rand::random::<u32>() % 2 == 0 {
                    let x = rand::random::<u32>() % 8 + 4;
                    let y = rand::random::<u32>() % 8 + 4;
                    room.item = Some((item, (x, y)));
                    println!("item at {}, {}", x, y);
                }
            }
        }
        for room_pos in cleared_rooms {
            match game.gamestate.map.room(room_pos.0, room_pos.1)?.item {
                Some((Item::Key, _)) => {
                    game.gamestate.keys += 1;
                    game.gamestate.messages.push(Message::new(format!("{} has picked up a key!", game.gamestate.players.get(&player_id).unwrap().name)))
                },
                Some(_) => {
                    game.gamestate.messages.push(Message::new(format!("{} has been cursed!", game.gamestate.players.get(&player_id).unwrap().name)))
                },
                None => {}
            }
            game.gamestate.map.mut_room(room_pos.0, room_pos.1)?.item = None;
        }
        Some(&game.gamestate)
    }
}
