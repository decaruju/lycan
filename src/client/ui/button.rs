use sfml::{
    graphics::{
        Color, Drawable, FloatRect, Font, RectangleShape, RenderStates, RenderTarget, Shape, Text,
        Transformable,
    },
    system::{SfBox, Vector2},
};

pub struct MenuButton<'a> {
    pub title_text: Text<'a>,
    pub background: RectangleShape<'a>,
    position: Vector2<f32>,
    size: Vector2<f32>,
}

impl<'a> MenuButton<'a> {
    pub fn new(size: (f32, f32), title: String, font: &'a SfBox<Font>) -> Self {
        let size = Vector2::from(size);
        let position = Vector2::from((0., 0.));
        let mut background = RectangleShape::with_size(size);
        let center = (size.x / 2., size.y / 2.);
        background.set_origin(center);
        // TODO only work if run from client folder
        let mut title_text = Text::default();
        title_text.set_string(title.as_str());
        title_text.set_fill_color(Color::RED);
        title_text.set_font(&font);
        let text_size = title_text.global_bounds();
        let text_center = (
            text_size.top + text_size.height / 2.,
            text_size.left + text_size.width / 2.,
        );
        title_text.set_origin(text_center);
        println!("{:?}", text_center);
        Self {
            title_text: title_text,
            position: position,
            background: background,
            size: size,
        }
    }

    pub fn set_position(&mut self, position: (f32, f32)) {
        self.background.set_position(position);
        self.title_text.set_position(position);
    }
    pub fn get_bounds(&mut self) -> FloatRect {
        self.background.global_bounds()
    }
}

impl<'a> Drawable for MenuButton<'a> {
    fn draw<'s: 'shader, 'texture, 'shader, 'shader_texture>(
        &'s self,
        render_target: &mut dyn RenderTarget,
        _: RenderStates<'texture, 'shader, 'shader_texture>,
    ) {
        render_target.draw(&self.background);
        render_target.draw(&self.title_text);
    }
}
