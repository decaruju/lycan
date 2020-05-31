use crate::menu::widget::Widget;

use std::collections::HashMap;

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
    widgets: Vec<Widget<'a, T>>,
    mousedown_coords: Vector2i,
}

impl<'a, T> Menu<'a, T> where T: Clone {
    pub fn new() -> Menu<'a, T> {
        Menu{
            widgets: vec![],
            mousedown_coords: Vector2i{x: 0, y: 0},
        }
    }

    pub fn add_widget(&mut self, widget: Widget<'a, T>) {
        self.widgets.push(widget);
    }

    pub fn data(&self) -> HashMap<&str, &str> {
        let mut hash = HashMap::new();
        for widget in self.widgets.iter() {
            if let Some(data) = widget.data() {
                hash.insert(widget.id(), data);
            }
        }
        hash
    }

    pub fn handle(&mut self, window: &mut RenderWindow) -> T {
        loop {
            while let Some(event) = window.poll_event() {
                match event {
                    Event::MouseMoved { x, y } => {
                        for widget in self.widgets.iter_mut() {
                            widget.hover(widget.contains(Vector2i{x, y}));
                        }
                    }

                    Event::MouseButtonPressed { button: mouse::Button::Left, x, y } => {
                        self.mousedown_coords = Vector2i{x, y};
                        for widget in self.widgets.iter_mut() {
                            widget.focus(widget.contains(Vector2i{x, y}));
                        }
                    }

                    Event::MouseButtonReleased { button: mouse::Button::Left, x, y } => {
                        for widget in self.widgets.iter_mut() {
                            if widget.contains(Vector2i{x, y}) {
                                if let Some(result) = widget.click() {
                                    return result
                                }
                            }
                        }
                    },

                    Event::TextEntered { unicode } => {
                        for widget in self.widgets.iter_mut() {
                            if widget.focused() {
                                widget.enter_char(unicode);
                            }
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
        for (index, widget) in self.widgets.iter_mut().enumerate() {
            widget.set_position((
                (size.x/2) as f32,
                (size.y/4) as f32 + index as f32 * 100.,
            ));
            window.draw(widget);
        }
        window.display();
    }
}
