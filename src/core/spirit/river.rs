// This file contains copyrighted assets owned by Greater Than Games.

use crate::base::{
    GameState, StepFailure, SpiritDescription,
    LandKind, PowerCardKind, PowerSpeed,
    PowerCardDescription,
    effect::*
};


pub struct SpiritDescriptionRiver {

}

impl SpiritDescription for SpiritDescriptionRiver {
    fn name(&self) -> &'static str { "River Surges in Sunlight" }
    fn all_names(&self) -> &'static [&'static str] { &["River Surges in Sunlight", "river", "rss", "rsis"] }

    fn get_power_cards(&self) -> Vec<PowerCardDescription> {
        vec![
            PowerCardDescription {
                name: "Boon of Vigor",
                kind: PowerCardKind::Spirit,
                speed: PowerSpeed::Fast,

                effect: Box::new(NotImplementedEffect { what: "Boon of Vigor" }),
            },
            PowerCardDescription {
                name: "River's Bounty",
                kind: PowerCardKind::Spirit,
                speed: PowerSpeed::Fast,

                effect: Box::new(NotImplementedEffect { what: "River's Bounty" }),
            },
            PowerCardDescription {
                name: "Wash Away",
                kind: PowerCardKind::Spirit,
                speed: PowerSpeed::Fast,

                effect: Box::new(NotImplementedEffect { what: "Wash Away" }),
            },
            PowerCardDescription {
                name: "Flash Floods",
                kind: PowerCardKind::Spirit,
                speed: PowerSpeed::Fast,

                effect: Box::new(NotImplementedEffect { what: "Boon of Vigor" }),
            },
        ]
    }

    fn do_setup(&self, game: &mut GameState, si: usize) -> Result<(), StepFailure> {
        // River puts 1 in the highest wetland
        let land_index = game.desc.table.boards[si]
            .lands.iter()
            .filter(|l| l.kind == LandKind::Wetlands)
            // boards are sorted lowest to highest by default
            .last().unwrap()
            .index_on_table;
        game.do_effect(AddPresenceEffect{land_index, spirit: si as u8, count: 1})?;

        Ok(())
    }
}

impl SpiritDescriptionRiver {
    pub fn new() -> SpiritDescriptionRiver {
        SpiritDescriptionRiver {

        }
    }
}
