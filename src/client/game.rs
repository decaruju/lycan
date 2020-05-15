use crate::client_state::ClientGamestate;
use crate::displayer::Displayer;
use crate::http;
use lycan::shared::gamestate::{Gamestate, Player};
use lycan::shared::room::{Room, Tile, TileType, WallType};
use lycan::shared::utils::Direction;
use sfml::{
    graphics::{
        CircleShape, Color, IntRect, RectangleShape, RenderTarget, RenderWindow, Shape, Sprite,
        Texture, Transformable, View,
    },
    system::SfBox,
    window::{Event, Key},
};
use std::{
    sync::{Arc, RwLock},
    thread,
    time::Duration,
};

pub enum GameResult {
    Menu,
    Quit,
}

pub fn start_game(
    window: &mut RenderWindow,
    gamestate: Arc<RwLock<ClientGamestate>>,
) -> GameResult {
    let displayer = Displayer::new();
    let thread_gamestate = Arc::clone(&gamestate);
    thread::spawn(move || loop {
        {
            let game_id = thread_gamestate.read().unwrap().get_game_id().clone();
            let player_id = thread_gamestate.read().unwrap().get_player_id().clone();
            let position = thread_gamestate
                .read()
                .unwrap()
                .get_player()
                .unwrap()
                .position;
            let new_rooms = thread_gamestate.write().unwrap().get_new_rooms();
            match http::update(&game_id, &player_id, position, new_rooms) {
                Ok(data) => {
                    thread_gamestate.write().unwrap().update(data);
                }
                Err(err) => println!("{}", err),
            };
        }
        thread::sleep(Duration::from_millis(15));
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
        if Key::Z.is_pressed() {
            zoom_in(window);
        }
        if Key::X.is_pressed() {
            zoom_out(window);
        }

        {
            let mut gamestate = gamestate.write().unwrap();

            {
                let mut player = gamestate.get_mut_player().unwrap();
                player.move_player(movement);
            }
            if gamestate.player_in_door() {
                let player_room = gamestate.player_room();
                let position = player_room.position;
                if let TileType::Door(direction) =
                    player_room.tile(gamestate.player_tile()).tile_type
                {
                    gamestate.add_room(match direction {
                        Direction::Up => (position.0, position.1 + 1),
                        Direction::Down => (position.0, position.1 - 1),
                        Direction::Left => (position.0 - 1, position.1),
                        Direction::Right => (position.0 + 1, position.1),
                    });
                }
            } else if gamestate.player_in_wall() {
                let mut player = gamestate.get_mut_player().unwrap();
                player.move_player((-movement.0, -movement.1))
            }
            let player = gamestate.get_player().unwrap();
            center_view(window, player);
            // println!("player{:?}, in_wall{:?}, in_door{:?}, room{:?}, tile{:?}", gamestate.player_position(), gamestate.player_in_wall(), gamestate.player_in_wall(), gamestate.player_room_coord(), gamestate.player_tile());
        }

        displayer.display(window, Arc::clone(&gamestate));
        thread::sleep(Duration::from_millis(1));
    }
}

pub fn zoom_out(window: &mut RenderWindow) {
    let view = window.view();
    let mut new_view = View::new(view.center(), view.size());
    new_view.set_size(view.size() * 0.99);
    window.set_view(&new_view);
}

pub fn zoom_in(window: &mut RenderWindow) {
    let view = window.view();
    let mut new_view = View::new(view.center(), view.size());
    new_view.set_size(view.size() / 0.99);
    window.set_view(&new_view);
}

pub fn center_view(window: &mut RenderWindow, player: &Player) {
    let player_position = window.map_coords_to_pixel_current_view(sfml::system::Vector2 {
        x: player.position.0,
        y: player.position.1,
    });
    let direction = if player_position.x - (window.size().x / 2) as i32 > 100 {
        (10.0, 0.0)
    } else if (window.size().x / 2) as i32 - player_position.x > 100 {
        (-10.0, 0.0)
    } else if (window.size().y / 2) as i32 - player_position.y > 100 {
        (0.0, -10.0)
    } else if player_position.y - (window.size().y / 2) as i32 > 100 {
        (0.0, 10.0)
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

pub fn draw(
    window: &mut RenderWindow,
    gamestate: Arc<RwLock<ClientGamestate>>,
    wall_sprite: &Sprite,
) {
}
