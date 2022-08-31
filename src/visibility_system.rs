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
        // destructure system components
        let (mut map, ents, mut viewshed, pos, player) = data;
        // select all entities that have a position and viewshed components
        for (viewshed, pos, ent) in (&mut viewshed, &pos, &ents).join() {
            // when the viewshed is dirty, update vecs of visible tiles on the viewshed
            // if the viewshed belongs to a player, also update the revealed and visible tiles of the map
            if viewshed.dirty {
                viewshed.visible_tiles.clear();
                // calculate all points around the x,y pos of the current entity for the viewshed range
                viewshed.visible_tiles =
                    field_of_view(Point::new(pos.x, pos.y), viewshed.range, &*map);
                // filter vec down to those points that are within the map boundaries
                viewshed
                    .visible_tiles
                    .retain(|p| p.x < map.width && p.y < map.height && p.y >= 0 && p.x >= 0);

                // if the entity is a player, update the map's visible and revealed tiles
                let _p: Option<&Player> = player.get(ent);
                if let Some(_p) = _p {
                    let length = map.height * map.width;
                    map.visible_tiles = vec![false; length as usize];

                    for pt in viewshed.visible_tiles.iter() {
                        let pt_to_idx = map.xy_idx(pt.x, pt.y);
                        map.revealed_tiles[pt_to_idx] = true;
                        map.visible_tiles[pt_to_idx] = true;
                    }
                }
                // the viewshed is no longer dirty, this system will not run again
                // until something dirties the viewshed
                viewshed.dirty = false;
            }
        }
    }
}
