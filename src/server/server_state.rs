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
        let mut default_map: Vec<(i32, i32)> = vec![(-15, 0), (-14, -1), (-14, 0), (-14, 1), (-13, -2), (-13, 0), (-13, 2), (-12, -3), (-12, -2), (-12, -1), (-12, 0), (-12, 1), (-12, 2), (-12, 3), (-11, -4), (-11, -2), (-11, 0), (-11, 2), (-11, 4), (-10, -5), (-10, -4), (-10, -3), (-10, -1), (-10, 0), (-10, 1), (-10, 3), (-10, 4), (-10, 5), (-9, -6), (-9, -4), (-9, -2), (-9, 0), (-9, 2), (-9, 4), (-9, 6), (-8, -7), (-8, -6), (-8, -5), (-8, -4), (-8, -3), (-8, -2), (-8, -1), (-8, 0), (-8, 1), (-8, 2), (-8, 3), (-8, 4), (-8, 5), (-8, 6), (-8, 7), (-7, -8), (-7, -6), (-7, -4), (-7, -2), (-7, 0), (-7, 2), (-7, 4), (-7, 6), (-7, 8), (-6, -9), (-6, -8), (-6, -7), (-6, -5), (-6, -4), (-6, -3), (-6, -1), (-6, 0), (-6, 1), (-6, 3), (-6, 4), (-6, 5), (-6, 7), (-6, 8), (-6, 9), (-5, -10), (-5, -8), (-5, -6), (-5, -4), (-5, -2), (-5, 0), (-5, 2), (-5, 4), (-5, 6), (-5, 8), (-5, 10), (-4, -11), (-4, -10), (-4, -9), (-4, -8), (-4, -7), (-4, -6), (-4, -5), (-4, -3), (-4, -2), (-4, -1), (-4, 0), (-4, 1), (-4, 2), (-4, 3), (-4, 5), (-4, 6), (-4, 7), (-4, 8), (-4, 9), (-4, 10), (-4, 11), (-3, -12), (-3, -10), (-3, -8), (-3, -6), (-3, -4), (-3, -2), (-3, 0), (-3, 2), (-3, 4), (-3, 6), (-3, 8), (-3, 10), (-3, 12), (-2, -13), (-2, -12), (-2, -11), (-2, -9), (-2, -8), (-2, -7), (-2, -5), (-2, -4), (-2, -3), (-2, -1), (-2, 0), (-2, 1), (-2, 3), (-2, 4), (-2, 5), (-2, 7), (-2, 8), (-2, 9), (-2, 11), (-2, 12), (-2, 13), (-1, -14), (-1, -12), (-1, -10), (-1, -8), (-1, -6), (-1, -4), (-1, -2), (-1, 0), (-1, 2), (-1, 4), (-1, 6), (-1, 8), (-1, 10), (-1, 12), (-1, 14), (0, -15), (0, -14), (0, -13), (0, -12), (0, -11), (0, -10), (0, -9), (0, -8), (0, -7), (0, -6), (0, -5), (0, -4), (0, -3), (0, -2), (0, -1), (0, 1), (0, 2), (0, 3), (0, 4), (0, 5), (0, 6), (0, 7), (0, 8), (0, 9), (0, 10), (0, 11), (0, 12), (0, 13), (0, 14), (0, 15), (1, -14), (1, -12), (1, -10), (1, -8), (1, -6), (1, -4), (1, -2), (1, 0), (1, 2), (1, 4), (1, 6), (1, 8), (1, 10), (1, 12), (1, 14), (2, -13), (2, -12), (2, -11), (2, -9), (2, -8), (2, -7), (2, -5), (2, -4), (2, -3), (2, -1), (2, 0), (2, 1), (2, 3), (2, 4), (2, 5), (2, 7), (2, 8), (2, 9), (2, 11), (2, 12), (2, 13), (3, -12), (3, -10), (3, -8), (3, -6), (3, -4), (3, -2), (3, 0), (3, 2), (3, 4), (3, 6), (3, 8), (3, 10), (3, 12), (4, -11), (4, -10), (4, -9), (4, -8), (4, -7), (4, -6), (4, -5), (4, -3), (4, -2), (4, -1), (4, 0), (4, 1), (4, 2), (4, 3), (4, 5), (4, 6), (4, 7), (4, 8), (4, 9), (4, 10), (4, 11), (5, -10), (5, -8), (5, -6), (5, -4), (5, -2), (5, 0), (5, 2), (5, 4), (5, 6), (5, 8), (5, 10), (6, -9), (6, -8), (6, -7), (6, -5), (6, -4), (6, -3), (6, -1), (6, 0), (6, 1), (6, 3), (6, 4), (6, 5), (6, 7), (6, 8), (6, 9), (7, -8), (7, -6), (7, -4), (7, -2), (7, 0), (7, 2), (7, 4), (7, 6), (7, 8), (8, -7), (8, -6), (8, -5), (8, -4), (8, -3), (8, -2), (8, -1), (8, 0), (8, 1), (8, 2), (8, 3), (8, 4), (8, 5), (8, 6), (8, 7), (9, -6), (9, -4), (9, -2), (9, 0), (9, 2), (9, 4), (9, 6), (10, -5), (10, -4), (10, -3), (10, -1), (10, 0), (10, 1), (10, 3), (10, 4), (10, 5), (11, -4), (11, -2), (11, 0), (11, 2), (11, 4), (12, -3), (12, -2), (12, -1), (12, 0), (12, 1), (12, 2), (12, 3), (13, -2), (13, 0), (13, 2), (14, -1), (14, 0), (14, 1), (15, 0), (0, 0), (-17, 0), (17, 0), (0, 17), (0, -17)];

        let mut gamestate = Gamestate::default();
        let mut server_gamestate = ServerGamestate {
            gamestate,
            curses: HashMap::new(),
        };


        let mut index = 0;
        while let Some(room) = default_map.get(index) {
            if server_gamestate.gamestate.map.room_degree(*room) == 1 {
                server_gamestate.add_room(*room);
                default_map.remove(index);
                index = 0
            } else {
                index += 1;
            }
        }

        for item in &[
            Item::Clear,
            Item::Spin,
            Item::Bad,
            Item::Key,
            Item::Key,
            Item::Key,
            Item::Key,
            Item::Key,
            Item::Key,
            Item::Key,
            Item::Key,
        ] {
            let mut room_x = rand::random::<i32>() % 31 + 15;
            let mut room_y = rand::random::<i32>() % 31 + 15;
            while server_gamestate.gamestate.map.room(room_x, room_y).is_none() || room_x == 0 && room_y == 0 {
                room_x = rand::random::<i32>() % 31 + 15;
                room_y = rand::random::<i32>() % 31 + 15;
            }
            let x = rand::random::<u32>() % 8 + 4;
            let y = rand::random::<u32>() % 8 + 4;
            server_gamestate.gamestate.map.mut_room(room_x, room_y).unwrap().item = Some((item.clone(), (x, y)));
        }

        server_gamestate
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

    pub fn add_room(&mut self, position: (i32, i32)) -> bool {
        self.gamestate.add_room(position)
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
        let mut coords = (1, 1);
        while self.gamestate.map.room(coords.0, coords.1).is_none() || coords.0 == 0 && coords.1 == 0 {
            coords = (
                ((rand::random::<i32>() % 16 - 8)),
                ((rand::random::<i32>() % 16 - 8)),
            );
        }
        self.gamestate.players.insert(
            uuid,
            Player {
                ready: false,
                name: player_name,
                position: (
                    coords.0 as f32 * 256. + 128.,
                    coords.1 as f32 * 256. + 128.,
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
        // let uuid = "yes".to_string();//Uuid::new_v4().to_string();
        let uuid = Uuid::new_v4().to_string()[..6].to_string();
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
