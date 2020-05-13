use crate::shared::utils::{Direction, ROOM_SIZE};

use serde::{Serialize, Deserialize};
use std::fmt::{ Debug };
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub enum TileType {
    None,
    Floor,
    Wall(WallType),
    Door(Direction),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Room {
    pub doors: HashMap<String, bool>,
    pub position: (i32, i32),
    pub room_type: RoomType,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum RoomType {
    Basic,
}


impl RoomType {
    pub fn tile(&self, position: (i32, i32)) -> Tile {
        match self {
            RoomType::Basic => Tile{x: position.0, y: position.1, tile_type: BASIC_ROOM.get(position.1 as usize).unwrap().get(position.0 as usize).unwrap().clone()}
        }
    }
}

#[derive(Clone, Debug)]
pub enum WallType {
    North,
    South,
    East,
    West,
    InnerNorthEast,
    InnerNorthWest,
    InnerSouthEast,
    InnerSouthWest,
    OuterNorthEast,
    OuterNorthWest,
    OuterSouthEast,
    OuterSouthWest,
}

pub struct Tile {
    pub x: i32,
    pub y: i32,
    pub tile_type: TileType,
}

impl Room {
    pub fn new(position: (i32, i32)) -> Self {
        let mut doors = HashMap::new();
        doors.insert(Direction::Right.to_string(), true);
        doors.insert(Direction::Up.to_string(), true);
        doors.insert(Direction::Down.to_string(), true);
        doors.insert(Direction::Left.to_string(), true);
        Room{doors, position, room_type: RoomType::Basic}
    }

    pub fn tile(&self, tile: (i32, i32)) -> Tile {
        self.room_type.tile(tile)
    }

    pub fn is_wall(&self, position: (i32, i32)) -> bool {
        match self.tile(position).tile_type {
            TileType::Wall(_) => true,
            TileType::Door(direction) => {
                !self.doors[&direction.to_string()]
            },
            _ => false
        }
    }

    pub fn is_door(&self, position: (i32, i32)) -> bool {
        match self.tile(position).tile_type {
            TileType::Door(direction) => {
                self.doors[&direction.to_string()]
            },
            _ => false
        }
    }
}

impl Default for Room {
    fn default() -> Self {
        Room::new((0, 0))
    }
}

const BASIC_ROOM: [[TileType; ROOM_SIZE]; ROOM_SIZE] = [
    [TileType::None, TileType::None, TileType::None, TileType::None, TileType::None, TileType::None, TileType::Door(Direction::Up), TileType::Door(Direction::Up), TileType::Door(Direction::Up), TileType::Door(Direction::Up), TileType::None, TileType::None, TileType::None, TileType::None, TileType::None, TileType::None],
    [TileType::None, TileType::Wall(WallType::InnerNorthWest), TileType::Wall(WallType::North), TileType::Wall(WallType::North), TileType::Wall(WallType::North), TileType::Wall(WallType::North), TileType::Floor, TileType::Floor, TileType::Floor, TileType::Floor, TileType::Wall(WallType::North), TileType::Wall(WallType::North), TileType::Wall(WallType::North), TileType::Wall(WallType::North), TileType::Wall(WallType::InnerNorthEast), TileType::None],
    [TileType::None, TileType::Wall(WallType::West), TileType::Floor, TileType::Floor, TileType::Floor, TileType::Floor, TileType::Floor, TileType::Floor, TileType::Floor, TileType::Floor, TileType::Floor, TileType::Floor, TileType::Floor, TileType::Floor, TileType::Wall(WallType::East), TileType::None],
    [TileType::None, TileType::Wall(WallType::West), TileType::Floor, TileType::Floor, TileType::Floor, TileType::Floor, TileType::Floor, TileType::Floor, TileType::Floor, TileType::Floor, TileType::Floor, TileType::Floor, TileType::Floor, TileType::Floor, TileType::Wall(WallType::East), TileType::None],
    [TileType::None, TileType::Wall(WallType::West), TileType::Floor, TileType::Floor, TileType::Floor, TileType::Floor, TileType::Floor, TileType::Floor, TileType::Floor, TileType::Floor, TileType::Floor, TileType::Floor, TileType::Floor, TileType::Floor, TileType::Wall(WallType::East), TileType::None],
    [TileType::None, TileType::Wall(WallType::West), TileType::Floor, TileType::Floor, TileType::Floor, TileType::Floor, TileType::Floor, TileType::Floor, TileType::Floor, TileType::Floor, TileType::Floor, TileType::Floor, TileType::Floor, TileType::Floor, TileType::Wall(WallType::East), TileType::None],
    [TileType::None, TileType::Door(Direction::Left), TileType::Floor, TileType::Floor, TileType::Floor, TileType::Floor, TileType::Floor, TileType::Floor, TileType::Floor, TileType::Floor, TileType::Floor, TileType::Floor, TileType::Floor, TileType::Floor, TileType::Door(Direction::Right), TileType::None],
    [TileType::None, TileType::Door(Direction::Left), TileType::Floor, TileType::Floor, TileType::Floor, TileType::Floor, TileType::Floor, TileType::Floor, TileType::Floor, TileType::Floor, TileType::Floor, TileType::Floor, TileType::Floor, TileType::Floor, TileType::Door(Direction::Right), TileType::None],
    [TileType::None, TileType::Door(Direction::Left), TileType::Floor, TileType::Floor, TileType::Floor, TileType::Floor, TileType::Floor, TileType::Floor, TileType::Floor, TileType::Floor, TileType::Floor, TileType::Floor, TileType::Floor, TileType::Floor, TileType::Door(Direction::Right), TileType::None],
    [TileType::None, TileType::Door(Direction::Left), TileType::Floor, TileType::Floor, TileType::Floor, TileType::Floor, TileType::Floor, TileType::Floor, TileType::Floor, TileType::Floor, TileType::Floor, TileType::Floor, TileType::Floor, TileType::Floor, TileType::Door(Direction::Right), TileType::None],
    [TileType::None, TileType::Wall(WallType::West), TileType::Floor, TileType::Floor, TileType::Floor, TileType::Floor, TileType::Floor, TileType::Floor, TileType::Floor, TileType::Floor, TileType::Floor, TileType::Floor, TileType::Floor, TileType::Floor, TileType::Wall(WallType::East), TileType::None],
    [TileType::None, TileType::Wall(WallType::West), TileType::Floor, TileType::Floor, TileType::Floor, TileType::Floor, TileType::Floor, TileType::Floor, TileType::Floor, TileType::Floor, TileType::Floor, TileType::Floor, TileType::Floor, TileType::Floor, TileType::Wall(WallType::East), TileType::None],
    [TileType::None, TileType::Wall(WallType::West), TileType::Floor, TileType::Floor, TileType::Floor, TileType::Floor, TileType::Floor, TileType::Floor, TileType::Floor, TileType::Floor, TileType::Floor, TileType::Floor, TileType::Floor, TileType::Floor, TileType::Wall(WallType::East), TileType::None],
    [TileType::None, TileType::Wall(WallType::West), TileType::Floor, TileType::Floor, TileType::Floor, TileType::Floor, TileType::Floor, TileType::Floor, TileType::Floor, TileType::Floor, TileType::Floor, TileType::Floor, TileType::Floor, TileType::Floor, TileType::Wall(WallType::East), TileType::None],
    [TileType::None, TileType::Wall(WallType::InnerSouthWest), TileType::Wall(WallType::South), TileType::Wall(WallType::South), TileType::Wall(WallType::South), TileType::Wall(WallType::South), TileType::Floor, TileType::Floor, TileType::Floor, TileType::Floor, TileType::Wall(WallType::South), TileType::Wall(WallType::South), TileType::Wall(WallType::South), TileType::Wall(WallType::South), TileType::Wall(WallType::InnerSouthEast), TileType::None],
    [TileType::None, TileType::None, TileType::None, TileType::None, TileType::None, TileType::None, TileType::Door(Direction::Down), TileType::Door(Direction::Down), TileType::Door(Direction::Down), TileType::Door(Direction::Down), TileType::None, TileType::None, TileType::None, TileType::None, TileType::None, TileType::None],
];
