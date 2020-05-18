mod client_state;
mod displayer;
mod http;
extern crate sfml;
use sfml::{
    graphics::RenderWindow,
    window::{ContextSettings, Style},
};
use std::sync::{Arc, RwLock};

mod game;
mod main_menu;
mod settings;
mod ui;

use client_state::ClientGamestate;
use game::{start_game, GameResult};
use main_menu::{main_menu, MenuChoice};

use settings::Settings;

fn main() {
    let mut gamestate: Arc<RwLock<ClientGamestate>> =
        Arc::new(RwLock::new(ClientGamestate::load(String::from("test"))));
    match http::new_game() {
        Ok(game_id) => {
            println!("{}", game_id);
            let game_id = String::from("yes");
            match http::join_game(&game_id) {
                Ok(response) => {
                    let mut gamestate = gamestate.write().unwrap();
                    gamestate.set_game(game_id);
                    gamestate.set_player(response.player_id, response.position);
                }
                Err(err) => println!("{}", err),
            }
        }
        Err(err) => println!("{}", err),
    }

    let mut setting = Settings::default();

    let context_settings = ContextSettings {
        antialiasing_level: 0,
        ..Default::default()
    };

    let mut window = RenderWindow::new(
        setting.resolution,
        "Lycan",
        Style::DEFAULT,
        &context_settings,
    );
    window.set_vertical_sync_enabled(true);

    loop {
        match main_menu(&mut setting, &mut window) {
            MenuChoice::Quit => break,
            MenuChoice::StartGame => match start_game(&mut window, Arc::clone(&gamestate)) {
                GameResult::Menu => continue,
                GameResult::Quit => break,
            },
        }
    }
}
