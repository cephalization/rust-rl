use rltk::{GameState, Rltk, VirtualKeyCode, RGB};
use specs::prelude::*;
use specs_derive::Component;
use std::cmp::{max, min};

mod map;
use map::{draw_map, new_map_test, xy_idx, TileType};

// CHARACTERS
#[derive(Component)]
struct Position {
    x: i32,
    y: i32,
}

#[derive(Component)]
struct Renderable {
    glyph: rltk::FontCharType,
    fg: RGB,
    bg: RGB,
}

#[derive(Component, Debug)]
struct Player {}

fn try_move_player(delta_x: i32, delta_y: i32, ecs: &mut World) {
    let mut positions = ecs.write_storage::<Position>();
    let mut players = ecs.write_storage::<Player>();
    let map = ecs.fetch::<Vec<TileType>>();

    for (_player, pos) in (&mut players, &mut positions).join() {
        let next_x = pos.x + delta_x;
        let next_y = pos.y + delta_y;
        if map[xy_idx(next_x, next_y)] != TileType::Wall {
            // only move the player if the next set of coords is not a wall tile
            pos.x = min(79, max(0, next_x));
            pos.y = min(49, max(0, next_y));
        }
    }
}

fn player_input(gs: &mut State, ctx: &mut Rltk) {
    match ctx.key {
        None => {}
        Some(key) => match key {
            VirtualKeyCode::Left => try_move_player(-1, 0, &mut gs.ecs),
            VirtualKeyCode::Right => try_move_player(1, 0, &mut gs.ecs),
            VirtualKeyCode::Up => try_move_player(0, -1, &mut gs.ecs),
            VirtualKeyCode::Down => try_move_player(0, 1, &mut gs.ecs),
            _ => {}
        },
    }
}

// STATE
struct State {
    ecs: World,
}
impl GameState for State {
    fn tick(&mut self, ctx: &mut Rltk) {
        ctx.cls();

        player_input(self, ctx);
        self.run_systems();

        let positions = self.ecs.read_storage::<Position>();
        let renderables = self.ecs.read_storage::<Renderable>();

        let map = self.ecs.fetch::<Vec<TileType>>();
        draw_map(&map, ctx);
        for (pos, render) in (&positions, &renderables).join() {
            ctx.set(pos.x, pos.y, render.fg, render.bg, render.glyph);
        }
    }
}
impl State {
    fn run_systems(&mut self) {
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
    // yellow ampersand
    gs.ecs
        .create_entity()
        .with(Position { x: 40, y: 30 })
        .with(Renderable {
            glyph: rltk::to_cp437('@'),
            fg: RGB::named(rltk::YELLOW),
            bg: RGB::named(rltk::BLACK),
        })
        .with(Player {})
        .build();

    gs.ecs.insert(new_map_test());

    rltk::main_loop(context, gs)
}
