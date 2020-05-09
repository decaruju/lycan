mod http;
use lycan::shared::gamestate::{Gamestate, Player};
extern crate sfml;
use sfml::{
    graphics::{RenderWindow},
    window::{ContextSettings, Style},
};

mod game;
mod main_menu;
mod settings;

use game::{start_game, GameResult};
use main_menu::{main_menu, MenuChoice};

use settings::Settings;

struct ClientGamestate {
    pub gamestate: Gamestate,
    pub player_id: Option<String>,
    pub game_id: Option<String>,
}

impl ClientGamestate {
    fn load(_string: String) -> ClientGamestate {
        ClientGamestate {
            gamestate: Gamestate::default(),
            player_id: None,
            game_id: None,
        }
    }

    fn get_player(&mut self) -> Option<&mut Player> {
        match &self.player_id {
            Some(player_id) => Some(self.gamestate.players.get_mut(player_id)?),
            None => None,
        }
    }
}

fn main() {
    let mut gamestate = ClientGamestate::load(String::from("test"));
    match http::new_game() {
        Ok(game_id) => {
            println!("{}", game_id);
            gamestate.game_id = Some(game_id);
        },
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
            MenuChoice::StartGame => match start_game(&mut window) {
                GameResult::Menu => continue,
                GameResult::Quit => break,
            },
        }
    }
}
