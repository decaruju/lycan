use lycan::shared::gamestate::Gamestate;
extern crate sfml;

use sfml::{
    graphics::{CircleShape, Color, RenderTarget, RenderWindow, Shape, Transformable},
    window::{ContextSettings, Event, Key, Style},
};

mod ball;

struct ClientGamestate {
    pub gamestate: Gamestate,
}

impl ClientGamestate {
    fn load(string: String) -> ClientGamestate {
        ClientGamestate {
            gamestate: Gamestate { test: string },
        }
    }
}

fn main() {
    let gamestate = ClientGamestate::load(String::from("test"));
    println!("{}", gamestate.gamestate.test);

    let game_width = 800;
    let game_height = 600;

    let context_settings = ContextSettings {
        antialiasing_level: 0,
        ..Default::default()
    };

    let mut window = RenderWindow::new(
        (game_width, game_height),
        "Lycan",
        Style::CLOSE,
        &context_settings,
    );
    window.set_vertical_sync_enabled(true);

    let mut theball = ball::the_ball();

    loop {
        while let Some(event) = window.poll_event() {
            match event {
                Event::Closed
                | Event::KeyPressed {
                    code: Key::Escape, ..
                } => return,
                Event::MouseMoved { x, y } => theball.set_position((x as f32, y as f32)),
                _ => {}
            }
        }

        if Key::Space.is_pressed() {}

        window.clear(Color::BLUE);
        window.draw(&theball);
        window.display();
    }
}
