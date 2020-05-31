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
        Text,
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
        let size = window.size();
        let mut title_text = Text::default();
        let mut game_id = Text::default();
        let mut circle = CircleShape::new(100., 100);
        if gamestate.read().unwrap().get_player().unwrap().ready {
            title_text.set_string("Waiting for players");
            circle.set_fill_color(Color::GREEN);
        } else {
            title_text.set_string("Press enter when ready");
            circle.set_fill_color(Color::RED);
        }
        title_text.set_font(font);
        game_id.set_font(font);
        game_id.set_string(gamestate.read().unwrap().game_id.as_ref().unwrap());
        let text_size = title_text.local_bounds();
        title_text.set_origin((text_size.width as f32/2., text_size.height as f32/2.));
        let game_id_size = game_id.local_bounds();
        game_id.set_origin((game_id_size.width as f32/2., game_id_size.height as f32/2.));
        title_text.set_position(Vector2::from((size.x as f32/2., size.y as f32/2. - 200.)));
        game_id.set_position(Vector2::from((size.x as f32/2., size.y as f32/2.)));
        circle.set_origin((100., 100.));
        circle.set_position(Vector2::from((size.x as f32/2., size.y as f32/2.)));
        if gamestate.read().unwrap().get_player().unwrap().ready {
        } else {
        }
        window.draw(&title_text);
        window.draw(&circle);
        window.draw(&game_id);
        window.display();
        thread::sleep(Duration::from_millis(15));
    }
    WaitingScreenChoice::Ready
}
