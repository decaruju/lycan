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
    pub background: RectangleShape<'a>,
}

impl<'a> TextField<'a> {
    pub fn new(size: (f32, f32), font: &'a SfBox<Font>) -> Self {
        let size = Vector2::from(size);
        let mut background = RectangleShape::with_size(size);
        let center = (size.x / 2., size.y / 2.);
        background.set_origin(center);
        // TODO only work if run from client folder
        let mut title_text = Text::default();
        title_text.set_fill_color(Color::RED);
        title_text.set_font(&font);
        Self {
            text: String::new(),
            title_text: title_text,
            background: background,
        }
    }

    pub fn set_position(&mut self, position: (f32, f32)) {
        self.background.set_position(position);

        let text_size = self.title_text.local_bounds();
        let text_center = (text_size.width / 2., text_size.height / 2.);
        self.title_text.set_origin(text_center);
        self.title_text.set_position(position);
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
    }
}
