use sfml::{
    graphics::{CircleShape, Color, RenderTarget, RenderWindow, Shape, Transformable},
    window::{Event, Key},
};

pub enum GameResult {
    Menu,
    Quit,
}

pub fn start_game(window: &mut RenderWindow) -> GameResult {
    let mut theball = the_ball();
    loop {
        while let Some(event) = window.poll_event() {
            match event {
                Event::Closed
                | Event::KeyPressed {
                    code: Key::Escape, ..
                } => return GameResult::Quit,
                Event::MouseMoved { x, y } => theball.set_position((x as f32, y as f32)),
                _ => {}
            }
        }

        if Key::Q.is_pressed() {
            return GameResult::Menu;
        }
        window.clear(Color::RED);
        window.draw(&theball);
        window.display();
    }
}

pub fn the_ball<'a>() -> CircleShape<'a> {
    let mut ball = CircleShape::default();
    ball.set_radius(20.);
    ball.set_fill_color(Color::YELLOW);
    ball.set_origin((20. / 2., 20. / 2.));
    ball
}
