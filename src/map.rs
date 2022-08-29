use super::Rect;
use rltk::{RandomNumberGenerator, Rltk, RGB};
use specs::prelude::*;
use specs_derive::Component;
use std::cmp::{max, min};

#[derive(PartialEq, Clone, Copy)]
pub enum TileType {
    Wall,
    Floor,
}

#[derive(Component)]
pub struct Map {
    pub rooms: Vec<Rect>,
    pub tiles: Vec<TileType>,
    pub height: i32,
    pub width: i32,
}

impl Map {
    pub fn xy_idx(&self, x: i32, y: i32) -> usize {
        (y as usize * self.width as usize) + x as usize
    }

    /// Makes a map with solid boundaries and 400 randomly placed walls. Just for testing.
    pub fn new_map_test() -> Map {
        let mut map = Map {
            tiles: vec![TileType::Floor; 80 * 50],
            rooms: Vec::new(),
            height: 50,
            width: 80,
        };

        // make the boundaries of the vector as Wall TileType
        for x in 0..map.width {
            // set the top edge to walls
            map.tiles[map.xy_idx(x, 0)] = TileType::Wall;
            // set the bottom edge to walls
            map.tiles[map.xy_idx(x, map.height - 1)] = TileType::Wall;
        }
        for y in 0..map.height {
            // set the left edge to walls
            map.tiles[map.xy_idx(0, y)] = TileType::Wall;
            // set the right edge to walls
            map.tiles[map.xy_idx(map.width - 1, y)] = TileType::Wall;
        }

        // randomly place walls around the inner part of the map
        // just for content for now
        let mut rng = rltk::RandomNumberGenerator::new();

        for _i in 0..400 {
            let x = rng.roll_dice(1, 79);
            let y = rng.roll_dice(1, 49);
            let idx = map.xy_idx(x, y);
            // place a random wall if the coords are not where the player spawns
            if idx != map.xy_idx(40, 25) {
                map.tiles[idx] = TileType::Wall;
            }
        }

        map
    }

    fn apply_room_to_map(&mut self, room: &Rect) {
        for y in room.y1 + 1..=room.y2 {
            for x in room.x1 + 1..=room.x2 {
                self.tiles[self.xy_idx(x, y)] = TileType::Floor;
            }
        }
    }

    fn apply_horizontal_tunnel(&mut self, x1: i32, x2: i32, y: i32) {
        for x in min(x1, x2)..=max(x1, x2) {
            let idx = self.xy_idx(x, y);
            let dimensions: usize = (self.width * self.height) as usize;
            if idx > 0 && idx < dimensions {
                self.tiles[idx as usize] = TileType::Floor;
            }
        }
    }

    fn apply_vertical_tunnel(&mut self, y1: i32, y2: i32, x: i32) {
        for y in min(y1, y2)..=max(y1, y2) {
            let idx = self.xy_idx(x, y);
            let dimensions: usize = (self.width * self.height) as usize;
            if idx > 0 && idx < dimensions {
                self.tiles[idx as usize] = TileType::Floor;
            }
        }
    }

    pub fn new() -> Map {
        let mut map = Map {
            tiles: vec![TileType::Wall; 80 * 50],
            rooms: Vec::new(),
            height: 50,
            width: 80,
        };

        const MAX_ROOMS: i32 = 30;
        const MIN_SIZE: i32 = 6;
        const MAX_SIZE: i32 = 10;

        let mut rng = RandomNumberGenerator::new();

        for _ in 0..MAX_ROOMS {
            let w = rng.range(MIN_SIZE, MAX_SIZE);
            let h = rng.range(MIN_SIZE, MAX_SIZE);
            let x = rng.roll_dice(1, map.width - w - 1) - 1;
            let y = rng.roll_dice(1, map.height - h - 1) - 1;
            let new_room = Rect::new(x, y, w, h);
            let mut ok = true;
            for other_room in map.rooms.iter() {
                if new_room.intersect(other_room) {
                    ok = false
                }
            }
            if ok {
                map.apply_room_to_map(&new_room);

                if !map.rooms.is_empty() {
                    let (new_x, new_y) = new_room.center();
                    let (prev_x, prev_y) = match map.rooms.last() {
                        Some(room) => room.center(),
                        None => (0, 0),
                    };

                    if rng.roll_dice(1, 2) == 1 {
                        map.apply_vertical_tunnel(prev_y, new_y, prev_x);
                        map.apply_horizontal_tunnel(prev_x, new_x, prev_x);
                    } else {
                        map.apply_horizontal_tunnel(prev_x, new_x, prev_y);
                        map.apply_vertical_tunnel(prev_y, new_y, prev_x);
                    }
                }

                map.rooms.push(new_room);
            }
        }

        map
    }

    pub fn draw_map(&self, ctx: &mut Rltk) {
        let mut x = 0;
        let mut y = 0;
        for tile in self.tiles.iter() {
            // map the tile type to a renderable representation
            match tile {
                TileType::Floor => {
                    ctx.set(
                        x,
                        y,
                        RGB::from_f32(0.5, 0.5, 0.5),
                        RGB::from_f32(0., 0., 0.),
                        rltk::to_cp437('.'),
                    );
                }
                TileType::Wall => {
                    ctx.set(
                        x,
                        y,
                        RGB::from_f32(0.0, 1.0, 0.0),
                        RGB::from_f32(0., 0., 0.),
                        rltk::to_cp437('#'),
                    );
                }
            }

            // Move to the next set of coordinates to draw
            x += 1;
            if x > 79 {
                x = 0;
                y += 1;
            }
        }
    }
}
