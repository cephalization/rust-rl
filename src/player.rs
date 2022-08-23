use super::{xy_idx, Position, State, TileType};
use rltk::{Rltk, VirtualKeyCode};
use specs::prelude::*;
use specs_derive::Component;
use std::cmp::{max, min};

#[derive(Component, Debug)]
pub struct Player {}

pub fn try_move_player(delta_x: i32, delta_y: i32, ecs: &mut World) {
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

pub fn player_input(gs: &mut State, ctx: &mut Rltk) {
    match ctx.key {
        None => {}
        Some(key) => match key {
            VirtualKeyCode::H | VirtualKeyCode::Numpad4 | VirtualKeyCode::Left => {
                try_move_player(-1, 0, &mut gs.ecs)
            }
            VirtualKeyCode::L | VirtualKeyCode::Numpad6 | VirtualKeyCode::Right => {
                try_move_player(1, 0, &mut gs.ecs)
            }
            VirtualKeyCode::K | VirtualKeyCode::Numpad8 | VirtualKeyCode::Up => {
                try_move_player(0, -1, &mut gs.ecs)
            }
            VirtualKeyCode::Numpad2 | VirtualKeyCode::J | VirtualKeyCode::Down => {
                try_move_player(0, 1, &mut gs.ecs)
            }
            _ => {}
        },
    }
}
