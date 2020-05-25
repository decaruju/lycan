use sfml::{
    graphics::{
        CircleShape, Color, Drawable, Font, Rect, RectangleShape, RenderStates, RenderTarget,
        RenderWindow, Shape, Text, Transformable, View,
    },
    system::Vector2,
    window::{mouse, ContextSettings, Event, Key, Style},
};

use crate::ui::{button, text_field};
use crate::Settings;

pub enum MenuChoice {
    StartGame,
    Quit,
}

pub fn main_menu(setting: &mut Settings, window: &mut RenderWindow) -> MenuChoice {
    let font = Font::from_file("src/client/resources/VCR_OSD_MONO_1.001.ttf").unwrap();

    let mut startgame_button =
        button::MenuButton::new((300., 70.), String::from("start the game"), &font);
    let mut joingame_field = text_field::TextField::new((300., 70.), &font);

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
                Event::Resized { width, height } => {
                    center = window.map_pixel_to_coords(
                        Vector2::from((width as i32 / 2, height as i32 / 2)),
                        &menu_view,
                    );
                    menu_view.set_center(center);
                }
                Event::MouseButtonPressed { button, x, y } => match button {
                    mouse::Button::Left => {
                        if startgame_button.get_bounds().contains2(x as f32, y as f32) {
                            startgame_button.is_pressed = true;
                        }
                        if joingame_field.get_bounds().contains2(x as f32, y as f32) {
                            joingame_field.is_focus = true;
                        } else {
                            joingame_field.is_focus = false;
                        }
                    }
                    _ => {}
                },
                Event::MouseButtonReleased { button, x, y } => match button {
                    mouse::Button::Left => {
                        println!("{} - {}", x, y);
                        if startgame_button.get_bounds().contains2(x as f32, y as f32)
                            && startgame_button.is_pressed
                        {
                            return MenuChoice::StartGame;
                        }
                        if joingame_field.get_bounds().contains2(x as f32, y as f32) {
                            joingame_field.is_focus = true;
                        } else {
                            joingame_field.is_focus = false;
                        }
                    }
                    _ => {}
                },
                Event::MouseMoved { x, y } => {
                    startgame_button.mouse_hover(x as f32, y as f32);
                }

                Event::TextEntered { unicode } => {
                    if joingame_field.is_focus {
                        joingame_field.add_unicode_char(unicode);
                    }
                }
                _ => {}
            }
        }

        window.clear(Color::rgb(60, 44, 41));
        window.set_view(&menu_view);

        startgame_button.set_position((center.x, center.y));
        joingame_field.set_position((center.x, center.y + 100.));

        window.draw(&startgame_button);
        window.draw(&joingame_field);
        window.display();
    }
}
