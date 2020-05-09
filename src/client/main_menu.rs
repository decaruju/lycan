use sfml::{
    graphics::{
        CircleShape, Color, Rect, RectangleShape, RenderTarget, RenderWindow, Shape, Transformable,
    },
    system::Vector2f,
    window::{ContextSettings, Event, Key, Style},
};

use crate::Settings;

pub enum MenuChoice {
    StartGame,
    Quit,
}

struct MenuButton<'a> {
    title: String,
    rect: Rect<f32>,
    texture: RectangleShape<'a>,
}

impl<'a> MenuButton<'a> {
    fn new(title: String) -> MenuButton<'a> {
        let size = Vector2f::from((30., 10.));
        let pos = Vector2f::from((0., 0.));
        MenuButton {
            title: title,
            rect: Rect::from_vecs(size, pos),
            texture: RectangleShape::with_size(size),
        }
    }
}

pub fn main_menu(setting: &mut Settings, window: &mut RenderWindow) -> MenuChoice {
    let mut theball = the_ball();
    loop {
        while let Some(event) = window.poll_event() {
            match event {
                Event::Closed
                | Event::KeyPressed {
                    code: Key::Escape, ..
                } => return MenuChoice::Quit,
                Event::MouseMoved { x, y } => theball.set_position((x as f32, y as f32)),
                _ => {}
            }
        }

        if Key::S.is_pressed() {
            return MenuChoice::StartGame;
        }

        if Key::Space.is_pressed() {
            println!("{:?}", window.size());
        }
        window.clear(Color::BLUE);
        window.draw(&theball);
        window.display();
    }
}

pub fn the_ball<'a>() -> CircleShape<'a> {
    let mut ball = CircleShape::default();
    ball.set_radius(20.);
    ball.set_fill_color(Color::BLACK);
    ball.set_origin((20. / 2., 20. / 2.));
    ball
}
