use std::collections::HashMap;
use reqwest;

pub fn new_game() -> Result<String, Box<dyn std::error::Error>> {
    let client = reqwest::blocking::Client::new();
    match client.post("http://localhost:1337/new").send()?.text() {
        Ok(data) => {
            let res: serde_json::Value = serde_json::from_str(&data)?;
            if let serde_json::Value::Object(map) = res {
                if let Some(serde_json::Value::String(game_id)) = map.get("game_id") {
                    return Ok(game_id.to_string());
                }
            }
        },
        Err(err) => panic!(err),
    }
    Ok(String::from(""))
}

pub fn join_game(game_id: &str) -> Result<String, Box<dyn std::error::Error>> {
    let client = reqwest::blocking::Client::new();
    match client.post("http://localhost:1337/join")
        .body(serde_json::json!({ "game_id": game_id, "player_name": "foo" }).to_string())
        .send()?.text() {
            Ok(data) => {
                let res: serde_json::Value = serde_json::from_str(&data)?;
                if let serde_json::Value::Object(map) = res {
                    if let Some(serde_json::Value::String(player_id)) = map.get("player_id") {
                        return Ok(player_id.to_string());
                    }
                }
            },
            Err(err) => panic!(err),
        }
    Ok(String::from(""))
}

pub fn update(game_id: &str, player_id: &str, position: (f32, f32), new_rooms: Vec<(i32, i32)>) -> Result<String, Box<dyn std::error::Error>> {
    let client = reqwest::blocking::Client::new();
    match client.post("http://localhost:1337/update")
        .body(serde_json::json!({ "game_id": game_id, "player_id": player_id, "position": [position.0, position.1], "new_rooms": new_rooms }).to_string())
        .send()?
        .text() {
            Ok(data) => return Ok(data),
            Err(err) => panic!(err),
        }
}
