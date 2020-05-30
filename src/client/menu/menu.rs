use crate::menu::button::Button;
use crate::menu::text_field::TextField;

use sfml::{
    graphics::{
        Color,
        RenderTarget,
        RenderWindow,
    },
    window::{
        mouse,
        Event,
    },
    system::{
        Vector2i,
    },
};

pub struct Menu<'a, T> where T: Clone {
    buttons: Vec<Button<'a, T>>,
    text_field: Option<TextField<'a>>,
    mousedown_coords: Vector2i,
}

impl<'a, T> Menu<'a, T> where T: Clone {
    pub fn new() -> Menu<'a, T> {
        Menu{
            buttons: vec![],
            mousedown_coords: Vector2i{x: 0, y: 0},
            text_field: None,
        }
    }

    pub fn add_button(&mut self, button: Button<'a, T>) {
        self.buttons.push(button);
    }

    pub fn add_text_field(&mut self, text_field: TextField<'a>) {
        self.text_field = Some(text_field);
    }

    pub fn text(&self) -> String {
        if let Some(text_field) = &self.text_field {
            return text_field.text.clone();
        }
        String::from("")
    }

    pub fn handle(&mut self, window: &mut RenderWindow) -> T {
        if let Some(text_field) = &mut self.text_field {
            text_field.is_focus = true;
        }
        loop {
            while let Some(event) = window.poll_event() {
                match event {
                    Event::MouseMoved { x, y } => {
                        for button in self.buttons.iter_mut() {
                            button.hover(button.contains(Vector2i{x, y}));
                        }
                    }

                    Event::MouseButtonPressed { button: mouse::Button::Left, x, y } => {
                        self.mousedown_coords = Vector2i{x, y};
                        for button in self.buttons.iter_mut() {
                            if button.contains(Vector2i{x, y}) {
                                button.focus();
                            }
                        }
                    }

                    Event::MouseButtonReleased { button: mouse::Button::Left, x, y } => {
                        for button in self.buttons.iter_mut() {
                            if button.contains(Vector2i{x, y}) {
                                if let Some(result) = button.click() {
                                    return result
                                }
                            }
                        }
                    },

                    Event::TextEntered { unicode } => {
                        if let Some(text_field) = &mut self.text_field {
                            text_field.add_unicode_char(unicode);
                        }
                    },

                    _ => {},
                }
            }

            self.render(window);
        }
    }

    fn render(&mut self, window: &mut RenderWindow) {
        window.clear(Color::rgb(60, 44, 41));
        let size = window.size();
        let mut index = 0;
        if let Some(text_field) = &mut self.text_field {
            text_field.set_position((
                (size.x/2) as f32,
                (size.y/2) as f32 + index as f32 * 100.,
            ));
            index += 1;
            window.draw(text_field);
        }
        for button in self.buttons.iter_mut() {
            button.set_position((
                (size.x/2) as f32,
                (size.y/2) as f32 + index as f32 * 100.,
            ));
            index += 1;
            window.draw(button);
        }
        window.display();
    }
}
