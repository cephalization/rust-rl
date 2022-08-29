use super::{Map, Player, Position, Viewshed};
use rltk::{field_of_view, Point};
use specs::prelude::*;

pub struct VisibilitySystem {}

impl<'a> System<'a> for VisibilitySystem {
    type SystemData = (
        WriteExpect<'a, Map>,
        Entities<'a>,
        WriteStorage<'a, Viewshed>,
        WriteStorage<'a, Position>,
        ReadStorage<'a, Player>,
    );
    fn run(&mut self, data: Self::SystemData) {
        let (mut map, ents, mut viewshed, pos, player) = data;
        for (viewshed, pos, ent) in (&mut viewshed, &pos, &ents).join() {
            viewshed.visible_tiles.clear();
            viewshed.visible_tiles = field_of_view(Point::new(pos.x, pos.y), viewshed.range, &*map);
            viewshed
                .visible_tiles
                .retain(|p| p.x < map.width && p.y < map.height && p.y >= 0 && p.x >= 0);
            let _p: Option<&Player> = player.get(ent);
            if let Some(_p) = _p {
                for pt in viewshed.visible_tiles.iter() {
                    let pt_to_idx = map.xy_idx(pt.x, pt.y);
                    map.seen_tiles[pt_to_idx] = true
                }
            }
        }
    }
}
