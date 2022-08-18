use rltk::{Rltk, RGB};

#[derive(PartialEq, Clone, Copy)]
pub enum TileType {
    Wall,
    Floor,
}

pub fn xy_idx(x: i32, y: i32) -> usize {
    (y as usize * 80) + x as usize
}

pub fn new_map_test() -> Vec<TileType> {
    let mut map = vec![TileType::Floor; 80 * 50];

    // make the boundaries of the vector as Wall TileType
    for x in 0..80 {
        // set the top edge to walls
        map[xy_idx(x, 0)] = TileType::Wall;
        // set the bottom edge to walls
        map[xy_idx(x, 49)] = TileType::Wall;
    }
    for y in 0..50 {
        // set the left edge to walls
        map[xy_idx(0, y)] = TileType::Wall;
        // set the right edge to walls
        map[xy_idx(79, y)] = TileType::Wall;
    }

    // randomly place walls around the inner part of the map
    // just for content for now
    let mut rng = rltk::RandomNumberGenerator::new();

    for _i in 0..400 {
        let x = rng.roll_dice(1, 79);
        let y = rng.roll_dice(1, 49);
        let idx = xy_idx(x, y);
        // place a random wall if the coords are not where the player spawns
        if idx != xy_idx(40, 25) {
            map[idx] = TileType::Wall;
        }
    }

    map
}

pub fn draw_map(map: &[TileType], ctx: &mut Rltk) {
    let mut x = 0;
    let mut y = 0;
    for tile in map.iter() {
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
