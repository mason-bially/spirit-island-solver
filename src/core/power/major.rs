// This file contains copyrighted assets owned by Greater Than Games.

use crate::base::{
    GameState, StepFailure,
    PowerCardDescription,
    PowerCardKind, PowerSpeed, PowerTargetFilter, PowerTarget, Element, ElementMap,
    LandKind, PieceKind, InvaderKind,
    effect::*, decision::*,
};

fn card_accelerated_rot (game: &mut GameState) -> Result<(), StepFailure> {
    game.do_effect(NotImplementedEffect { what: "Accelerated Rot" })
}

// lss progression 3
fn card_powerstorm (game: &mut GameState) -> Result<(), StepFailure> {
    game.do_effect(NotImplementedEffect { what: "Powerstorm" })
}

// lss progression 5
fn card_pillar_of_living_flame (game: &mut GameState) -> Result<(), StepFailure> {
    game.do_effect(NotImplementedEffect { what: "Pillar of Living Flame" })
}

pub fn make_major_power_cards() -> Vec<PowerCardDescription> {
    vec![
        PowerCardDescription {
            name: "Accelerated Rot",
            kind: PowerCardKind::Minor,
            elements: ElementMap::from_slice(&[Element::Sun, Element::Water, Element::Plant]),
            cost: 4, speed: PowerSpeed::Slow,
            target_filter: PowerTargetFilter::Land{
                range: 2,
                src: |_| true,
                dst: |l| l.desc.kind == LandKind::Jungle || l.desc.kind == LandKind::Wetlands
            },

            effect: card_accelerated_rot
        },
    ]
}
