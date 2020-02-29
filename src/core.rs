// This file contains copyrighted assets owned by Greater Than Games.

use crate::base::{
    ContentPack,
    SpiritDescription, BoardDescription,
    FearCardDescription, PowerCardDescription
};

mod spirit;
mod board;
mod fear;
mod power;

pub use spirit::{SpiritDescriptionRiver};
use board::{make_board_a};
use fear::{make_fear_cards};
use power::{make_minor_power_cards, make_major_power_cards};

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

    fn get_fear_cards(&self) -> Vec<FearCardDescription> {
        make_fear_cards()
    }

    fn get_power_cards(&self) -> Vec<PowerCardDescription> {
        let mut res = Vec::new();
        res.extend(make_minor_power_cards());
        res.extend(make_major_power_cards());

        res
    }
}

impl CoreContent {
    pub fn new() -> CoreContent {
        CoreContent {

        }
    }
}
