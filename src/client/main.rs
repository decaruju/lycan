use lycan::shared::gamestate::Gamestate;
extern crate sfml;

use sfml::{
    graphics::{CircleShape, Color, RenderTarget, RenderWindow, Shape, Transformable},
    window::{ContextSettings, Event, Key, Style},
};

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

    let mut ball = CircleShape::default();
    ball.set_radius(20.);
    ball.set_fill_color(Color::BLACK);
    ball.set_origin((20. / 2., 20. / 2.));
    ball.set_position((game_width as f32 / 2., game_height as f32 / 2.));

    loop {
        while let Some(event) = window.poll_event() {
            match event {
                Event::Closed
                | Event::KeyPressed {
                    code: Key::Escape, ..
                } => return,
                Event::MouseMoved { x, y } => ball.set_position((x as f32, y as f32)),
                _ => {}
            }
        }

        if Key::Space.is_pressed() {}

        window.clear(Color::BLUE);
        window.draw(&ball);
        window.display();
    }
}
