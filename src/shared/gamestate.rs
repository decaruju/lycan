use std::collections::HashMap;
use std::fmt::{ Debug, Display };

use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Gamestate {
    pub players: HashMap<String, Player>,
    pub map: Map,
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

#[derive(std::hash::Hash, std::cmp::PartialEq, std::cmp::Eq, Debug, Serialize, Deserialize)]
pub enum Direction {
    Down,
    Up,
    Left,
    Right,
}

impl Direction {
    pub fn to_string(&self) -> String {
        match self {
            Direction::Down => String::from("Down"),
            Direction::Up => String::from("Up"),
            Direction::Left => String::from("Left"),
            Direction::Right => String::from("Right"),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Room {
    pub doors: HashMap<String, bool>,
    pub position: (i32, i32),
}

impl Room {
    pub fn new(position: (i32, i32)) -> Self {
        let mut doors = HashMap::new();
        doors.insert(Direction::Right.to_string(), true);
        doors.insert(Direction::Up.to_string(), true);
        doors.insert(Direction::Down.to_string(), true);
        doors.insert(Direction::Left.to_string(), true);
        Room{doors, position}
    }
    pub fn is_wall(&self, tile: (i32, i32)) -> bool {
        if tile.0 == 0 || tile.1 == 0 {
            return true;
        }
        if tile.0 == 15 || tile.1 == 15 {
            return true;
        }
        return false;
    }

    pub fn is_door(&self, tile: (i32, i32)) -> bool {
        if tile.0 == 0 && self.doors[&Direction::Left.to_string()] && tile.1 > 6 && tile.1 < 10 {
            return true;
        }
        if tile.0 == 15 && self.doors[&Direction::Right.to_string()] && tile.1 > 6 && tile.1 < 10 {
            return true;
        }
        if tile.1 == 0 && self.doors[&Direction::Down.to_string()] && tile.0 > 6 && tile.0 < 10 {
            return true;
        }
        if tile.1 == 15 && self.doors[&Direction::Up.to_string()] && tile.0 > 6 && tile.0 < 10 {
            return true;
        }
        return false;
    }
}

impl Default for Room {
    fn default() -> Self {
        Room::new((0, 0))
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
pub struct UpdateResponse {
    pub players: HashMap<String, SharedPlayer>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SharedPlayer {
    pub name: String,
    pub position: (f32, f32),
}
