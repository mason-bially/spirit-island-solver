// This file contains copyrighted assets owned by Greater Than Games.

use crate::base::{ContentPack, SpiritDescription, BoardDescription};

mod spirit;
mod board;

pub use spirit::{SpiritDescriptionRiver};
use board::{make_board_a};

pub struct CoreContent {

}

impl ContentPack for CoreContent {
    fn get_spirits(&self) -> Vec<Box<dyn SpiritDescription>> {
        vec![
            Box::new(SpiritDescriptionRiver::new()),
        ]
    }

    fn get_boards(&self) -> Vec<BoardDescription> {
        vec![
            make_board_a(),
        ]
    }
}

impl CoreContent {
    pub fn new() -> CoreContent {
        CoreContent {

        }
    }
}
