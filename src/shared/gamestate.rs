use crate::shared::room::{Room};
use crate::shared::utils::{Direction};

use std::collections::HashMap;
use std::fmt::{ Debug };

use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Gamestate {
    pub players: HashMap<String, Player>,
    pub map: Map,
}

impl Gamestate {
    pub fn add_room(&mut self, position: (i32, i32)) {
        if !self.map.room(position.0, position.1).is_none() {
            return
        }
        self.map.add_room(position, Room::new(position));
        self.remove_doors(position);
    }

    pub fn remove_doors(&mut self, position: (i32, i32)) {
        for room_pos in [
            (position.0, position.1 + 1),
            (position.0, position.1 - 1),
            (position.0 - 1, position.1),
            (position.0 + 1, position.1),
        ].iter() {
            if self.map.room(room_pos.0, room_pos.1).is_none() && self.map.room_degree(*room_pos) > 1 {
                for (room_pos, direction) in [
                    ((room_pos.0, room_pos.1 + 1), Direction::Down),
                    ((room_pos.0, room_pos.1 - 1), Direction::Up),
                    ((room_pos.0 - 1, room_pos.1), Direction::Right),
                    ((room_pos.0 + 1, room_pos.1), Direction::Left),
                ].iter() {
                    match self.map.room_mut(room_pos.0, room_pos.1) {
                        Some(room) => {
                            *room.doors.get_mut(&direction.to_string()).unwrap() = false;
                        }
                        _ => (),
                    }
                }
            }
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Map {
    pub rooms: HashMap<i32, HashMap<i32, Room>>,
}

impl Map {
    pub fn room(&self, x: i32, y: i32) -> Option<&Room> {
        self.rooms.get(&x)?.get(&y)
    }

    pub fn room_mut(&mut self, x: i32, y: i32) -> Option<&mut Room> {
        self.rooms.get_mut(&x)?.get_mut(&y)
    }

    pub fn add_room(&mut self, position: (i32, i32), room: Room) {
        match self.rooms.get_mut(&position.0) {
            Some(row) => {
                row.insert(position.1, room);
            },
            None => {
                let mut row = HashMap::new();
                row.insert(position.1, room);
                self.rooms.insert(position.0, row);
            }
        }
    }

    pub fn room_degree(&self, position: (i32, i32)) -> i32 {
        let mut degree = 0;
        for room_pos in [
            (position.0, position.1 + 1),
            (position.0, position.1 - 1),
            (position.0 - 1, position.1),
            (position.0 + 1, position.1),
        ].iter() {
            if !self.room(room_pos.0, room_pos.1).is_none() {
                degree += 1;
            }
        }
        degree
    }

}

impl Default for Map {
    fn default() -> Self {
        let mut rooms = HashMap::new();
        let mut row = HashMap::new();
        row.insert(0, Room::default());
        rooms.insert(0, row);
        Map{rooms}
    }
}


#[derive(Debug, Serialize, Deserialize)]
pub struct Player {
    pub name: String,
    pub position: (f32, f32),
}

impl Player {
    pub fn move_player(&mut self, direction: (f32, f32)) {
        self.position.0 += direction.0;
        self.position.1 += direction.1;
    }
}

#[derive(Debug)]
pub struct Position {
    x: u32,
    y: u32,
}

impl Default for Gamestate {
    fn default() -> Self {
        Gamestate{players: HashMap::new(), map: Map::default()}
    }
}

struct NewResponse {
    pub game_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SharedPlayer {
    pub name: String,
    pub position: (f32, f32),
}
