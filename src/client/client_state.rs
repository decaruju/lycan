use lycan::shared::gamestate::{Gamestate, Player};
use lycan::shared::http::UpdateResponse;
use lycan::shared::room::Room;
use lycan::shared::utils::Direction;
use std::collections::HashMap;

pub struct ClientGamestate {
    pub gamestate: Gamestate,
    pub player_id: Option<String>,
    pub game_id: Option<String>,
    pub new_rooms: Vec<(i32, i32)>,
}

impl ClientGamestate {
    pub fn load(_string: String) -> ClientGamestate {
        ClientGamestate {
            gamestate: Gamestate::default(),
            new_rooms: Vec::new(),
            player_id: None,
            game_id: None,
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

    pub fn set_player(&mut self, player_id: String) {
        self.gamestate.players.insert(
            player_id.clone(),
            Player {
                name: String::from("foo"),
                position: (100.0, 100.0),
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

    pub fn add_room(&mut self, position: (i32, i32)) {
        if !self.gamestate.map.room(position.0, position.1).is_none() {
            return;
        }
        self.new_rooms.push(position);
        self.gamestate.add_room(position);
        self.gamestate.remove_doors(position);
    }

    pub fn get_new_rooms(&mut self) -> Vec<(i32, i32)> {
        let new_rooms = self.new_rooms.to_vec();
        self.new_rooms = Vec::new();
        new_rooms
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
        for (player_id, player_state) in data.players {
            if player_id == self.player_id.as_ref().unwrap().clone() {
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
                        },
                    );
                }
            }
        }
    }
}
