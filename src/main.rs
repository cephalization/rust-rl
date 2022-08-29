use rltk::{GameState, RandomNumberGenerator, Rltk, RGB};
use specs::prelude::*;

mod map;
pub use map::*;
mod components;
pub use components::*;
mod player;
pub use player::*;
mod rect;
pub use rect::*;
mod visibility_system;
use visibility_system::VisibilitySystem;

// STATE
pub struct State {
    ecs: World,
}
impl GameState for State {
    fn tick(&mut self, ctx: &mut Rltk) {
        ctx.cls();

        player_input(self, ctx);
        self.run_systems();

        let positions = self.ecs.read_storage::<Position>();
        let renderables = self.ecs.read_storage::<Renderable>();

        Map::draw_map(&self.ecs, ctx);
        for (pos, render) in (&positions, &renderables).join() {
            ctx.set(pos.x, pos.y, render.fg, render.bg, render.glyph);
        }
    }
}
impl State {
    fn run_systems(&mut self) {
        let mut vis = VisibilitySystem {};
        vis.run_now(&self.ecs);
        self.ecs.maintain();
    }
}

// MAIN

fn main() -> rltk::BError {
    use rltk::RltkBuilder;
    let context = RltkBuilder::simple80x50()
        .with_fps_cap(60.)
        .with_title("Rust RL")
        .build()?;
    let mut gs = State { ecs: World::new() };
    gs.ecs.register::<Player>();
    gs.ecs.register::<Position>();
    gs.ecs.register::<Renderable>();
    gs.ecs.register::<Viewshed>();
    gs.ecs.register::<Map>();
    // generate a vec of room coords and update map tiles accordingly
    let main_map = Map::new();
    let mut rng = RandomNumberGenerator::new();
    let (map_center_x, map_center_y) = main_map.rooms[rng.range(0, main_map.rooms.len())].center();
    gs.ecs.insert(main_map);
    // create the player and place them in the center of a random room
    gs.ecs
        .create_entity()
        .with(Position {
            x: map_center_x,
            y: map_center_y,
        })
        .with(Renderable {
            glyph: rltk::to_cp437('@'),
            fg: RGB::named(rltk::YELLOW),
            bg: RGB::named(rltk::BLACK),
        })
        .with(Viewshed {
            visible_tiles: Vec::new(),
            range: 8,
            dirty: true,
        })
        .with(Player {})
        .build();

    rltk::main_loop(context, gs)
}
