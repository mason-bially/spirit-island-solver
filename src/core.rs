use crate::base::{ContentPack, SpiritDescription};

mod spirit;

pub use spirit::{SpiritDescriptionRiver};

pub struct CoreContent {

}

impl ContentPack for CoreContent {
    fn get_spirits(&self) -> Vec<Box<dyn SpiritDescription>> {
        vec![
            Box::new(SpiritDescriptionRiver::new()),
        ]
    }
}

impl CoreContent {
    pub fn new() -> CoreContent {
        CoreContent {

        }
    }
}
