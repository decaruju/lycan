use serde::{Deserialize, Serialize};
use std::fmt::Debug;

pub const ROOM_SIZE: usize = 16;

#[derive(Debug, Serialize, Deserialize, Clone)]
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
