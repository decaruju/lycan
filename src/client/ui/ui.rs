use sfml::{
    graphics::{
        Color, Drawable, RenderWindow, View, RenderTarget, Transformable
    },
    window::{
        mouse,
        Event,
    },
    system::{Vector2, SfBox},
};

use crate::ui::ui_element::UiElement;

pub struct Ui<'a, T> {
    elements: Vec<Box<dyn UiElement<T>>>,
    focused_index: Option<usize>,
    window: &'a mut RenderWindow,
    mousedown_coords: (i32, i32),
    center: (f32, f32),
    view: SfBox<View>,
}

impl<'a, T> Ui<'a, T> {
    pub fn new(window: &'a mut RenderWindow) -> Ui<'a, T> {
        let size = window.size();
        let mut center = window.map_pixel_to_coords_current_view(Vector2::from((size.x as i32 / 2, size.y as i32 / 2)));
        let mut view = View::new(
            Vector2::from(center),
            Vector2::from((window.size().x as f32, window.size().y as f32)),
        );

        Ui::<'a>{
            elements: vec![],
            window,
            focused_index: None,
            mousedown_coords: (0, 0),
            center: (center.x, center.y),
            view,
        }
    }

    pub fn handle(&mut self) -> T {
        loop {
            while let Some(event) = self.window.poll_event() {
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

            self.render();
        }
    }

    pub fn add_element(&mut self, element: Box<dyn UiElement<T>>) {
        self.elements.push(element);
    }

    fn render(&mut self) {
        self.window.clear(Color::rgb(60, 44, 41));
        for (index, element) in self.elements.iter_mut().enumerate() {
            element.set_position((self.center.0 as f32, self.center.0 as f32 + index as f32 * 100.));
            self.window.draw(*element.drawable());
        }
        self.window.display();
    }
}
