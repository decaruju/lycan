use crate::client_state::ClientGamestate;
use lycan::shared::room::{Room, Tile, TileType, WallType};
use lycan::shared::utils::Direction;
use sfml::{
    graphics::{
        CircleShape, Color, IntRect, RenderTarget, RenderWindow, Shape, Sprite, Texture,
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
        sprite.set_texture_rect(&IntRect::new(x, y, 32, 32));
        sprite.set_origin((16., 16.));
        sprite
    }

    fn wall_sprite(&self) -> Sprite {
        self.sprite(0, 156)
    }

    fn inner_corner_sprite(&self) -> Sprite {
        self.sprite(0, 140)
    }

    fn outer_corner_sprite(&self) -> Sprite {
        self.sprite(0, 192)
    }

    fn floor_sprite(&self) -> Sprite {
        self.sprite(32, 156)
    }

    fn none_sprite(&self) -> Sprite {
        self.sprite(32, 188)
    }

    fn player_sprite(&self) -> Sprite {
        self.sprite(226, 128)
    }

    pub fn display(&self, window: &mut RenderWindow, gamestate: Arc<RwLock<ClientGamestate>>) {
        window.clear(Color::rgb(34, 32, 52));
        for room in gamestate.read().unwrap().get_rooms() {
            self.draw_room(window, &room);
        }
        for (id, player) in gamestate.read().unwrap().get_players() {
            let mut player_sprite = self.player_sprite();
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
    }

    fn draw_tile(&self, window: &mut RenderWindow, room: &Room, tile: Tile) {
        let mut sprite = match tile.tile_type {
            TileType::Floor => self.floor_sprite(),
            TileType::Door(direction) => {
                if room.doors[&direction.to_string()] {
                    self.floor_sprite()
                } else {
                    match direction {
                        Direction::Up => {
                            let mut wall_sprite = self.wall_sprite();
                            wall_sprite.rotate(280.);
                            wall_sprite
                        }
                        Direction::Down => {
                            let mut wall_sprite = self.wall_sprite();
                            wall_sprite.rotate(90.);
                            wall_sprite
                        }
                        Direction::Left => {
                            let mut wall_sprite = self.wall_sprite();
                            wall_sprite
                        }
                        Direction::Right => {
                            let mut wall_sprite = self.wall_sprite();
                            wall_sprite.rotate(180.);
                            wall_sprite
                        }
                    }
                }
            }
            TileType::Wall(wall_type) => match wall_type {
                WallType::East => {
                    let mut wall_sprite = self.wall_sprite();
                    wall_sprite.rotate(180.);
                    wall_sprite
                }
                WallType::West => {
                    let mut wall_sprite = self.wall_sprite();
                    wall_sprite
                }
                WallType::North => {
                    let mut wall_sprite = self.wall_sprite();
                    wall_sprite.rotate(270.);
                    wall_sprite
                }
                WallType::South => {
                    let mut wall_sprite = self.wall_sprite();
                    wall_sprite.rotate(90.);
                    wall_sprite
                }
                WallType::InnerNorthEast => {
                    let mut wall_sprite = self.inner_corner_sprite();
                    wall_sprite.rotate(180.);
                    wall_sprite
                }
                WallType::InnerNorthWest => {
                    let mut wall_sprite = self.inner_corner_sprite();
                    wall_sprite.rotate(270.);
                    wall_sprite
                }
                WallType::InnerSouthEast => {
                    let mut wall_sprite = self.inner_corner_sprite();
                    wall_sprite.rotate(90.);
                    wall_sprite
                }
                WallType::InnerSouthWest => {
                    let mut wall_sprite = self.inner_corner_sprite();
                    wall_sprite
                }
                WallType::OuterNorthEast => {
                    let mut wall_sprite = self.outer_corner_sprite();
                    wall_sprite.rotate(180.);
                    wall_sprite
                }
                WallType::OuterNorthWest => {
                    let mut wall_sprite = self.outer_corner_sprite();
                    wall_sprite.rotate(180.);
                    wall_sprite.scale((-1., 1.));
                    wall_sprite
                }
                WallType::OuterSouthWest => {
                    let mut wall_sprite = self.outer_corner_sprite();
                    wall_sprite
                }
                WallType::OuterSouthEast => {
                    let mut wall_sprite = self.outer_corner_sprite();
                    wall_sprite.scale((-1., 1.));
                    wall_sprite
                }
                _ => {
                    let mut wall_sprite = self.wall_sprite();
                    wall_sprite
                }
            },
            _ => self.none_sprite(),
        };
        sprite.set_position((
            tile.x as f32 * 32.0 + room.position.0 as f32 * 16. * 32. + 16.,
            tile.y as f32 * 32.0 + room.position.1 as f32 * 16. * 32. + 16.,
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
