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

pub struct Button<'a, T> where T: Clone {
    pub title_text: Text<'a>,
    pub background: RectangleShape<'a>,
    pub result: T,
    pub id: String,
    pub focused: bool,
}

impl<'a, T> WidgetTrait<T> for Button<'a, T> where T: Clone {
    fn contains(&self, point: Vector2i) -> bool {
        self.background.global_bounds().contains(Vector2f{
            x: point.x as f32,
            y: point.y as f32,
        })
    }

    fn hover(&mut self, on: bool) {
        if on {
            self.background.set_outline_color(Color::BLUE);
        } else {
            self.background.set_outline_color(Color::WHITE);
        }
    }

    fn focus(&mut self, on: bool) {
        self.focused = on;
        if on {
            self.background.set_outline_color(Color::RED);
        } else {
            self.background.set_outline_color(Color::WHITE);
        }
    }

    fn click(&self) -> Option<T> {
        Some(self.result.clone())
    }

    fn set_position(&mut self, position: (f32, f32)) {
        self.background.set_position(position);
        self.title_text.set_position(position);
    }

    fn data(&self) -> Option<&str> {
        None
    }

    fn id(&self) -> &str {
        &self.id
    }

    fn enter_char(&mut self, unicode: char) {}

    fn focused(&self) -> bool {
        self.focused
    }
}

impl<'a, T> Button<'a, T> where T: Clone {
    pub fn new(text: &str, font: &'a Font, result: T, id: String) -> Widget<'a, T> {
        let size = Vector2i{x: 300, y: 60};
        let mut background = RectangleShape::with_size(Vector2f{
            x: size.x as f32,
            y: size.y as f32,
        });
        background.set_origin(Vector2f{
            x: size.x as f32/2.,
            y: size.y as f32/2.,
        });
        background.set_fill_color(Color::YELLOW);
        background.set_outline_thickness(5.);

        let mut title_text = Text::default();
        title_text.set_string(text);
        title_text.set_fill_color(Color::RED);
        title_text.set_font(font);
        let text_size = title_text.local_bounds();
        title_text.set_origin((text_size.width/2., text_size.height/2.));

        Widget::Button(
            Button{
                title_text,
                background,
                result,
                id,
                focused: false,
            }
        )
    }

}

impl<'a, T> Drawable for Button<'a, T> where T: Clone {
    fn draw<'s: 'shader, 'texture, 'shader, 'shader_texture>(
        &'s self,
        render_target: &mut dyn RenderTarget,
        _: RenderStates<'texture, 'shader, 'shader_texture>,
    ) {
        render_target.draw(&self.background);
        render_target.draw(&self.title_text);
    }
}
