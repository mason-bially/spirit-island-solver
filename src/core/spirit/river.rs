// This file contains copyrighted assets owned by Greater Than Games.

use crate::base::{
    GameState, StepFailure, SpiritDescription,
    PowerCardDescription,
    PowerCardKind, PowerSpeed, PowerTargetFilter, PowerTarget, Element, ElementMap,
    LandKind, PieceKind, InvaderKind,
    effect::*, decision::*,
};


pub struct SpiritDescriptionRiver {

}

fn card_boon_of_vigor (game: &GameState) -> Result<Box<dyn Effect>, StepFailure> {
    let usage = game.get_power_usage()?;
    if let PowerTarget::Spirit(dst_spirit_index) = usage.target {
        let energy 
            = if dst_spirit_index == usage.using_spirit_index {
                1
            } else {
                let spirit = game.get_spirit(dst_spirit_index)?;
                spirit.deck.pending.len() as u8
            };

        Ok(Box::new(GenerateEnergyEffect{spirit_index: dst_spirit_index, energy}))
    } else {
        Err(StepFailure::RulesViolation("Power must target a spirit.".to_string()))
    }
}

fn card_flash_floods (game: &GameState) -> Result<Box<dyn Effect>, StepFailure> {
    let usage = game.get_power_usage()?;
    if let PowerTarget::Land(land_index) = usage.target {
        let land = game.get_land_desc(land_index)?;
        
        let mut damage = 1;
        
        if land.is_coastal {
            damage += 1;
        }

        Ok(Box::new(DoDamageToInvadersDecision{land_index: land_index, damage}))
    } else {
        Err(StepFailure::RulesViolation("Power must target a land.".to_string()))
    }
}

fn card_rivers_bounty (game: &GameState) -> Result<Box<dyn Effect>, StepFailure> {
    let usage = game.get_power_usage()?;
    if let PowerTarget::Land(land_index) = usage.target {
        Ok(Box::new(SequencedEffect{sequence: vec![
            SubEffect::Built(Box::new(GatherDecision{land_index: land_index, count: 3, may: true,
                kinds: vec![PieceKind::Invader(InvaderKind::Explorer), PieceKind::Invader(InvaderKind::Town)]})),
            SubEffect::ConditionalBuild(|game| {
                let usage = game.get_power_usage()?;
                if let PowerTarget::Land(land_index) = usage.target {
                    let land = game.get_land(land_index)?;
                    if land.dahan.len() >= 2 {
                        Ok(Some(Box::new(SequencedEffect{sequence: vec![
                            SubEffect::Built(Box::new(AddDahanEffect{land_index: land_index, count: 1})),
                            SubEffect::Built(Box::new(GenerateEnergyEffect{spirit_index: usage.using_spirit_index, energy: 1})),
                        ]})))
                    } else {
                        Ok(None)
                    }
                } else {
                    Err(StepFailure::RulesViolation("Power must target a land.".to_string()))
                }}),
        ]}))
    } else {
        Err(StepFailure::RulesViolation("Power must target a land.".to_string()))
    }
}

fn card_wash_away (game: &GameState) -> Result<Box<dyn Effect>, StepFailure> {
    let usage = game.get_power_usage()?;
    if let PowerTarget::Land(land_index) = usage.target {
        Ok(Box::new(PushDecision{land_index: land_index, count: 3, may: true,
            kinds: vec![PieceKind::Invader(InvaderKind::Explorer), PieceKind::Invader(InvaderKind::Town)]}))
    } else {
        Err(StepFailure::RulesViolation("Power must target a land.".to_string()))
    }
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
                name: "Flash Floods",
                kind: PowerCardKind::Spirit(spirit_index),
                elements: ElementMap::from_slice(&[Element::Sun, Element::Water]),
                cost: 1, speed: PowerSpeed::Fast, range: Some(1),
                target_filter: PowerTargetFilter::Land(|_| true),

                effect_builder: card_flash_floods,
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
