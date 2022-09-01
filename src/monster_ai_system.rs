use super::{Map, Monster, Position, Viewshed};
use rltk::{console, Point, RandomNumberGenerator};
use specs::prelude::*;

pub struct MonsterAI {}

impl<'a> System<'a> for MonsterAI {
    type SystemData = (
        ReadStorage<'a, Viewshed>,
        ReadStorage<'a, Position>,
        ReadStorage<'a, Monster>,
        ReadExpect<'a, Point>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (viewshed, pos, monster, player_point) = data;
        let idle_text: Vec<&str> = vec![
            "Monster considers their own existence",
            "Monster shouts at you",
            "Monster scratches itself",
        ];
        let mut rng = RandomNumberGenerator::new();

        for (viewshed, pos, _monster) in (&viewshed, &pos, &monster).join() {
            if viewshed.visible_tiles.contains(&player_point) {
                let text = rng.random_slice_entry(&idle_text);
                match text {
                    None => {}
                    Some(s) => console::log(s),
                }
            }
        }
    }
}
