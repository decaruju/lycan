use sfml::{
    graphics::{
        Color, Drawable, FloatRect, Font, RectangleShape, RenderStates, RenderTarget, Shape, Text,
        Transformable,
    },
    system::{SfBox, Vector2},
};

use std::sync::Arc;

use crate::ui::ui_element::UiElement;

pub struct MenuButton<'a, T> where T: Clone {
    pub title_text: Text<'a>,
    pub background: RectangleShape<'a>,
    pub is_pressed: bool,
    pub result: T,
}

impl<'a, T> UiElement<T> for MenuButton<'a, T> where T: Clone {
    fn point_is_on(&self, x: i32, y: i32) -> bool {
        self.background.global_bounds().contains2(x as f32, y as f32)
    }

    fn hover(&mut self, x: i32, y: i32) {
        if self.point_is_on(x, y) {
            self.background.set_outline_color(Color::WHITE);
        } else {
            self.background.set_outline_color(Color::BLACK);
        }
    }

    fn click(&self) -> Option<T> {
        Some(self.result.clone())
    }

    fn text_entered(&mut self, text: char) {}

    fn set_position(&mut self, position: (f32, f32)) {
        self.background.set_position(position);
        self.title_text.set_position(position);
    }

    fn drawable(&self) -> Box<&Drawable> {
        Box::new(self)
    }
}

impl<'a, T: 'a> MenuButton<'a, T> where T: Clone {
    pub fn new(size: (f32, f32), title: String, result: T, font: &'a Font) -> Self {
        let size = Vector2::from(size);
        let mut background = RectangleShape::with_size(size);
        let center = (size.x / 2., size.y / 2.);
        background.set_origin(center);
        background.set_outline_color(Color::WHITE);
        background.set_outline_thickness(5.);

        let mut title_text = Text::default();
        title_text.set_string(title.as_str());
        title_text.set_fill_color(Color::RED);
        let text_size = title_text.local_bounds();
        let text_center = (text_size.width / 2., text_size.height / 2.);
        title_text.set_origin(text_center);
        title_text.set_font(font);
        Self {
            title_text: title_text,
            background: background,
            is_pressed: false,
            result: result,
        }
    }

    pub fn get_bounds(&mut self) -> FloatRect {
        self.background.global_bounds()
    }
}

impl<'a, T> Drawable for MenuButton<'a, T> where T: Clone {
    fn draw<'s: 'shader, 'texture, 'shader, 'shader_texture>(
        &'s self,
        render_target: &mut dyn RenderTarget,
        _: RenderStates<'texture, 'shader, 'shader_texture>,
    ) {
        render_target.draw(&self.background);
        render_target.draw(&self.title_text);
    }
}
