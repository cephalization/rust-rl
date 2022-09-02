use super::{Map, Monster, Name, Position, Viewshed};
use rltk::{console, Point, RandomNumberGenerator};
use specs::prelude::*;

pub struct MonsterAI {}

impl<'a> System<'a> for MonsterAI {
    type SystemData = (
        ReadStorage<'a, Viewshed>,
        ReadStorage<'a, Position>,
        ReadStorage<'a, Monster>,
        ReadExpect<'a, Point>,
        ReadStorage<'a, Name>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (viewshed, pos, monster, player_point, name) = data;
        let idle_text: Vec<&str> = vec![
            "{} considers their own existence",
            "{} shouts at you",
            "{} scratches itself",
        ];
        let mut rng = RandomNumberGenerator::new();

        for (viewshed, pos, _monster, name) in (&viewshed, &pos, &monster, &name).join() {
            if viewshed.visible_tiles.contains(&player_point) {
                let text = rng.random_slice_entry(&idle_text);
                match text {
                    None => {}
                    Some(s) => console::log(s.to_string().replace("{}", &name.name)),
                }
            }
        }
    }
}
