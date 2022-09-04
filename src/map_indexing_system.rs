use super::{BlocksTiles, Map, Position};
use specs::prelude::*;

pub struct MapIndexingSystem {}

impl<'a> System<'a> for MapIndexingSystem {
    type SystemData = (
        WriteExpect<'a, Map>,
        ReadStorage<'a, Position>,
        ReadStorage<'a, BlocksTiles>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut s_map, s_position, s_blocks_tiles) = data;

        s_map.populate_blocked();
        for (position, _blocks) in (&s_position, &s_blocks_tiles).join() {
            let idx = s_map.xy_idx(position.x, position.y);
            s_map.blocked[idx] = true;
        }
    }
}
