use sfml::{
    graphics::{
        Color, Drawable, Rect, RectangleShape, RenderStates, RenderTarget, Text, Transformable,
    },
    system::Vector2,
};

pub struct MenuButton<'a> {
    title: String,
    pub title_text: Text<'a>,
    position: Vector2<f32>,
    pub background: RectangleShape<'a>,
    size: Vector2<f32>,
}

impl<'a> MenuButton<'a> {
    pub fn new(size: (f32, f32), title: String) -> Self {
        let size = Vector2::from(size);
        let position = Vector2::from((0., 0.));
        let mut background = RectangleShape::with_size(size);
        let center = (size.x / 2., size.y / 2.);
        background.set_origin(center);
        // TODO only work if run from client folder
        let mut title_text = Text::default();
        title_text.set_string(title.as_str());
        title_text.set_fill_color(Color::RED);
        title_text.set_origin(center);
        Self {
            title: title,
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
