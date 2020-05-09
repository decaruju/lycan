use sfml::{
    graphics::{CircleShape, Color, RenderTarget, RenderWindow, Shape, Transformable},
    window::{Event, Key},
};
use std::{
    sync::{Arc, RwLock},
    thread,
    time::Duration,
};
use crate::client_state::{ClientGamestate};
use lycan::shared::gamestate::{UpdateResponse};
use crate::http;

pub enum GameResult {
    Menu,
    Quit,
}

pub fn start_game(window: &mut RenderWindow, gamestate: Arc<RwLock<ClientGamestate>>) -> GameResult {
    let thread_gamestate = Arc::clone(&gamestate);
    thread::spawn(move || {
        loop {
            {
                let mut gamestate = thread_gamestate.write().unwrap();
                match http::update(&gamestate.get_game_id(), &gamestate.get_player_id(), gamestate.get_player().unwrap().position) {
                    Ok(data) => {
                        let new_state: UpdateResponse = serde_json::from_str(&data).unwrap();
                        gamestate.update(new_state);
                    },
                    Err(err) => println!("{}", err),
                };
            }
            thread::sleep(Duration::from_millis(15));
        }
    });

    loop {
        while let Some(event) = window.poll_event() {
            match event {
                Event::Closed
                | Event::KeyPressed {
                    code: Key::Escape, ..
                } => return GameResult::Quit,
                Event::MouseMoved { x, y } => {
                    let gamestate = Arc::clone(&gamestate);
                    let mut gamestate = gamestate.write().unwrap();
                    gamestate.get_mut_player().unwrap().position = (x as f32, y as f32);
                }
                _ => {}
            }
        }

        if Key::Q.is_pressed() {
            return GameResult::Menu;
        }


        window.clear(Color::RED);
        for (id, player) in Arc::clone(&gamestate).read().unwrap().get_players() {
            window.draw(&ball(player.position));
        }
        window.display();
        thread::sleep(Duration::from_millis(1));
    }
}

pub fn ball<'a>(position: (f32, f32)) -> CircleShape<'a> {
    let mut ball = CircleShape::default();
    ball.set_radius(20.);
    ball.set_fill_color(Color::YELLOW);
    ball.set_origin((20. / 2., 20. / 2.));
    ball.set_position(position);
    ball
}
