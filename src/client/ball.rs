extern crate sfml;

use sfml::{
    graphics::{CircleShape, Color, RenderTarget, RenderWindow, Shape, Transformable},
    window::{ContextSettings, Event, Key, Style},
};

pub fn the_ball<'a>() -> CircleShape<'a> {
    let mut ball = CircleShape::default();
    ball.set_radius(20.);
    ball.set_fill_color(Color::BLACK);
    ball.set_origin((20. / 2., 20. / 2.));
    ball
}
