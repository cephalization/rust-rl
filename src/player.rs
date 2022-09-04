use super::{Map, Position, RunState, State, Viewshed};
use rltk::{Point, Rltk, VirtualKeyCode};
use specs::prelude::*;
use specs_derive::Component;
use std::cmp::{max, min};

#[derive(Component, Debug)]
pub struct Player {}

pub fn try_move_player(delta_x: i32, delta_y: i32, ecs: &mut World) {
    let mut positions = ecs.write_storage::<Position>();
    let mut players = ecs.write_storage::<Player>();
    let mut viewshed = ecs.write_storage::<Viewshed>();
    let mut player_position = ecs.write_resource::<Point>();
    let map = ecs.fetch::<Map>();

    for (_player, pos, vs) in (&mut players, &mut positions, &mut viewshed).join() {
        let next_x = pos.x + delta_x;
        let next_y = pos.y + delta_y;
        let next_idx = map.xy_idx(next_x, next_y);
        if !map.blocked[next_idx] {
            // only move the player if the next set of coords is not a wall tile
            pos.x = min(79, max(0, next_x));
            pos.y = min(49, max(0, next_y));
            // update global player position in the ecs
            player_position.x = pos.x;
            player_position.y = pos.y;
            // now that the player has moved, set their viewshed to dirty to recaculate FoV
            vs.dirty = true;
        }
    }
}

pub fn player_input(gs: &mut State, ctx: &mut Rltk) -> RunState {
    match ctx.key {
        None => return RunState::Paused,
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

            _ => return RunState::Paused,
        },
    }

    RunState::Running
}
