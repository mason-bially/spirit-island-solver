// This file contains copyrighted assets owned by Greater Than Games.

use crate::base::{
    GameState, StepFailure, SpiritDescription,
    PowerCardDescription,
    PowerCardKind, PowerSpeed, PowerTargetFilter, PowerTarget, Element, ElementMap,
    LandKind, PieceKind, InvaderKind,
    effect::*, decision::*,
};

fn card_accelerated_rot (game: &mut GameState) -> Result<(), StepFailure> {
    game.do_effect(NotImplementedEffect { what: "Accelerated Rot" })
}

pub fn make_major_power_cards() -> Vec<PowerCardDescription> {
    vec![
        PowerCardDescription {
            name: "Accelerated Rot",
            kind: PowerCardKind::Minor,
            elements: ElementMap::from_slice(&[Element::Sun, Element::Water, Element::Plant]),
            cost: 4, speed: PowerSpeed::Slow, range: Some(2),
            target_filter: PowerTargetFilter::Land(|l| l.desc.kind == LandKind::Jungle || l.desc.kind == LandKind::Wetlands),

            effect: card_accelerated_rot
        },
    ]
}
