// This file contains copyrighted assets owned by Greater Than Games.

use crate::base::{
    GameState, StepFailure, SpiritDescription,
    PowerCardDescription,
    PowerCardKind, PowerSpeed, PowerTargetFilter, PowerTarget, Element, ElementMap,
    LandKind, PieceKind, InvaderKind,
    effect::*, decision::*,
};


fn card_call_of_the_dahan_ways (game: &mut GameState) -> Result<(), StepFailure> {
    game.do_effect(NotImplementedEffect { what: "Call of The Dahan Ways" })
}

fn card_call_to_bloodshed (game: &mut GameState) -> Result<(), StepFailure> {
    game.do_effect(NotImplementedEffect { what: "Call to Bloodshed" })
}

fn card_call_to_isolation (game: &mut GameState) -> Result<(), StepFailure> {
    game.do_effect(NotImplementedEffect { what: "Call to Isolation" })
}

fn card_call_to_migrate (game: &mut GameState) -> Result<(), StepFailure> {
    game.do_effect(NotImplementedEffect { what: "Call to Migrate" })
}

fn card_call_to_tend (game: &mut GameState) -> Result<(), StepFailure> {
    game.do_effect(NotImplementedEffect { what: "Call to Tend" })
}

fn card_dark_and_tangled_woods (game: &mut GameState) -> Result<(), StepFailure> {
    let usage = game.get_power_usage()?;
    if let PowerTarget::Land(land_index) = usage.target {
        // 2 fear
        game.do_effect(GenerateFearEffect{fear: 2, land_index: Some(land_index)})?;

        // if land is m|s def 3
        let land = game.get_land_desc(land_index)?;
        if land.kind == LandKind::Mountain || land.kind == LandKind::Jungle {
            game.do_effect(PersistDefenseEffect{land_index, defense: 3})?;
        }

        Ok(())
    } else {
        Err(StepFailure::RulesViolation("Power must target a land.".to_string()))
    }
}

fn card_delusions_of_danger (game: &mut GameState) -> Result<(), StepFailure> {
    game.do_effect(NotImplementedEffect { what: "Delusions of Danger" })
}

fn card_devouring_ants (game: &mut GameState) -> Result<(), StepFailure> {
    game.do_effect(NotImplementedEffect { what: "Devouring Ants" })
}

fn card_drift_down_into_slumber (game: &mut GameState) -> Result<(), StepFailure> {
    let usage = game.get_power_usage()?;
    if let PowerTarget::Land(land_index) = usage.target {
        let land = game.get_land_desc(land_index)?;
        if land.kind == LandKind::Jungle || land.kind == LandKind::Sands {
            // instead def 4
            game.do_effect(PersistDefenseEffect{land_index, defense: 4})
        } else {
            // def 1
            game.do_effect(PersistDefenseEffect{land_index, defense: 1})
        }
    } else {
        Err(StepFailure::RulesViolation("Power must target a land.".to_string()))
    }
}

pub fn make_minor_power_cards() -> Vec<PowerCardDescription> {
    vec![
        PowerCardDescription {
            name: "Call of The Dahan Ways",
            kind: PowerCardKind::Minor,
            elements: ElementMap::from_slice(&[Element::Moon, Element::Water, Element::Animal]),
            cost: 1, speed: PowerSpeed::Slow, range: Some(1),
            target_filter: PowerTargetFilter::Land(|l| l.dahan.len() != 0),

            effect: card_call_of_the_dahan_ways
        },
        PowerCardDescription {
            name: "Call to Bloodshed",
            kind: PowerCardKind::Minor,
            elements: ElementMap::from_slice(&[Element::Sun, Element::Fire, Element::Animal]),
            cost: 1, speed: PowerSpeed::Slow, range: Some(1),
            target_filter: PowerTargetFilter::Land(|l| l.dahan.len() != 0),

            effect: card_call_to_bloodshed
        },
        PowerCardDescription {
            name: "Call to Isolation",
            kind: PowerCardKind::Minor,
            elements: ElementMap::from_slice(&[Element::Sun, Element::Air, Element::Animal]),
            cost: 0, speed: PowerSpeed::Fast, range: Some(1),
            target_filter: PowerTargetFilter::Land(|l| l.dahan.len() != 0),

            effect: card_call_to_isolation
        },
        PowerCardDescription {
            name: "Call to Migrate",
            kind: PowerCardKind::Minor,
            elements: ElementMap::from_slice(&[Element::Fire, Element::Air, Element::Animal]),
            cost: 1, speed: PowerSpeed::Slow, range: Some(1),
            target_filter: PowerTargetFilter::Land(|_| true),

            effect: card_call_to_migrate
        },
        PowerCardDescription {
            name: "Call to Tend",
            kind: PowerCardKind::Minor,
            elements: ElementMap::from_slice(&[Element::Water, Element::Plant, Element::Animal]),
            cost: 1, speed: PowerSpeed::Slow, range: Some(1),
            target_filter: PowerTargetFilter::Land(|l| l.dahan.len() != 0),

            effect: card_call_to_tend
        },
        PowerCardDescription {
            name: "Dark and Tangled Woods",
            kind: PowerCardKind::Minor,
            elements: ElementMap::from_slice(&[Element::Moon, Element::Earth, Element::Plant]),
            cost: 1, speed: PowerSpeed::Fast, range: Some(1),
            target_filter: PowerTargetFilter::Land(|_| true),

            effect: card_dark_and_tangled_woods
        },
        PowerCardDescription {
            name: "Delusions of Danger",
            kind: PowerCardKind::Minor,
            elements: ElementMap::from_slice(&[Element::Sun, Element::Moon, Element::Air]),
            cost: 1, speed: PowerSpeed::Fast, range: Some(1),
            target_filter: PowerTargetFilter::Land(|_| true),

            effect: card_delusions_of_danger
        },
        PowerCardDescription {
            name: "Devouring Ants",
            kind: PowerCardKind::Minor,
            elements: ElementMap::from_slice(&[Element::Sun, Element::Earth, Element::Animal]),
            cost: 1, speed: PowerSpeed::Slow, range: Some(1),
            target_filter: PowerTargetFilter::Land(|_| true),

            effect: card_devouring_ants,
        },
        PowerCardDescription {
            name: "Dirft Down into Slumber",
            kind: PowerCardKind::Minor,
            elements: ElementMap::from_slice(&[Element::Air, Element::Earth, Element::Plant]),
            cost: 0, speed: PowerSpeed::Fast, range: Some(2),
            target_filter: PowerTargetFilter::Land(|_| true),

            effect: card_drift_down_into_slumber,
        },
    ]
}
