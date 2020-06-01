use sfml::{
    graphics::{
        Color,
        Drawable,
        Font,
        RectangleShape,
        RenderStates,
        RenderTarget,
        Shape,
        Text,
        Transformable,
    },
    system::{
        Vector2i,
        Vector2f,
    },
};
use crate::menu::widget::{WidgetTrait, Widget};

pub struct Label<'a, T> where T: Clone {
    pub title_text: Text<'a>,
    pub result: Option<T>,
    pub id: String,
}

impl<'a, T> WidgetTrait<T> for Label<'a, T> where T: Clone {
    fn contains(&self, point: Vector2i) -> bool {
        self.title_text.global_bounds().contains(Vector2f{
            x: point.x as f32,
            y: point.y as f32,
        })
    }

    fn hover(&mut self, on: bool) {}

    fn focus(&mut self, on: bool) {}

    fn click(&self) -> Option<T> { None }

    fn set_position(&mut self, position: (f32, f32)) {
        self.title_text.set_position(position);
    }

    fn data(&self) -> Option<&str> { None }

    fn id(&self) -> &str { &self.id }

    fn enter_char(&mut self, unicode: char) {}

    fn focused(&self) -> bool { false }
}

impl<'a, T> Label<'a, T> where T: Clone {
    pub fn new(text: &str, font: &'a Font, id: String) -> Widget<'a, T> {
        let mut title_text = Text::default();
        title_text.set_string(text);
        title_text.set_fill_color(Color::RED);
        title_text.set_font(font);
        let text_size = title_text.local_bounds();
        title_text.set_origin((text_size.width/2., text_size.height/2.));

        Widget::Label(
            Label{
                title_text,
                result: None,
                id
            }
        )
    }

}

impl<'a, T> Drawable for Label<'a, T> where T: Clone {
    fn draw<'s: 'shader, 'texture, 'shader, 'shader_texture>(
        &'s self,
        render_target: &mut dyn RenderTarget,
        _: RenderStates<'texture, 'shader, 'shader_texture>,
    ) {
        render_target.draw(&self.title_text);
    }
}
