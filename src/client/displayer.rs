use crate::client_state::ClientGamestate;
use lycan::shared::gamestate::{Player};
use lycan::shared::room::{Room, Tile, TileType, WallType, Item};
use lycan::shared::utils::Direction;
use sfml::{
    graphics::{
        CircleShape, Color, IntRect, RenderTarget, RenderWindow, Shape, Sprite, Texture,
        RectangleShape,
        Transformable,
        View,
    },
    system::{SfBox, Vector2, Vector2u},
};
use std::sync::{Arc, RwLock};
use std::rc::Rc;

pub struct Displayer {
    texture: SfBox<Texture>,
    game_view: SfBox<View>,
    hud_view: SfBox<View>,
    size: Vector2u,
}

impl Displayer {
    pub fn new(size: Vector2u) -> Displayer {
        let texture = Texture::from_file("resources/cave_tileset.png").unwrap();

        Displayer {
            size,
            texture,
            game_view: View::new(
                Vector2::from((size.x as f32/2., size.y as f32/2.)),
                Vector2::from((size.x as f32, size.y as f32)),
            ),
            hud_view: View::new(
                Vector2::from((size.x as f32/2., size.y as f32/2.)),
                Vector2::from((size.x as f32, size.y as f32)),
            ),
        }
    }

    pub fn zoom_in(&mut self) {
        self.game_view.zoom(1./0.99);
    }

    pub fn zoom_out(&mut self) {
        self.game_view.zoom(0.99);
    }

    pub fn set_center(&mut self, center: (f32, f32)) {
        self.game_view.set_center(center)
    }

    pub fn move_center(&mut self, direction: (f32, f32)) {
        let old_center = self.game_view.center();
        self.game_view.set_center(
            (
                old_center.x + direction.0,
                old_center.y + direction.1,
            ),
        );
    }

    fn sprite(&self, x: i32, y: i32) -> Sprite {
        let mut sprite = Sprite::with_texture(&self.texture);
        sprite.set_texture_rect(&IntRect::new(x, y, 16, 16));
        sprite.set_origin((8., 8.));
        sprite
    }

    pub fn center_view(&mut self, window: &RenderWindow, player: &Player) {
        let player_position = window.map_coords_to_pixel(
            sfml::system::Vector2 {
                x: player.position.0,
                y: player.position.1,
            },
            &self.game_view,
        );
        let center_x = (self.size.x / 2) as i32;
        let center_y = (self.size.y / 2) as i32;
        let buffer_x = (self.size.x/4) as i32;
        let buffer_y = (self.size.y/4) as i32;
        let direction = (
            if (player_position.x - center_x > buffer_x) {
                std::cmp::min(player_position.x - center_x - buffer_x, 3)
            } else if (center_x - player_position.x > buffer_x) {
                std::cmp::max(player_position.x - center_x + buffer_x, -3)
            } else {
                0
            } as f32,
            if (player_position.y - center_y > buffer_y) {
                std::cmp::min(player_position.y - center_y - buffer_y, 3)
            } else if (center_y - player_position.y > buffer_y) {
                std::cmp::max(player_position.y - center_y + buffer_y, -3)
            } else {
                0
            } as f32,
        );
        self.move_center(direction)
    }

    pub fn display(&mut self, window: &mut RenderWindow, gamestate: Arc<RwLock<ClientGamestate>>) {
        window.clear(Color::rgb(60, 44, 41));
        self.game_view.set_rotation(gamestate.read().unwrap().rotation);
        window.set_view(&self.game_view);
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
        window.set_view(&self.hud_view);
        self.display_hud(window, &gamestate);
        window.display();
    }

    fn display_hud(&mut self, window: &mut RenderWindow, gamestate: &Arc<RwLock<ClientGamestate>>) {

        println!("{}", gamestate.read().unwrap().gamestate.keys);
        let keys = gamestate.read().unwrap().gamestate.keys;
        for i in 0..keys {
            let mut rect = RectangleShape::new();
            rect.set_position((
                i as f32 * 20. + 40.,
                40.,
            ));
            rect.set_size((10., 20.));
            rect.set_origin((5., 10.));
            rect.set_fill_color(Color::CYAN);
            window.draw(&rect);
        }
        for i in keys..8 {
            let mut rect = RectangleShape::new();
            rect.set_position((
                i as f32 * 20. + 40.,
                40.,
            ));
            rect.set_outline_thickness(2.);
            rect.set_size((6., 16.));
            rect.set_origin((3., 8.));
            rect.set_outline_color(Color::CYAN);
            rect.set_fill_color(Color::TRANSPARENT);
            window.draw(&rect);
        }
    }

    fn draw_room(&mut self, window: &mut RenderWindow, room: &Room) {
        for i in 0..16 {
            for j in 0..16 {
                self.draw_tile(window, room, room.tile((i, j)));
            }
        }

        self.draw_item(window, &room.item, room);
    }

    fn draw_item(&mut self, window: &mut RenderWindow, item: &Option<(Item, (u32, u32))>, room: &Room) {
        if let Some(item) = item {
            let mut rect = RectangleShape::default();
            let coord = item.1;
            rect.set_position((
                coord.0 as f32 * 16.0 + room.position.0 as f32 * 16. * 16. + 8.,
                coord.1 as f32 * 16.0 + room.position.1 as f32 * 16. * 16. + 8.,
            ));
            rect.set_size((10., 20.));
            rect.set_origin((5., 10.));
            rect.set_fill_color(Color::CYAN);
            window.draw(&rect);
        }
    }

    fn draw_tile(&mut self, window: &mut RenderWindow, room: &Room, tile: Tile) {
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
