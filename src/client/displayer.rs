use crate::client_state::ClientGamestate;
use lycan::shared::room::{Room, Tile, TileType, WallType, Item};
use lycan::shared::utils::Direction;
use sfml::{
    graphics::{
        CircleShape, Color, IntRect, RenderTarget, RenderWindow, Shape, Sprite, Texture,
        RectangleShape,
        Transformable,
    },
    system::SfBox,
};
use std::sync::{Arc, RwLock};

pub struct Displayer {
    texture: SfBox<Texture>,
}

impl Displayer {
    pub fn new() -> Displayer {
        let texture = Texture::from_file("resources/cave_tileset.png").unwrap();

        Displayer { texture }
    }
    fn sprite(&self, x: i32, y: i32) -> Sprite {
        let mut sprite = Sprite::with_texture(&self.texture);
        sprite.set_texture_rect(&IntRect::new(x, y, 16, 16));
        sprite.set_origin((8., 8.));
        sprite
    }

    pub fn display(&self, window: &mut RenderWindow, gamestate: Arc<RwLock<ClientGamestate>>) {
        window.clear(Color::rgb(60, 44, 41));
        for room in gamestate.read().unwrap().get_rooms() {
            if gamestate.read().unwrap().explored(room.position) {
                self.draw_room(window, &room);
            }
        }
        for (id, player) in gamestate.read().unwrap().get_players() {
            let mut player_sprite = CircleShape::new(4.0, 100);
            player_sprite.set_origin((4.0, 4.0));
            player_sprite.set_position(player.position);
            window.draw(&player_sprite);
        }
        window.display();
    }

    fn draw_room(&self, window: &mut RenderWindow, room: &Room) {
        for i in 0..16 {
            for j in 0..16 {
                self.draw_tile(window, room, room.tile((i, j)));
            }
        }

        for (x, row) in room.items.iter() {
            for (y, item) in row.iter() {
                self.draw_item(window, (*x, *y), &item, room);
            }
        }
    }

    fn draw_item(&self, window: &mut RenderWindow, coord: (u32, u32), item: &Item, room: &Room) {
        let mut rect = RectangleShape::default();
        rect.set_position((
            coord.0 as f32 * 16.0 + room.position.0 as f32 * 16. * 16. + 8.,
            coord.1 as f32 * 16.0 + room.position.1 as f32 * 16. * 16. + 8.,
        ));
        rect.set_size((10., 20.));
        rect.set_origin((5., 10.));
        rect.set_fill_color(Color::CYAN);
        window.draw(&rect);
    }

    fn draw_tile(&self, window: &mut RenderWindow, room: &Room, tile: Tile) {
        let mut sprite = match tile.tile_type {
            TileType::Floor => self.sprite(18, 18),
            TileType::Door(direction) => {
                if room.doors[&direction.to_string()] {
                    self.sprite(18, 18)
                } else {
                    match direction {
                        Direction::Up => self.sprite(18, 52),
                        Direction::Down => self.sprite(18, 0),
                        Direction::Left => self.sprite(0, 18),
                        Direction::Right => self.sprite(35, 18),
                    }
                }
            }
            TileType::Wall(wall_type) => match wall_type {
                WallType::East => self.sprite(35, 18),
                WallType::West => self.sprite(0, 18),
                WallType::North => self.sprite(18, 0),
                WallType::South => self.sprite(18, 52),
                WallType::InnerNorthEast => self.sprite(35, 0),
                WallType::InnerNorthWest => self.sprite(0, 0),
                WallType::InnerSouthEast => self.sprite(35, 52),
                WallType::InnerSouthWest => self.sprite(0, 52),
                WallType::OuterNorthEast => self.sprite(52, 35),
                WallType::OuterNorthWest => self.sprite(69, 35),
                WallType::OuterSouthEast => self.sprite(52, 18),
                WallType::OuterSouthWest => self.sprite(69, 18),
                _ => return,
            },
            TileType::Exit => self.sprite(86, 103),
            _ => return,
        };
        sprite.set_position((
            tile.x as f32 * 16.0 + room.position.0 as f32 * 16. * 16. + 8.,
            tile.y as f32 * 16.0 + room.position.1 as f32 * 16. * 16. + 8.,
        ));
        window.draw(&sprite);
    }
}

fn ball<'a>(position: (f32, f32)) -> CircleShape<'a> {
    let mut ball = CircleShape::default();
    ball.set_radius(20.);
    ball.set_origin((20., 20.));
    ball.set_fill_color(Color::YELLOW);
    ball.set_position(position);
    ball
}
