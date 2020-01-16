use crate::base::{ContentPack, Spirit};

mod spirit;

pub use spirit::{SpiritRiver};

pub struct CoreContent {

}

impl ContentPack for CoreContent {
    fn get_spirits(&self) -> Vec<Box<dyn Spirit>> {
        vec![
            Box::new(SpiritRiver::new()),
        ]
    }
}

impl CoreContent {
    pub fn new() -> CoreContent {
        CoreContent {

        }
    }
}
