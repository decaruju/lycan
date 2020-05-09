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
