use sfml::graphics::{Drawable};

pub trait UiElement<T>: Drawable {
    fn hover(&mut self, x: i32, y: i32);
    fn point_is_on(&self, x: i32, y: i32) -> bool;
    fn click(&self) -> Option<T>;
    fn text_entered(&mut self, text: char);
    fn set_position(&mut self, position: (f32, f32));
    fn drawable(&self) -> Box<&Drawable>;
}
