// This file contains copyrighted assets owned by Greater Than Games.

use crate::base::{
    GameState, StepFailure, SpiritDescription,
    LandKind, PowerCardKind, PowerSpeed, PowerTargetFilter, PowerTarget, Element, ElementMap,
    PowerCardDescription,
    effect::*
};


pub struct SpiritDescriptionRiver {

}

fn card_boon_of_vigor (target: &PowerTarget) -> Box<dyn Effect> {
    Box::new(NotImplementedEffect { what: "Boon of Vigor" })
}

fn card_rivers_bounty (target: &PowerTarget) -> Box<dyn Effect> {
    Box::new(NotImplementedEffect { what: "River's Bounty" })
}

fn card_wash_away (target: &PowerTarget) -> Box<dyn Effect> {
    Box::new(NotImplementedEffect { what: "Wash Away" })
}

fn card_flash_floods (target: &PowerTarget) -> Box<dyn Effect> {
    Box::new(NotImplementedEffect { what: "Flash Floods" })
}

impl SpiritDescription for SpiritDescriptionRiver {
    fn name(&self) -> &'static str { "River Surges in Sunlight" }
    fn all_names(&self) -> &'static [&'static str] { &["River Surges in Sunlight", "river", "rss", "rsis"] }

    fn get_power_cards(&self, spirit_index: u8) -> Vec<PowerCardDescription> {
        vec![
            PowerCardDescription {
                name: "Boon of Vigor",
                kind: PowerCardKind::Spirit(spirit_index),
                elements: ElementMap::from_slice(&[Element::Sun, Element::Water, Element::Plant]),
                cost: 0, speed: PowerSpeed::Fast, range: None,
                target_filter: PowerTargetFilter::Spirit(|_| true),

                effect_builder: card_boon_of_vigor
            },
            PowerCardDescription {
                name: "River's Bounty",
                kind: PowerCardKind::Spirit(spirit_index),
                elements: ElementMap::from_slice(&[Element::Sun, Element::Water, Element::Animal]),
                cost: 0, speed: PowerSpeed::Slow, range: Some(0),
                target_filter: PowerTargetFilter::Land(|_| true),

                effect_builder: card_rivers_bounty,
            },
            PowerCardDescription {
                name: "Wash Away",
                kind: PowerCardKind::Spirit(spirit_index),
                elements: ElementMap::from_slice(&[Element::Water, Element::Earth]),
                cost: 1, speed: PowerSpeed::Slow, range: Some(1),
                target_filter: PowerTargetFilter::Land(|_| true),

                effect_builder: card_wash_away,
            },
            PowerCardDescription {
                name: "Flash Floods",
                kind: PowerCardKind::Spirit(spirit_index),
                elements: ElementMap::from_slice(&[Element::Sun, Element::Water]),
                cost: 1, speed: PowerSpeed::Fast, range: Some(1),
                target_filter: PowerTargetFilter::Land(|_| true),

                effect_builder: card_flash_floods,
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
