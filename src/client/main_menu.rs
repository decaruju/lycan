use sfml::{
    graphics::{
        CircleShape, Color, Drawable, Font, Rect, RectangleShape, RenderStates, RenderTarget,
        RenderWindow, Shape, Text, Transformable, View,
    },
    system::Vector2,
    window::{mouse, ContextSettings, Event, Key, Style},
};

use crate::ui::button;
use crate::Settings;

pub enum MenuChoice {
    StartGame,
    Quit,
}

pub fn main_menu(setting: &mut Settings, window: &mut RenderWindow) -> MenuChoice {
    let mut theball = the_ball();
    let font = Font::from_file("resources/VCR_OSD_MONO_1.001.ttf").unwrap();

    let mut startgame_button =
        button::MenuButton::new((300., 100.), String::from("start the game"));
    startgame_button.title_text.set_font(&font);

    let size = window.size();
    let mut center = window
        .map_pixel_to_coords_current_view(Vector2::from((size.x as i32 / 2, size.y as i32 / 2)));
    let mut menu_view = View::new(
        Vector2::from(center),
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
                Event::Resized { width, height } => {
                    center = window.map_pixel_to_coords(
                        Vector2::from((width as i32 / 2, height as i32 / 2)),
                        &menu_view,
                    );
                    menu_view.set_center(center);
                }
                Event::MouseButtonPressed { button, x, y } => match button {
                    mouse::Button::Left => {
                        println!("{} - {}", x, y);
                        if startgame_button
                            .background
                            .global_bounds()
                            .contains2(x as f32, y as f32)
                        {
                            return MenuChoice::StartGame;
                        }
                    }
                    _ => {}
                },
                _ => {}
            }
        }

        window.clear(Color::BLUE);
        window.set_view(&menu_view);

        startgame_button.set_position((center.x, center.y));

        window.draw(&startgame_button);
        window.draw(&theball);
        window.display();
    }
}

pub fn the_ball<'a>() -> CircleShape<'a> {
    let mut ball = CircleShape::default();
    ball.set_radius(20.);
    ball.set_origin((20., 20.));
    ball.set_fill_color(Color::BLACK);
    ball
}
