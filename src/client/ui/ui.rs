use sfml::{
    graphics::{
        Color, Drawable, RenderWindow, View, RenderTarget, Transformable, Font
    },
    window::{
        mouse,
        Event,
    },
    system::{Vector2, SfBox},
};

use crate::ui::ui_element::UiElement;

pub struct Ui<T> {
    elements: Vec<Box<dyn UiElement<T>>>,
    focused_index: Option<usize>,
    mousedown_coords: (i32, i32),
    size: Vector2<u32>,
    view: SfBox<View>,
}

impl<T> Ui<T> {
    pub fn new(size: Vector2<u32>) -> Ui<T> {
        let mut view = View::new(
            Vector2::from(((size.x/2) as f32, (size.y/2) as f32)),
            Vector2::from((size.x as f32, size.y as f32)),
        );

        Ui{
            elements: vec![],
            focused_index: None,
            mousedown_coords: (0, 0),
            size,
            view,
        }
    }

    pub fn handle(&mut self, window: &mut RenderWindow) -> T {
        loop {
            while let Some(event) = window.poll_event() {
                match event {
                    Event::MouseButtonPressed { button: mouse::Button::Left, x, y } => {
                        self.mousedown_coords = (x, y);
                    }

                    Event::MouseButtonReleased { button: mouse::Button::Left, x, y } => {
                        for (index, element) in self.elements.iter().enumerate() {
                            if element.point_is_on(x, y) && element.point_is_on(self.mousedown_coords.0, self.mousedown_coords.1) {
                                self.focused_index = Some(index);
                                if let Some(result) = element.click() {
                                    // return result
                                }
                            }
                        }
                    },

                    Event::MouseMoved { x, y } => {
                        for element in self.elements.iter_mut() {
                            element.hover(x, y);
                        }
                    }

                    Event::TextEntered { unicode } => {
                        // if let Some(element) = self.focused_element {
                        //     element.text_entered(unicode);
                        // }
                    }

                    _ => {}
                }
            }

            self.render(window);
        }
    }

    pub fn add_element(&mut self, element: Box<dyn UiElement<T>>) {
        self.elements.push(element);
    }

    fn render(&mut self, window: &mut RenderWindow) {
        window.clear(Color::rgb(60, 44, 41));
        for (index, element) in self.elements.iter_mut().enumerate() {
            element.set_position(((self.size.x/2) as f32, (self.size.y/2) as f32 + index as f32 * 100.));
            window.draw(*element.drawable());
        }
        window.display();
    }
}
