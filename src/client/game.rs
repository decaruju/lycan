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
    system::{SfBox, Vector2},
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

    {
        let mut gamestate = gamestate.write().unwrap();
        gamestate.add_player_room();
    }

    let thread_gamestate = Arc::clone(&gamestate);
    thread::spawn(move || loop {
        {
            let game_id = thread_gamestate.read().unwrap().get_game_id().clone();
            let player_id = thread_gamestate.read().unwrap().get_player_id().clone();
            let position = thread_gamestate.read().unwrap().get_player().unwrap().position;
            let ready = thread_gamestate.read().unwrap().get_player().unwrap().ready;
            let new_rooms = thread_gamestate.write().unwrap().get_new_rooms();
            let cleared_rooms = thread_gamestate.write().unwrap().get_cleared_rooms();
            let end = thread_gamestate.write().unwrap().end;
            match http::update(&game_id, &player_id, position, new_rooms, cleared_rooms, ready, end) {
                Ok(data) => {
                    thread_gamestate.write().unwrap().update(data);
                }
                Err(err) => println!("{}", err),
            };
        }
        thread::sleep(Duration::from_millis(15));
    });

    let mut displayer = Displayer::new(window.size());

    while !gamestate.read().unwrap().is_started() {
        while let Some(event) = window.poll_event() {
            if !window.has_focus() {
                continue
            }
            match event {
                Event::KeyPressed {code: Key::Return, ..} =>  {
                    let mut gamestate = gamestate.write().unwrap();
                    let mut player = gamestate.get_mut_player().unwrap();
                    player.ready = !player.ready;
                    println!("{}", player.ready);
                }
                _ => {}
            }
        }
        window.clear(Color::BLACK);
        let mut circle = CircleShape::new(50., 100);
        circle.set_origin((50., 50.));
        circle.set_position(Vector2::from((window.size().x as f32/2., window.size().y as f32/2.)));
        if gamestate.read().unwrap().get_player().unwrap().ready {
            circle.set_fill_color(Color::GREEN);
        } else {
            circle.set_fill_color(Color::RED);
        }
        window.draw(&circle);
        window.display();
        thread::sleep(Duration::from_millis(15));
    }

    {
        let gamestate = gamestate.read().unwrap();
        let position = gamestate.player_position();
        displayer.set_center(position);
    }

    loop {
        while let Some(event) = window.poll_event() {
            if !window.has_focus() {
                continue
            }
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
        if window.has_focus() {
            if Key::A.is_pressed() {
                movement.0 -= 3.0;
            }
            if Key::D.is_pressed() {
                movement.0 += 3.0;
            }
            if Key::W.is_pressed() {
                movement.1 -= 3.0;
            }
            if Key::S.is_pressed() {
                movement.1 += 3.0;
            }
            if Key::Z.is_pressed() {
                displayer.zoom_in();
            }
            if Key::X.is_pressed() {
                displayer.zoom_out();
            }
        }

        {
            let mut gamestate = gamestate.write().unwrap();

            {
                let mut player = gamestate.get_mut_player().unwrap();
                player.move_player(movement);
            }
            if let Some(player_room) = gamestate.player_room() {
                if gamestate.player_in_door() {
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
                } else if gamestate.player_in_exit() {
                    if gamestate.gamestate.keys == 8 {
                        gamestate.end = true;
                    } else {
                        let mut player = gamestate.get_mut_player().unwrap();
                        player.move_player((-movement.0, -movement.1))
                    }
                }
                if gamestate.player_on_item() {
                    gamestate.remove_item();
                }
            }
            let player = gamestate.get_player().unwrap();
            displayer.center_view(window, player);
            // println!("player{:?}, in_wall{:?}, in_door{:?}, room{:?}, tile{:?}", gamestate.player_position(), gamestate.player_in_wall(), gamestate.player_in_wall(), gamestate.player_room_coord(), gamestate.player_tile());
        }

        displayer.display(window, Arc::clone(&gamestate));
        thread::sleep(Duration::from_millis(15));
    }
}

pub fn draw(
    window: &mut RenderWindow,
    gamestate: Arc<RwLock<ClientGamestate>>,
    wall_sprite: &Sprite,
) {
}
