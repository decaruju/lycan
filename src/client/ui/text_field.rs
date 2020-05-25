use sfml::{
    graphics::{
        Color, Drawable, FloatRect, Font, RectangleShape, RenderStates, RenderTarget, Shape, Text,
        Transformable,
    },
    system::{SfBox, Vector2},
};

pub struct TextField<'a> {
    pub text: String,
    pub title_text: Text<'a>,
    pub is_focus: bool,
    size: Vector2<f32>,
    background: RectangleShape<'a>,
    cursor: RectangleShape<'a>,
}

impl<'a> TextField<'a> {
    pub fn new(size: (f32, f32), font: &'a SfBox<Font>) -> Self {
        let size = Vector2::from(size);
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
        Self {
            text: String::new(),
            title_text: title_text,
            is_focus: false,
            size: size,
            background: background,
            cursor: cursor,
        }
    }

    pub fn set_position(&mut self, position: (f32, f32)) {
        self.background.set_position(position);

        let text_position = ((position.0 - self.size.x / 2.) + 10., position.1 - 20.);
        self.title_text.set_position(text_position);
        let text_size = self.title_text.local_bounds();
        let cursor_position = (text_position.0 + text_size.width + 10., text_position.1);
        self.cursor.set_position(cursor_position);
    }

    pub fn get_bounds(&mut self) -> FloatRect {
        self.background.global_bounds()
    }

    pub fn add_unicode_char(&mut self, unicode: char) {
        if unicode == '\u{8}' {
            self.text.pop();
            self.title_text.set_string(self.text.as_str());
        } else {
            self.text.push(unicode);
            self.title_text.set_string(self.text.as_str());
        }
    }
}

impl<'a> Drawable for TextField<'a> {
    fn draw<'s: 'shader, 'texture, 'shader, 'shader_texture>(
        &'s self,
        render_target: &mut dyn RenderTarget,
        _: RenderStates<'texture, 'shader, 'shader_texture>,
    ) {
        render_target.draw(&self.background);
        render_target.draw(&self.title_text);
        if self.is_focus {
            render_target.draw(&self.cursor);
        }
    }
}
