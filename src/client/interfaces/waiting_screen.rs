use crate::client_state::ClientGamestate;
use std::{
    thread,
    time::{Duration},
    sync::{Arc, RwLock},
};
use sfml::{
    graphics::{
        CircleShape,
        Color,
        Font,
        RenderTarget,
        RenderWindow,
        Shape,
        Transformable,
    },
    system::{
        Vector2,
    },
    window::{Event, Key},
};

use crate::menu::{
    menu::Menu,
    button::Button,
    text_field::TextField,
};

#[derive(Clone)]
pub enum WaitingScreenChoice {
    Back,
    Ready,
}

pub fn waiting_screen(window: &mut RenderWindow, font: &Font, gamestate: Arc<RwLock<ClientGamestate>>) -> WaitingScreenChoice {
    println!("waiting started");
    while !gamestate.read().unwrap().is_started() {
        while let Some(event) = window.poll_event() {
            if !window.has_focus() {
                continue
            }
            match event {
                Event::KeyPressed {code: Key::Return, ..} =>  {
                    let mut gamestate = gamestate.write().unwrap();
                    let mut player = gamestate.get_mut_player().unwrap();
                    player.ready = !player.ready;
                    println!("{}", player.ready);
                }
                Event::KeyPressed {code: Key::Escape, ..} =>  {
                    return WaitingScreenChoice::Back;
                }
                _ => {}
            }
        }
        window.clear(Color::BLACK);
        let mut circle = CircleShape::new(50., 100);
        circle.set_origin((50., 50.));
        circle.set_position(Vector2::from((window.size().x as f32/2., window.size().y as f32/2.)));
        if gamestate.read().unwrap().get_player().unwrap().ready {
            circle.set_fill_color(Color::GREEN);
        } else {
            circle.set_fill_color(Color::RED);
        }
        window.draw(&circle);
        window.display();
        thread::sleep(Duration::from_millis(15));
    }
    WaitingScreenChoice::Ready
}
