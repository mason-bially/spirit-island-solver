use crate::base::{GameState, Spirit};

pub struct SpiritRiver {

}

impl Spirit for SpiritRiver {
    fn name(&self) -> &'static str { "River Surges in Sunlight" }
    fn all_names(&self) -> &'static [&'static str] { &["River Surges in Sunlight", "river", "rss", "rsis"] }

    fn do_reset(&mut self, game: &mut GameState)
    {

    }
    fn do_setup(&mut self, game: &mut GameState)
    {

    }
}

impl SpiritRiver {
    pub fn new() -> SpiritRiver {
        SpiritRiver {

        }
    }
}
