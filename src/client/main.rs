use lycan::shared::gamestate::Gamestate;
extern crate sfml;
use sfml::{
    graphics::{CircleShape, Color, RenderTarget, RenderWindow, Shape, Transformable},
    window::{ContextSettings, Event, Key, Style},
};

mod game;
mod main_menu;
mod settings;

use game::{start_game, GameResult};
use main_menu::{main_menu, MenuChoice};

use settings::Settings;

struct ClientGamestate {
    pub gamestate: Gamestate,
}

impl ClientGamestate {
    fn load(string: String) -> ClientGamestate {
        ClientGamestate {
            gamestate: Gamestate::default(),
        }
    }
}

fn main() {
    let gamestate = ClientGamestate::load(String::from("test"));

    let mut setting = Settings::default();

    let context_settings = ContextSettings {
        antialiasing_level: 0,
        ..Default::default()
    };

    let mut window =
        RenderWindow::new(setting.resolution, "Lycan", Style::CLOSE, &context_settings);
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
