use rltk::{GameState, Point, RandomNumberGenerator, Rltk, RGB};
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
mod monster_ai_system;
pub use monster_ai_system::*;

// STATE
#[derive(PartialEq, Clone, Copy)]
pub enum RunState {
    Paused,
    Running,
}

pub struct State {
    pub ecs: World,
    pub runstate: RunState,
}
impl GameState for State {
    fn tick(&mut self, ctx: &mut Rltk) {
        ctx.cls();

        if self.runstate == RunState::Running {
            self.run_systems();
            self.runstate = RunState::Paused;
        } else {
            self.runstate = player_input(self, ctx);
        }

        let positions = self.ecs.read_storage::<Position>();
        let renderables = self.ecs.read_storage::<Renderable>();
        let map = self.ecs.fetch::<Map>();

        // draw both visible and revealed tiles on the map
        Map::draw_map(&self.ecs, ctx);
        // draw all other renderables that are within the vec of visible tiles on the map
        for (pos, render) in (&positions, &renderables).join() {
            let xy_idx = map.xy_idx(pos.x, pos.y);
            if map.visible_tiles[xy_idx] {
                ctx.set(pos.x, pos.y, render.fg, render.bg, render.glyph);
            }
        }
    }
}
impl State {
    fn run_systems(&mut self) {
        let mut vis = VisibilitySystem {};
        let mut monster_ai = MonsterAI {};
        vis.run_now(&self.ecs);
        monster_ai.run_now(&self.ecs);
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
    let mut gs = State {
        ecs: World::new(),
        runstate: RunState::Running,
    };
    gs.ecs.register::<Player>();
    gs.ecs.register::<Monster>();
    gs.ecs.register::<Position>();
    gs.ecs.register::<Renderable>();
    gs.ecs.register::<Viewshed>();
    gs.ecs.register::<Map>();
    gs.ecs.register::<Name>();
    // generate a Map for placing entities
    let main_map = Map::new();
    let mut rng = RandomNumberGenerator::new();
    let player_spawn_room = rng.range(0, main_map.rooms.len());
    let (map_center_x, map_center_y) = main_map.rooms[player_spawn_room].center();

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
        .with(Name {
            name: "Player".to_string(),
        })
        .build();

    // create an enemy in each room other than the player's
    let mut rng = RandomNumberGenerator::new();
    for room in main_map.rooms.iter().skip(player_spawn_room + 1) {
        let types = ['g', 'o'];
        let names = ["grim gram", "orca", "orgrimmar", "goob", "gremlin", "osha"];
        let roll_type = rng.random_slice_entry(&types);
        let roll_name = rng.random_slice_entry(&names);
        let mut chosen_type = types[0];
        let mut chosen_name = names[0];
        match roll_type {
            Some(t) => {
                chosen_type = *t;
            }
            _ => {}
        }
        match roll_name {
            Some(n) => {
                chosen_name = *n;
            }
            _ => {}
        }

        gs.ecs
            .create_entity()
            .with(Position {
                x: room.center().0,
                y: room.center().1,
            })
            .with(Renderable {
                glyph: rltk::to_cp437(chosen_type),
                fg: RGB::named(rltk::RED),
                bg: RGB::named(rltk::BLACK),
            })
            .with(Viewshed {
                dirty: true,
                range: 8,
                visible_tiles: Vec::new(),
            })
            .with(Monster {})
            .with(Name {
                name: chosen_name.to_string(),
            })
            .build();
    }

    // register the map and move it into ecs
    gs.ecs.insert(main_map);
    // register the player's position with ecs
    gs.ecs.insert(Point::new(map_center_x, map_center_y));
    rltk::main_loop(context, gs)
}
