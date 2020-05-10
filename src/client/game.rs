use sfml::{
    graphics::{CircleShape, Color, RenderTarget, RenderWindow, Shape, Transformable, RectangleShape, View},
    window::{Event, Key},
};
use std::{
    sync::{Arc, RwLock},
    thread,
    time::Duration,
};
use crate::client_state::{ClientGamestate};
use lycan::shared::gamestate::{UpdateResponse, Room, Player};
use crate::http;

pub enum GameResult {
    Menu,
    Quit,
}

pub fn start_game(window: &mut RenderWindow, gamestate: Arc<RwLock<ClientGamestate>>) -> GameResult {
    let thread_gamestate = Arc::clone(&gamestate);
    thread::spawn(move || {
        loop {
            {
                let mut gamestate = thread_gamestate.write().unwrap();
                match http::update(&gamestate.get_game_id(), &gamestate.get_player_id(), gamestate.get_player().unwrap().position) {
                    Ok(data) => {
                        let new_state: UpdateResponse = serde_json::from_str(&data).unwrap();
                        gamestate.update(new_state);
                    },
                    Err(err) => println!("{}", err),
                };
            }
            thread::sleep(Duration::from_millis(15));
        }
    });

    loop {
        while let Some(event) = window.poll_event() {
            match event {
                Event::Closed
                | Event::KeyPressed {
                    code: Key::Escape, ..
                } => return GameResult::Quit,
                Event::MouseMoved { x, y } => {
                    // let gamestate = Arc::clone(&gamestate);
                    // let mut gamestate = gamestate.write().unwrap();
                    // gamestate.get_mut_player().unwrap().position = (x as f32, y as f32);
                }
                _ => {}
            }
        }

        if Key::Q.is_pressed() {
            return GameResult::Menu;
        }

        let mut movement = (0.0, 0.0);

        if Key::A.is_pressed() {
            movement.0 -= 10.0;
        }
        if Key::D.is_pressed() {
            movement.0 += 10.0;
        }
        if Key::W.is_pressed() {
            movement.1 -= 10.0;
        }
        if Key::S.is_pressed() {
            movement.1 += 10.0;
        }

        {
            let mut gamestate = gamestate.write().unwrap();

            let mut player = gamestate.get_mut_player().unwrap();
            player.move_player(movement);
            center_view(window, player);
        }


        draw(window, Arc::clone(&gamestate));
        thread::sleep(Duration::from_millis(1));
    }
}

pub fn center_view(window: &mut RenderWindow, player: &Player) {
    let player_position = window.map_coords_to_pixel_current_view(sfml::system::Vector2{x: player.position.0, y: player.position.1});
    let direction = if player_position.x - (window.size().x/2) as i32 > 10 {
        (10.0, 0.0)
    } else if (window.size().x/2) as i32 - player_position.x > 10 {
        (-10.0, 0.0)
    } else {
        (0.0, 0.0)
    };
    move_center(window, direction)
}

fn move_center(window: &mut RenderWindow, direction: (f32, f32)) {
    let view = window.view();
    let mut new_view = View::new(view.center(), view.size());
    let old_center = view.center();
    new_view.set_center((view.center().x + direction.0, view.center().y + direction.1));
    window.set_view(&new_view);
}

pub fn draw(window: &mut RenderWindow, gamestate: Arc<RwLock<ClientGamestate>>) {
    window.clear(Color::RED);
    for (position, room) in gamestate.read().unwrap().get_rooms() {
        draw_room(window, room);
    }
    for (id, player) in gamestate.read().unwrap().get_players() {
        window.draw(&ball(player.position));
    }
    window.display();
}

pub fn draw_room(window: &mut RenderWindow, room: &Room) {
    for i in 0..32 {
        for j in 0..32 {
            let x0 = (i * 32 + room.position.0*32*32) as f32;
            let y0 = (j * 32 + room.position.1*32*32) as f32;
            draw_rect(window, x0, y0, 32.0, 32.0);
        }
    }
}

pub fn draw_rect(window: &mut RenderWindow, x0: f32, y0: f32, width: f32, height: f32) {
    let mut rectangle = RectangleShape::default();
    rectangle.set_size((width, height));
    rectangle.set_outline_color(Color::BLUE);
    rectangle.set_outline_thickness(2.0);
    rectangle.set_position((x0, y0));
    window.draw(&rectangle);
}

pub fn ball<'a>(position: (f32, f32)) -> CircleShape<'a> {
    let mut ball = CircleShape::default();
    ball.set_radius(20.);
    ball.set_fill_color(Color::YELLOW);
    ball.set_origin((20. / 2., 20. / 2.));
    ball.set_position(position);
    ball
}
