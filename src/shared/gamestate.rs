use std::collections::HashMap;
use std::fmt::Debug;

use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Gamestate {
    pub players: HashMap<String, Player>,
    pub map: Map,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Map {
    pub rooms: HashMap<(i32, i32), Room>,
}

impl Default for Map {
    fn default() -> Self {
        let mut rooms = HashMap::new();
        rooms.insert((0, 0), Room::default());
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

#[derive(Debug, Serialize, Deserialize)]
pub struct Room {
    pub doors: HashMap<Direction, bool>,
    pub position: (i32, i32),
}

impl Room {
    pub fn new(position: (i32, i32)) -> Self {
        let mut doors = HashMap::new();
        doors.insert(Direction::Right, true);
        doors.insert(Direction::Up, true);
        doors.insert(Direction::Down, true);
        doors.insert(Direction::Left, true);
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
        if tile.0 == 0 && self.doors[&Direction::Left] && tile.1 > 6 && tile.1 < 10 {
            return true;
        }
        if tile.0 == 15 && self.doors[&Direction::Right] && tile.1 > 6 && tile.1 < 10 {
            return true;
        }
        if tile.1 == 0 && self.doors[&Direction::Down] && tile.0 > 6 && tile.0 < 10 {
            return true;
        }
        if tile.1 == 15 && self.doors[&Direction::Up] && tile.0 > 6 && tile.0 < 10 {
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
