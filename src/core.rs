// This file contains copyrighted assets owned by Greater Than Games.

use crate::base::{ContentPack, SpiritDescription, BoardDescription, FearCardDescription};

mod spirit;
mod board;
mod fear;

pub use spirit::{SpiritDescriptionRiver};
use board::{make_board_a};
use fear::{make_fear_cards};

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

    fn get_fear_cards(&self) -> Vec<Box<dyn FearCardDescription>> {
        make_fear_cards()
    }
}

impl CoreContent {
    pub fn new() -> CoreContent {
        CoreContent {

        }
    }
}
