use lycan::shared::gamestate::{Gamestate, Player};
use lycan::shared::http::UpdateResponse;
use lycan::shared::room::{Room, Item};
use lycan::shared::utils::Direction;
use std::collections::HashMap;

pub struct ClientGamestate {
    pub gamestate: Gamestate,
    pub player_id: Option<String>,
    pub game_id: Option<String>,
    pub new_rooms: Vec<(i32, i32)>,
    pub cleared_rooms: Vec<(i32, i32)>,
    pub explored_rooms: HashMap<(i32, i32), bool>,
}

impl ClientGamestate {
    pub fn load(_string: String) -> ClientGamestate {
        ClientGamestate {
            gamestate: Gamestate::default(),
            new_rooms: Vec::new(),
            player_id: None,
            game_id: None,
            explored_rooms: HashMap::new(),
            cleared_rooms: Vec::new(),
        }
    }

    pub fn is_started(&self) -> bool {
        self.gamestate.started
    }

    pub fn explored(&self, position: (i32, i32)) -> bool {
        match self.explored_rooms.get(&position) {
            Some(explored) => *explored,
            None => false,
        }
    }

    pub fn get_player(&self) -> Option<&Player> {
        match &self.player_id {
            Some(player_id) => Some(self.gamestate.players.get(player_id)?),
            None => None,
        }
    }

    pub fn get_mut_player(&mut self) -> Option<&mut Player> {
        match &self.player_id {
            Some(player_id) => Some(self.gamestate.players.get_mut(player_id)?),
            None => None,
        }
    }

    pub fn set_player(&mut self, player_id: String, position: (f32, f32)) {
        self.gamestate.players.insert(
            player_id.clone(),
            Player {
                name: String::from("foo"),
                position,
                ready: false,
            },
        );
        self.player_id = Some(player_id);
    }

    pub fn get_players(&self) -> &HashMap<String, Player> {
        &self.gamestate.players
    }

    pub fn set_game(&mut self, game_id: String) {
        self.game_id = Some(game_id);
    }

    pub fn player_room_coord(&self) -> (i32, i32) {
        let position = self.player_position();
        let room_x = position.0 / 16. / 16.;
        let room_y = position.1 / 16. / 16.;
        (room_x.floor() as i32, room_y.floor() as i32)
    }

    pub fn player_position(&self) -> (f32, f32) {
        self.get_player().unwrap().position
    }

    pub fn player_tile(&self) -> (i32, i32) {
        let position = self.get_player().unwrap().position;
        (
            (((position.0 as i32).rem_euclid(16 * 16)) as f32 / 16.0).floor() as i32,
            (((position.1 as i32).rem_euclid(16 * 16)) as f32 / 16.0).floor() as i32,
        )
    }

    pub fn player_room(&self) -> &Room {
        let player_room_coord = self.player_room_coord();
        self.gamestate
            .map
            .room(player_room_coord.0, player_room_coord.1)
            .unwrap()
    }

    pub fn mut_player_room(&mut self) -> &mut Room {
        let player_room_coord = self.player_room_coord();
        self.gamestate
            .map
            .mut_room(player_room_coord.0, player_room_coord.1)
            .unwrap()
    }

    pub fn player_in_wall(&self) -> bool {
        let tile = self.player_tile();
        let room = self.player_room();
        room.is_wall(tile)
    }

    pub fn player_in_door(&self) -> bool {
        let room = self.player_room();
        let tile = self.player_tile();
        room.is_door(tile)
    }

    pub fn player_in_exit(&self) -> bool {
        let room = self.player_room();
        let tile = self.player_tile();
        room.is_exit(tile)
    }

    pub fn player_on_item(&self) -> bool {
        let room = self.player_room();
        let tile = self.player_tile();
        if let Some(item) = &room.item {
            let pos = item.1;
            pos.0 as i32 == tile.0 && pos.1 as i32 == tile.1
        } else {
            false
        }
    }

    pub fn remove_item(&mut self) {
        let mut room = self.mut_player_room();
        match room.item {
            Some((Item::Key, _)) => {
            },
            Some((Item::Spin, _)) => {
            },
            None => {}
        }
        room.item = None;
        self.cleared_rooms.push(self.player_room_coord());
    }

    pub fn add_room(&mut self, position: (i32, i32)) {
        *self.explored_rooms.entry(position.clone()).or_insert(true) = true;
        if !self.gamestate.map.room(position.0, position.1).is_none() {
            return;
        }
        self.new_rooms.push(position);
        self.gamestate.add_room(position);
        self.gamestate.remove_doors(position);
    }

    pub fn add_player_room(&mut self) {
        println!("{:?}", self.player_room_coord());
        self.add_room(self.player_room_coord());
    }

    pub fn get_new_rooms(&mut self) -> Vec<(i32, i32)> {
        let new_rooms = self.new_rooms.to_vec();
        self.new_rooms = Vec::new();
        new_rooms
    }

    pub fn get_cleared_rooms(&mut self) -> Vec<(i32, i32)> {
        let cleared_rooms = self.cleared_rooms.to_vec();
        self.cleared_rooms = Vec::new();
        cleared_rooms
    }

    pub fn get_game_id(&self) -> String {
        match &self.game_id {
            Some(game_id) => game_id.clone(),
            None => String::from(""),
        }
    }

    pub fn get_rooms(&self) -> Vec<&Room> {
        let mut rtn = Vec::new();
        for (x, row) in &self.gamestate.map.rooms {
            for (y, room) in row {
                rtn.push(room);
            }
        }
        rtn
    }

    pub fn get_player_id(&self) -> String {
        match &self.player_id {
            Some(player_id) => player_id.clone(),
            None => String::from(""),
        }
    }

    pub fn update(&mut self, data: UpdateResponse) {
        self.gamestate.map = data.map;
        self.gamestate.started = data.started;
        self.gamestate.keys = data.keys;
        for (player_id, player_state) in data.players {
            if self.gamestate.started && player_id == self.player_id.as_ref().unwrap().clone() {
                continue;
            }
            match self.gamestate.players.get_mut(&player_id) {
                Some(player) => {
                    player.position = player_state.position;
                }
                None => {
                    self.gamestate.players.insert(
                        player_id,
                        Player {
                            position: player_state.position,
                            name: player_state.name,
                            ready: false,
                        },
                    );
                }
            }
        }
    }
}
