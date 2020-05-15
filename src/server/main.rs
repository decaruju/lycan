mod server_state;

use bytes::buf::BufExt;
use hyper::service::{make_service_fn, service_fn};
use hyper::{header, Body, Method, Request, Response, Server, StatusCode};
use std::fmt;
use std::sync::{Arc, RwLock};

use server_state::ServerState;

use serde::{Deserialize, Serialize};

use lycan::shared::http::{
    JoinGameRequest, JoinGameResponse, NewGameRequest, NewGameResponse, UpdateRequest,
    UpdateResponse,
};

type GenericError = Box<dyn std::error::Error + Send + Sync>;
type Result<T> = std::result::Result<T, GenericError>;

type State = Arc<RwLock<ServerState>>;

fn not_found<T>() -> Result<T> {
    Err(Box::new(HttpError::NotFound))
}

#[derive(fmt::Debug)]
enum HttpError {
    NotFound,
}

impl fmt::Display for HttpError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for HttpError {
    fn description(&self) -> &str {
        "not_found"
    }
}

fn data<T>(response: T) -> Result<String>
where
    T: Serialize,
{
    Ok(serde_json::to_string(&response)?)
}

async fn parse<'a, T>(request: Request<Body>) -> Result<T>
where
    T: for<'de> Deserialize<'de>,
{
    let body = hyper::body::aggregate(request).await?;
    match serde_json::from_reader(body.reader()) {
        Ok(data) => Ok(data),
        Err(err) => Err(Box::new(err)),
    }
}

fn join_game(request: JoinGameRequest, state: State) -> Result<String> {
    match state
        .write()
        .unwrap()
        .join_game(request.game_id, request.player_name)
    {
        Some(player_id) => data(JoinGameResponse { player_id }),
        None => not_found(),
    }
}

fn new_game(request: NewGameRequest, state: State) -> Result<String> {
    let game_id = state.write().unwrap().new_game(request.public);
    data(NewGameResponse { game_id })
}

fn update(request: UpdateRequest, state: State) -> Result<String> {
    match state.write().unwrap().update(
        request.game_id.clone(),
        request.player_id.clone(),
        request.position,
        request.new_rooms,
    ) {
        Some(gamestate) => data(UpdateResponse::new(gamestate)),
        None => not_found(),
    }
}

async fn router(req: Request<Body>, state: State) -> Result<Response<Body>> {
    let (method, path) = (req.method(), req.uri().path());

    let data = match (method, path) {
        (&Method::POST, "/new") => new_game(parse(req).await?, state),
        (&Method::POST, "/join") => join_game(parse(req).await?, state),
        (&Method::POST, "/update") => update(parse(req).await?, state),
        _ => return not_found(),
    };

    Ok(Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, "application/json")
        .body(Body::from(data?))?)
}

#[tokio::main]
async fn main() -> Result<()> {
    pretty_env_logger::init();

    let addr = "0.0.0.0:1337".parse().unwrap();

    let state: State = Arc::new(RwLock::new(ServerState::new()));

    let new_service = make_service_fn(move |_| {
        let state = Arc::clone(&state);
        async { Ok::<_, GenericError>(service_fn(move |req| router(req, Arc::clone(&state)))) }
    });

    let server = Server::bind(&addr).serve(new_service);

    println!("Listening on http://{}", addr);

    server.await?;

    Ok(())
}
