use crate::shared::room::Room;
use crate::shared::utils::Direction;

use std::collections::HashMap;
use std::fmt::Debug;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Gamestate {
    pub players: HashMap<String, Player>,
    pub map: Map,
    pub started: bool,
    pub keys: u32,
    pub messages: Vec<Message>,
    pub round: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Message {
    pub text: String,
}

impl Message {
    pub fn new(text: String) -> Message {
        Message{text}
    }
}

impl Gamestate {
    pub fn next_round(&mut self) {
        self.map = Map::default();
        self.keys = 0;
        self.round += 1;
        self.messages = vec![];
    }

    pub fn add_room(&mut self, position: (i32, i32)) -> bool {
        if !self.map.room(position.0, position.1).is_none() {
            return false;
        }
        self.map.add_room(position, Room::basic(position));
        self.remove_doors(position);
        true
    }

    pub fn remove_doors(&mut self, position: (i32, i32)) {
        for room_pos in [
            (position.0, position.1 + 1),
            (position.0, position.1 - 1),
            (position.0 - 1, position.1),
            (position.0 + 1, position.1),
        ]
        .iter()
        {
            if self.map.room(room_pos.0, room_pos.1).is_none()
                && self.map.room_degree(*room_pos) > 1
            {
                for (room_pos, direction) in [
                    ((room_pos.0, room_pos.1 + 1), Direction::Down),
                    ((room_pos.0, room_pos.1 - 1), Direction::Up),
                    ((room_pos.0 - 1, room_pos.1), Direction::Right),
                    ((room_pos.0 + 1, room_pos.1), Direction::Left),
                ]
                .iter()
                {
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

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Map {
    pub rooms: HashMap<i32, HashMap<i32, Room>>,
}

impl Map {
    pub fn room(&self, x: i32, y: i32) -> Option<&Room> {
        self.rooms.get(&x)?.get(&y)
    }

    pub fn mut_room(&mut self, x: i32, y: i32) -> Option<&mut Room> {
        self.rooms.get_mut(&x)?.get_mut(&y)
    }

    pub fn room_mut(&mut self, x: i32, y: i32) -> Option<&mut Room> {
        self.rooms.get_mut(&x)?.get_mut(&y)
    }

    pub fn add_room(&mut self, position: (i32, i32), room: Room) {
        match self.rooms.get_mut(&position.0) {
            Some(row) => {
                row.insert(position.1, room);
            }
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
        ]
        .iter()
        {
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
        row.insert(0, Room::exit((0, 0)));
        rooms.insert(0, row);
        Map { rooms }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Player {
    pub name: String,
    pub position: (f32, f32),
    pub ready: bool,
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
        Gamestate {
            players: HashMap::new(),
            map: Map::default(),
            started: false,
            keys: 0,
            messages: vec![],
            round: 1,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SharedPlayer {
    pub name: String,
    pub position: (f32, f32),
}
