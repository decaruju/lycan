use reqwest;

use serde::{Deserialize, Serialize};

use lycan::shared::http::{
    JoinGameRequest, JoinGameResponse, NewGameRequest, NewGameResponse, UpdateRequest,
    UpdateResponse,
};

type ClientResult<T> = Result<T, Box<dyn std::error::Error>>;

fn post<'a, T, S>(url: &str, payload: T) -> Result<S, Box<dyn std::error::Error>> where T: Serialize, S: for<'de> Deserialize<'de> {
    let client = reqwest::blocking::Client::new();
    match client
        .post(url)
        .body(serde_json::to_string(&payload)?)
        .send() {
            Ok(data) => Ok(data.json()?),
            Err(err) => Err(Box::new(err)),
        }
}



pub fn new_game() -> ClientResult<String> {
    let response: NewGameResponse = post(
        "http://localhost:1337/new",
        NewGameRequest{public: true},
    ).unwrap();
    Ok(response.game_id)
}

pub fn join_game(game_id: &str) -> ClientResult<String> {
    let response: JoinGameResponse = post(
        "http://localhost:1337/join",
        JoinGameRequest{game_id: game_id.to_string(), player_name: "".to_string()},
    ).unwrap();
    Ok(response.player_id)
}

pub fn update(
    game_id: &str,
    player_id: &str,
    position: (f32, f32),
    new_rooms: Vec<(i32, i32)>,
) -> ClientResult<UpdateResponse> {
    let response: UpdateResponse = post(
        "http://localhost:1337/update",
        UpdateRequest{
            game_id: game_id.to_string(),
            player_id: player_id.to_string(),
            position,
            new_rooms,
        },
    ).unwrap();

    Ok(response)
}
