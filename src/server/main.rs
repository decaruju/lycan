mod server_state;

use serde::Deserialize;

use std::collections::HashMap;
use std::{
    sync::{Arc, RwLock},
};
use bytes::buf::BufExt;
use hyper::service::{make_service_fn, service_fn};
use hyper::{header, Body, Method, Request, Response, Server, StatusCode};

use server_state::ServerState;

type GenericError = Box<dyn std::error::Error + Send + Sync>;
type Result<T> = std::result::Result<T, GenericError>;

type State = Arc<RwLock<ServerState>>;

fn not_found() -> Result<Response<Body>> {
    return Ok(Response::builder().status(StatusCode::NOT_FOUND).body(Body::empty())?)
}

fn bad_request() -> Result<Response<Body>> {
    return Ok(Response::builder().status(StatusCode::BAD_REQUEST).body(Body::empty())?)
}

#[derive(Deserialize)]
struct UpdatePayload {
    game_id: String,
    player_id: String,
    position: (f32, f32),
    new_rooms: Vec<(i32, i32)>,
}

async fn update(req: Request<Body>, state: State) -> Result<Response<Body>> {
    let whole_body = hyper::body::aggregate(req).await?;
    let data: serde_json::Value = serde_json::from_reader(whole_body.reader())?;

    let payload: UpdatePayload = serde_json::from_value(data).unwrap();
    let mut state = state.write().unwrap();
    println!("{:?}", payload.new_rooms);
    match state.update(payload.game_id.clone(), payload.player_id.clone(), payload.position, payload.new_rooms) {
        Some(game_state) => {
            let response = Response::builder()
                .status(StatusCode::OK)
                .header(header::CONTENT_TYPE, "application/json")
                .body(Body::from(serde_json::to_string(&game_state).unwrap()))?;
            return Ok(response)
        },
        None => {
            return not_found();
        },
    }
    return bad_request();
}

async fn join_game(req: Request<Body>, state: State) -> Result<Response<Body>> {
    let whole_body = hyper::body::aggregate(req).await?;
    let data: serde_json::Value = serde_json::from_reader(whole_body.reader())?;
    println!("{}", data);

    if let serde_json::Value::Object(map) = data {
        if let (Some(serde_json::Value::String(game_id)), Some(serde_json::Value::String(player_name))) = (map.get("game_id"), map.get("player_name")) {
            let mut state = state.write().unwrap();
            match state.join_game(game_id.clone(), player_name.clone()) {
                Some(player_id) => {
                    let response = Response::builder()
                        .status(StatusCode::OK)
                        .header(header::CONTENT_TYPE, "application/json")
                        .body(Body::from(serde_json::json!({ "player_id": player_id }).to_string()))?;
                    return Ok(response)
                },
                None => {
                    return not_found();
                },
            };
        }
    }
    return bad_request();
}

async fn new_game(_req: Request<Body>, state: State) -> Result<Response<Body>> {
    let uuid = state.write().unwrap().new_game();
    let response = Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, "application/json")
        .body(Body::from(serde_json::json!({ "game_id": uuid }).to_string()))?;
    Ok(response)
}

async fn router(
    req: Request<Body>,
    state: State,
) -> Result<Response<Body>> {

    match (req.method(), req.uri().path()) {
        (&Method::POST, "/join") => join_game(req, state).await,
        (&Method::POST, "/update") => update(req, state).await,
        (&Method::POST, "/new") => new_game(req, state).await,
        _ => not_found(),
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    pretty_env_logger::init();

    let addr = "0.0.0.0:1337".parse().unwrap();

    let state: State = Arc::new(RwLock::new(ServerState::new()));

    let new_service = make_service_fn(move |_| {
        let state = Arc::clone(&state);
        async {
            Ok::<_, GenericError>(service_fn(move |req| {
                router(req, Arc::clone(&state))
            }))
        }
    });

    let server = Server::bind(&addr).serve(new_service);

    println!("Listening on http://{}", addr);

    server.await?;

    Ok(())
}
