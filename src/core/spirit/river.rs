use crate::base::{GameState, SpiritDescription};

pub struct SpiritDescriptionRiver {

}

// All copyrighted game assets (rendered in code form) are owned by Greater Than Games.

impl SpiritDescription for SpiritDescriptionRiver {
    fn name(&self) -> &'static str { "River Surges in Sunlight" }
    fn all_names(&self) -> &'static [&'static str] { &["River Surges in Sunlight", "river", "rss", "rsis"] }

    fn do_reset(&mut self, game: &mut GameState)
    {

    }
    fn do_setup(&mut self, game: &mut GameState)
    {

    }
}

impl SpiritDescriptionRiver {
    pub fn new() -> SpiritDescriptionRiver {
        SpiritDescriptionRiver {

        }
    }
}
