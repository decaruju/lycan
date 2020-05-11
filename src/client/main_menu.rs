use sfml::{
    graphics::{
        CircleShape, Color, Drawable, Rect, RectangleShape, RenderStates, RenderTarget,
        RenderWindow, Shape, Transformable, View,
    },
    system::Vector2,
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
    circle: CircleShape<'a>,
}

impl<'a> MenuButton<'a> {
    fn new(title: String) -> Self {
        let size = Vector2::from((300., 100.));
        let mut texture = RectangleShape::with_size(size);
        texture.set_origin((size.x / 2., size.y / 2.));
        Self {
            title: title,
            texture: texture,
            circle: the_ball(),
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
        render_target.draw(&self.circle);
    }
}

pub fn main_menu(setting: &mut Settings, window: &mut RenderWindow) -> MenuChoice {
    let mut theball = the_ball();
    let mut startgame_button = MenuButton::new(String::from("start the game"));
    let mut menu_view = View::new(
        Vector2::from((0., 0.)),
        Vector2::from((window.size().x as f32, window.size().y as f32)),
    );
    loop {
        while let Some(event) = window.poll_event() {
            match event {
                Event::Closed
                | Event::KeyPressed {
                    code: Key::Escape, ..
                } => return MenuChoice::Quit,
                Event::MouseMoved { x, y } => theball
                    .set_position(window.map_pixel_to_coords(Vector2::from((x, y)), &menu_view)),
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
        window.clear(Color::BLUE);
        window.set_view(&menu_view);
        startgame_button.texture.set_position((100., 100.));
        window.draw(&startgame_button);
        window.draw(&theball);
        window.display();
    }
}

pub fn the_ball<'a>() -> CircleShape<'a> {
    let mut ball = CircleShape::default();
    ball.set_radius(20.);
    ball.set_fill_color(Color::BLACK);
    ball
}
