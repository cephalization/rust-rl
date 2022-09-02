use super::{Map, Monster, Name, Position, Viewshed};
use rltk::{console, Point, RandomNumberGenerator};
use specs::prelude::*;

pub struct MonsterAI {}

impl<'a> System<'a> for MonsterAI {
    type SystemData = (
        WriteStorage<'a, Viewshed>,
        WriteStorage<'a, Position>,
        ReadStorage<'a, Monster>,
        ReadExpect<'a, Point>,
        ReadStorage<'a, Name>,
        WriteExpect<'a, Map>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut s_viewshed, mut s_pos, s_monster, s_player_point, s_name, mut s_map) = data;
        let idle_text: Vec<&str> = vec![
            "considers their own existence",
            "shouts at you",
            "scratches itself",
            "insults you",
        ];
        let mut rng = RandomNumberGenerator::new();

        let mut index = 0;
        for (mut viewshed, mut pos, _monster, name) in
            (&mut s_viewshed, &mut s_pos, &s_monster, &s_name).join()
        {
            if viewshed.visible_tiles.contains(&s_player_point) {
                if let Some(shout) = rng.random_slice_entry(&idle_text) {
                    console::log(format!("({}) {} {}", index, name.name, *shout));
                }
                let path = rltk::a_star_search(
                    s_map.xy_idx(pos.x, pos.y),
                    s_map.xy_idx(s_player_point.x, s_player_point.y),
                    &mut *s_map,
                );
                if path.success && path.steps.len() > 1 {
                    let (x, y) = s_map.idx_xy(path.steps[1]);
                    pos.x = x;
                    pos.y = y;
                    viewshed.dirty = true;
                }
            }
            index = index + 1
        }
    }
}
