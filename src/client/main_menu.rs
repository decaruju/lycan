use sfml::{
    graphics::{
        CircleShape, Color, Drawable, Font, Rect, RectangleShape, RenderStates, RenderTarget,
        RenderWindow, Shape, Text, Transformable, View,
    },
    system::Vector2,
    window::{mouse, ContextSettings, Event, Key, Style},
};

use crate::Settings;

pub enum MenuChoice {
    StartGame,
    Quit,
}

struct MenuButton<'a> {
    title: String,
    title_text: Text<'a>,
    position: Vector2<f32>,
    background: RectangleShape<'a>,
    size: Vector2<f32>,
}

impl<'a> MenuButton<'a> {
    fn new(title: String) -> Self {
        let size = Vector2::from((300., 100.));
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

    fn set_position(&mut self, position: (f32, f32)) {
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

pub fn main_menu(setting: &mut Settings, window: &mut RenderWindow) -> MenuChoice {
    let mut theball = the_ball();
    let font = Font::from_file("resources/VCR_OSD_MONO_1.001.ttf").unwrap();

    let mut startgame_button = MenuButton::new(String::from("start the game"));
    startgame_button.title_text.set_font(&font);

    let size = window.size();
    let mut center = window
        .map_pixel_to_coords_current_view(Vector2::from((size.x as i32 / 2, size.y as i32 / 2)));
    let mut menu_view = View::new(
        Vector2::from(center),
        Vector2::from((window.size().x as f32, window.size().y as f32)),
    );
    loop {
        while let Some(event) = window.poll_event() {
            match event {
                Event::Closed
                | Event::KeyPressed {
                    code: Key::Escape, ..
                } => return MenuChoice::Quit,
                Event::MouseMoved { x, y } => theball
                    .set_position(window.map_pixel_to_coords(Vector2::from((x, y)), &menu_view)),
                Event::Resized { width, height } => {
                    center = window.map_pixel_to_coords(
                        Vector2::from((width as i32 / 2, height as i32 / 2)),
                        &menu_view,
                    );
                    menu_view.set_center(center);
                }
                _ => {}
            }
        }

        if Key::S.is_pressed() {
            return MenuChoice::StartGame;
        }
        if mouse::Button::Left.is_pressed() {}
        if Key::Space.is_pressed() {}

        window.clear(Color::BLUE);
        window.set_view(&menu_view);

        startgame_button.set_position((center.x, center.y));

        window.draw(&startgame_button);
        window.draw(&theball);
        window.display();
    }
}

pub fn the_ball<'a>() -> CircleShape<'a> {
    let mut ball = CircleShape::default();
    ball.set_radius(20.);
    ball.set_fill_color(Color::BLACK);
    ball
}
