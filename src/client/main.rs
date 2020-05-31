mod client_state;
mod displayer;
mod http;
extern crate sfml;
use sfml::{
    graphics::{Font, RenderWindow, RenderTarget},
    window::{ContextSettings, Style},
    system::{SfBox}
};
use std::{
    thread,
    sync::{Arc, RwLock},
    time::Duration,
};

mod game;
mod settings;
mod menu;
mod interfaces;

use interfaces::{
    main_menu::{
        main_menu,
        MainMenuChoice,
    },
    name_entry::{
        name_entry,
        NameEntryChoice,
    },
    waiting_screen::{
        waiting_screen,
        WaitingScreenChoice,
    },
    game_join::{
        game_join,
        GameJoinChoice,
    },
};

use client_state::ClientGamestate;
use game::{start_game, GameResult};
// use main_menu::{MenuChoice};

use settings::Settings;


fn main() {
    let font = Font::from_file("src/client/resources/VCR_OSD_MONO_1.001.ttf").unwrap().to_owned();
    let mut gamestate: Arc<RwLock<ClientGamestate>> = Arc::new(
        RwLock::new(
            ClientGamestate::default(),
        ),
    );

    let setting = Settings::default();

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
        match main_menu(&mut window, &font) {
            MainMenuChoice::Quit => break,
            MainMenuChoice::NewGame => {
                new_game(Arc::clone(&gamestate));
                match name_entry(&mut window, &font) {
                    NameEntryChoice::Back => continue,
                    NameEntryChoice::Name(name) => {
                        join_game(Arc::clone(&gamestate), &name);

                        start_update_loop(Arc::clone(&gamestate));
                        match waiting_screen(&mut window, &font, Arc::clone(&gamestate)) {
                            WaitingScreenChoice::Back => continue,
                            WaitingScreenChoice::Ready => {
                                match start_game(&mut window, Arc::clone(&gamestate)) {
                                    GameResult::Menu => continue,
                                    GameResult::Quit => break,
                                };
                            }
                        };
                    },
                };
            },
            MainMenuChoice::JoinGame => {
                match game_join(&mut window, &font) {
                    GameJoinChoice::Back => continue,
                    GameJoinChoice::Game(game_id) => {
                        gamestate.write().unwrap().set_game(game_id);

                        match name_entry(&mut window, &font) {
                            NameEntryChoice::Back => continue,
                            NameEntryChoice::Name(name) => {
                                join_game(Arc::clone(&gamestate), &name);

                                start_update_loop(Arc::clone(&gamestate));
                                match waiting_screen(&mut window, &font, Arc::clone(&gamestate)) {
                                    WaitingScreenChoice::Back => continue,
                                    WaitingScreenChoice::Ready => {
                                        match start_game(&mut window, Arc::clone(&gamestate)) {
                                            GameResult::Menu => continue,
                                            GameResult::Quit => break,
                                        };
                                    }
                                };
                            },
                        };
                    }
                }
            }
        }
    }
}

fn new_game(gamestate: Arc<RwLock<ClientGamestate>>) {
    match http::new_game() {
        Ok(game_id) => {
            println!("{}", game_id);
            let mut gamestate = gamestate.write().unwrap();
            gamestate.set_game(game_id);
        }
        Err(err) => println!("{}", err),
    }
}

fn join_game(gamestate: Arc<RwLock<ClientGamestate>>, player_name: &str) {
    let game_id = gamestate.read().unwrap().game_id.as_ref().unwrap().clone();
    match http::join_game(&game_id, player_name) {
        Ok(response) => {
            println!("ok");
            println!("{:?}", response);
            let mut gamestate = gamestate.write().unwrap();
            gamestate.set_game(game_id);
            gamestate.set_player(response.player_id, response.position);
            println!("game set");
        }
        Err(err) => println!("{}", err),
    }
}


fn start_update_loop(gamestate: Arc<RwLock<ClientGamestate>>) {
    let thread_gamestate = Arc::clone(&gamestate);
    thread::spawn(move || loop {
        {
            let game_id = thread_gamestate.read().unwrap().get_game_id().clone();
            let player_id = thread_gamestate.read().unwrap().get_player_id().clone();
            let position = thread_gamestate.read().unwrap().get_player().unwrap().position;
            let ready = thread_gamestate.read().unwrap().get_player().unwrap().ready;
            let new_rooms = thread_gamestate.write().unwrap().get_new_rooms();
            let cleared_rooms = thread_gamestate.write().unwrap().get_cleared_rooms();
            let end = thread_gamestate.write().unwrap().end;
            match http::update(&game_id, &player_id, position, new_rooms, cleared_rooms, ready, end) {
                Ok(data) => {
                    thread_gamestate.write().unwrap().update(data);
                }
                Err(err) => println!("{}", err),
            };
        }
        thread::sleep(Duration::from_millis(15));
    });
}
