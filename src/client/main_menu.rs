use sfml::{
    graphics::{
        CircleShape, Color, Drawable, Rect, RectangleShape, RenderStates, RenderTarget,
        RenderWindow, Shape, Transformable,
    },
    system::Vector2f,
    window::{mouse, ContextSettings, Event, Key, Style},
};

use crate::Settings;

pub enum MenuChoice {
    StartGame,
    Quit,
}

struct MenuButton<'a> {
    title: String,
    texture: RectangleShape<'a>,
}

impl<'a> MenuButton<'a> {
    fn new(title: String) -> Self {
        let size = Vector2f::from((300., 100.));
        Self {
            title: title,
            texture: RectangleShape::with_size(size),
        }
    }
}

impl<'a> Drawable for MenuButton<'a> {
    fn draw<'s: 'shader, 'texture, 'shader, 'shader_texture>(
        &'s self,
        render_target: &mut dyn RenderTarget,
        _: RenderStates<'texture, 'shader, 'shader_texture>,
    ) {
        render_target.draw(&self.texture);
    }
}

pub fn main_menu(setting: &mut Settings, window: &mut RenderWindow) -> MenuChoice {
    let mut theball = the_ball();
    let mut startgame_button = MenuButton::new(String::from("start the game"));
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

        if mouse::Button::Left.is_pressed() {
            println!("{:?}", window.mouse_position());
        }

        if Key::Space.is_pressed() {
            println!("{:?}", window.default_view());
        }
        let size = window.size();
        startgame_button
            .texture
            .set_position(((size.x as f32) / 2.0, (size.y as f32) / 2.0));
        window.clear(Color::BLUE);
        window.draw(&startgame_button);
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
