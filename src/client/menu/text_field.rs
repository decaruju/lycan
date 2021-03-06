use sfml::{
    graphics::{
        Color,
        Drawable,
        FloatRect,
        Font,
        RectangleShape,
        RenderStates,
        RenderTarget,
        Shape,
        Text,
        Transformable,
    },
    system::{SfBox, Vector2, Vector2f, Vector2i},
};

use crate::menu::widget::{WidgetTrait, Widget};

pub struct TextField<'a, T> {
    pub text: String,
    pub title_text: Text<'a>,
    pub focused: bool,
    pub id: String,
    result: Option<T>,
    size: Vector2<f32>,
    background: RectangleShape<'a>,
    cursor: RectangleShape<'a>,
}

impl<'a, T> WidgetTrait<T> for TextField<'a, T> where T: Clone {
    fn set_position(&mut self, position: (f32, f32)) {
        self.background.set_position(position);

        let text_position = ((position.0 - self.size.x / 2.) + 10., position.1 - 20.);
        self.title_text.set_position(text_position);
        let text_size = self.title_text.local_bounds();
        let cursor_position = (text_position.0 + text_size.width + 10., text_position.1);
        self.cursor.set_position(cursor_position);
    }

    fn contains(&self, point: Vector2i) -> bool {
        self.background.global_bounds().contains(Vector2f{
            x: point.x as f32,
            y: point.y as f32,
        })
    }

    fn enter_char(&mut self, unicode: char) {
        if unicode == '\u{8}' {
            self.text.pop();
        } else {
            self.text.push(unicode);
        }
        self.title_text.set_string(self.text.as_str());
    }

    fn hover(&mut self, on: bool) {}

    fn focus(&mut self, on: bool) {
        self.focused = on;
    }

    fn click(&self) -> Option<T> {
        None
    }

    fn data(&self) -> Option<&str> {
        Some(&self.text)
    }

    fn id(&self) -> &str {
        &self.id
    }

    fn focused(&self) -> bool {
        self.focused
    }
}

impl<'a, T> TextField<'a, T> where T: Clone {
    pub fn new(font: &'a Font, id: String) -> Widget<'a, T> {
        let size = Vector2f{x: 300., y: 60.};
        let mut background = RectangleShape::with_size(size);
        let center = (size.x / 2., size.y / 2.);
        background.set_origin(center);
        background.set_fill_color(Color::BLACK);
        background.set_outline_color(Color::WHITE);
        background.set_outline_thickness(5.);
        let mut title_text = Text::default();
        title_text.set_fill_color(Color::RED);
        title_text.set_font(&font);
        let mut cursor =
            RectangleShape::with_size(Vector2::from((3., title_text.character_size() as f32)));
        cursor.set_fill_color(Color::WHITE);
        Widget::TextField(
            TextField {
                text: String::new(),
                title_text: title_text,
                focused: false,
                id,
                size: size,
                background: background,
                cursor: cursor,
                result: None,
            }
        )
    }
    pub fn get_bounds(&mut self) -> FloatRect {
        self.background.global_bounds()
    }
}

impl<'a, T> Drawable for TextField<'a, T> {
    fn draw<'s: 'shader, 'texture, 'shader, 'shader_texture>(
        &'s self,
        render_target: &mut dyn RenderTarget,
        _: RenderStates<'texture, 'shader, 'shader_texture>,
    ) {
        render_target.draw(&self.background);
        render_target.draw(&self.title_text);
        if self.focused {
            render_target.draw(&self.cursor);
        }
    }
}
