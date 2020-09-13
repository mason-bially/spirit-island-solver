// This file contains copyrighted assets owned by Greater Than Games.

use std::{
    cmp::{min}
};

use crate::base::{
    GameState, StepFailure, SpiritDescription, PresenceState,
    PowerCardDescription,
    PowerCardKind, PowerSpeed, PowerTargetFilter, PowerTarget, Element, ElementMap,
    LandKind, PieceKind, InvaderKind,
    effect::*, decision::*,
};


pub struct SpiritDescriptionLightning {

}

fn card_harbringers_of_the_lightning (game: &mut GameState) -> Result<(), StepFailure> {
    Ok(())
}

fn card_lightnings_boon (game: &mut GameState) -> Result<(), StepFailure> {
    Ok(())
}

fn card_raging_storm (game: &mut GameState) -> Result<(), StepFailure> {
    Ok(())
}

fn card_shatter_homesteads (game: &mut GameState) -> Result<(), StepFailure> {
    Ok(())
}

const _TOP_TRACK_START : u8 = 2;
const _BOT_TRACK_START : u8 = 9;

impl SpiritDescription for SpiritDescriptionLightning {
    fn name(&self) -> &'static str { "Lightning's Swift Strike" }
    fn all_names(&self) -> &'static [&'static str] { &["Lightning's Swift Strike", "lightning", "lss"] }

    fn get_power_cards(&self, spirit_index: u8) -> Vec<PowerCardDescription> {
        vec![
            PowerCardDescription {
                name: "Harbringer's of the Lightning",
                kind: PowerCardKind::Spirit(spirit_index),
                elements: ElementMap::from_slice(&[Element::Fire, Element::Air]),
                cost: 0, speed: PowerSpeed::Slow,
                target_filter: PowerTargetFilter::Land{range: 1, src: |_| true, dst: |_| true},

                effect: card_harbringers_of_the_lightning
            },
            PowerCardDescription {
                name: "Lightning's Boon",
                kind: PowerCardKind::Spirit(spirit_index),
                elements: ElementMap::from_slice(&[Element::Fire, Element::Air]),
                cost: 1, speed: PowerSpeed::Fast,
                target_filter: PowerTargetFilter::Spirit(|_| true),

                effect: card_lightnings_boon,
            },
            PowerCardDescription {
                name: "Raging Storm",
                kind: PowerCardKind::Spirit(spirit_index),
                elements: ElementMap::from_slice(&[Element::Fire, Element::Air, Element::Water]),
                cost: 3, speed: PowerSpeed::Slow,
                target_filter: PowerTargetFilter::Land{range: 1, src: |_| true, dst: |_| true},

                effect: card_raging_storm,
            },
            PowerCardDescription {
                name: "Shatter Homesteads",
                kind: PowerCardKind::Spirit(spirit_index),
                elements: ElementMap::from_slice(&[Element::Fire, Element::Air]),
                cost: 1, speed: PowerSpeed::Slow,
                target_filter: PowerTargetFilter::Land{range: 2, src: |_| true /* sacred site */, dst: |_| true},

                effect: card_shatter_homesteads,
            },
        ]
    }
    fn get_power_progression(&self) -> Vec<&'static str> {
        vec![
            "Delusions of Danger",
            "Call to Bloodshed",
            "Powerstorm",
            "Purfying Flame",
            "Pillar of Living Flame",
            "Entrancing Apparitions",
            "Call to Isolation",
        ]
    }

    fn do_setup(&self, game: &mut GameState, si: usize) -> Result<(), StepFailure> {
        // Lightning puts 2 in the highest sands
        let land_index = game.desc.table.boards[si]
            .lands.iter()
            .filter(|l| l.kind == LandKind::Sands)
            // boards are sorted lowest to highest by default
            .last().unwrap()
            .index_on_table;

        game.do_effect(AddPresenceEffect{land_index, spirit_index: si as u8, presence_index: 0})?;
        game.do_effect(AddPresenceEffect{land_index, spirit_index: si as u8, presence_index: 1})?;

        let spirit = game.get_spirit_mut(si as u8)?;
        
        for i in 2..13 {
            spirit.presence[i] = PresenceState::OnTrack(i as u8);
        }

        Ok(())
    }

    fn may_place_presence(&self, state: &[PresenceState; 13], presence_index: usize) -> Result<bool, StepFailure> {
        match state[presence_index] {
            PresenceState::OnTrack(track_loc) => {
                if track_loc < _BOT_TRACK_START {
                    // Top Track
                    if track_loc == _TOP_TRACK_START {
                        Ok(true)
                    } else {
                        Ok(state[(track_loc - 1) as usize] != PresenceState::OnTrack(track_loc - 1))
                    }
                } else {
                    // Bottom Track
                    if track_loc == _BOT_TRACK_START {
                        Ok(true)
                    } else {
                        Ok(state[(track_loc - 1) as usize] != PresenceState::OnTrack(track_loc - 1))
                    }
                }
            },
            _ => Ok(true)
        }
    }

    fn do_growth(&self, game: &mut GameState, spirit_index: usize) -> Result<(), StepFailure> {
        game.do_effect(ChooseGrowthDecision{
            spirit_index: spirit_index as u8,
            count: 1,
            choices: vec![
                |game, spirit_index| {
                    // Growth A
                    game.do_effect(ReclaimAllEffect{ spirit_index })?;
                    game.do_effect(GainPowerCardDecision{ spirit_index })?;
                    game.do_effect(GenerateEnergyEffect{ spirit_index, energy: 1 })?;

                    Ok(())
                },
                |game, spirit_index| {
                    // Growth B
                    game.do_effect(AddPresenceDecision{ spirit_index, distance: 2 })?;
                    game.do_effect(AddPresenceDecision{ spirit_index, distance: 0 })?;

                    Ok(())
                },
                |game, spirit_index| {
                    // Growth C
                    game.do_effect(AddPresenceDecision{ spirit_index, distance: 1 })?;
                    game.do_effect(GenerateEnergyEffect{ spirit_index, energy: 3 })?;
                    
                    Ok(())
                },
            ]
        })
    }

    fn do_income(&self, game: &mut GameState, spirit_index: usize) -> Result<(), StepFailure> {
        let spirit = game.get_spirit_mut(spirit_index as u8)?;
        
        let mut top_track_min = 15;
        let mut bot_track_min = 15;

        for presence in spirit.presence.iter() {
            match *presence {
                PresenceState::OnTrack(track_loc) => {
                    if track_loc >= _BOT_TRACK_START {
                        bot_track_min = min(bot_track_min, track_loc);
                    } else {
                        top_track_min = min(top_track_min, track_loc);
                    }
                },
                _ => {}
            }
        }

        // The above loop fings the minimum track spot with presence still on it. To find the
        // minimum open spot we should have to subtract 1, to get normative numbering of the tracks
        // (where 0 is the free space and 1 is the first open space) we need to add 1. These cancel
        // out. Make sure to change this code if these assumptions change.
        // TODO: use a struct for all this shit. 

        top_track_min -= _TOP_TRACK_START;
        bot_track_min -= _BOT_TRACK_START;

        let card_plays;
        let energy;

        match top_track_min {
            1 => energy = 1,
            2 | 3 => energy = 2,
            4 => energy = 3,
            5 | 6 => energy = 4,
            7 => energy = 5,
            _ => energy = 1,
        }

        match bot_track_min {
            1 => card_plays = 3,
            2 => card_plays = 4,
            3 => card_plays = 5,
            4 => card_plays = 6,
            _ => card_plays = 2,
        }

        spirit.plays = card_plays; // TODO: effect maybe?
        
        game.do_effect(GenerateEnergyEffect{ spirit_index: spirit_index as u8, energy })?;

        Ok(())
    }
}

impl SpiritDescriptionLightning {
    pub fn new() -> SpiritDescriptionLightning {
        SpiritDescriptionLightning {

        }
    }
}
