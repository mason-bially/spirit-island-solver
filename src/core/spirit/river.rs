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


pub struct SpiritDescriptionRiver {

}

fn card_boon_of_vigor (game: &mut GameState) -> Result<(), StepFailure> {
    let usage = game.get_power_usage()?;
    if let PowerTarget::Spirit(dst_spirit_index) = usage.target {
        let energy 
            = if dst_spirit_index == usage.using_spirit_index {
                1
            } else {
                let spirit = game.get_spirit(dst_spirit_index)?;
                spirit.deck.pending.len() as u8
            };

        game.do_effect(GenerateEnergyEffect{spirit_index: dst_spirit_index, energy})
    } else {
        Err(StepFailure::RulesViolation("Power must target a spirit.".to_string()))
    }
}

fn card_flash_floods (game: &mut GameState) -> Result<(), StepFailure> {
    let usage = game.get_power_usage()?;
    if let PowerTarget::Land(land_index) = usage.target {
        let land = game.get_land_desc(land_index)?;
        
        let mut damage = 1;
        
        if land.is_coastal {
            damage += 1;
        }

        game.do_effect(DoDamageToInvadersDecision{land_index, damage})
    } else {
        Err(StepFailure::RulesViolation("Power must target a land.".to_string()))
    }
}

fn card_rivers_bounty (game: &mut GameState) -> Result<(), StepFailure> {
    let usage = *game.get_power_usage()?;
    let land_index = usage.target_land()?;

    // gather up to 2 dahan
    game.do_effect(GatherDecision{land_index, count: 2, may: true,
        kinds: vec![PieceKind::Dahan]})?;

    // if 2 or more dahan
    if game.get_land(land_index)?.dahan.len() >= 2 {
        game.do_effect(AddDahanEffect{land_index, count: 1})?;
        game.do_effect(GenerateEnergyEffect{spirit_index: usage.using_spirit_index, energy: 1})?;
    }

    Ok(())
}

fn card_wash_away (game: &mut GameState) -> Result<(), StepFailure> {
    let usage = game.get_power_usage()?;
    if let PowerTarget::Land(land_index) = usage.target {
        game.do_effect(PushDecision{land_index, count: 3, may: true,
            kinds: vec![PieceKind::Invader(InvaderKind::Explorer), PieceKind::Invader(InvaderKind::Town)]})
    } else {
        Err(StepFailure::RulesViolation("Power must target a land.".to_string()))
    }
}

const _TOP_TRACK_START : u8 = 1;
const _BOT_TRACK_START : u8 = 7;

impl SpiritDescription for SpiritDescriptionRiver {
    fn name(&self) -> &'static str { "River Surges in Sunlight" }
    fn all_names(&self) -> &'static [&'static str] { &["River Surges in Sunlight", "river", "rss", "rsis"] }

    fn get_power_cards(&self, spirit_index: u8) -> Vec<PowerCardDescription> {
        vec![
            PowerCardDescription {
                name: "Boon of Vigor",
                kind: PowerCardKind::Spirit(spirit_index),
                elements: ElementMap::from_slice(&[Element::Sun, Element::Water, Element::Plant]),
                cost: 0, speed: PowerSpeed::Fast,
                target_filter: PowerTargetFilter::Spirit(|_| true),

                effect: card_boon_of_vigor
            },
            PowerCardDescription {
                name: "Flash Floods",
                kind: PowerCardKind::Spirit(spirit_index),
                elements: ElementMap::from_slice(&[Element::Sun, Element::Water]),
                cost: 1, speed: PowerSpeed::Fast,
                target_filter: PowerTargetFilter::Land{range: 1, src: |_| true, dst: |_| true},

                effect: card_flash_floods,
            },
            PowerCardDescription {
                name: "River's Bounty",
                kind: PowerCardKind::Spirit(spirit_index),
                elements: ElementMap::from_slice(&[Element::Sun, Element::Water, Element::Animal]),
                cost: 0, speed: PowerSpeed::Slow,
                target_filter: PowerTargetFilter::Land{range: 0, src: |_| true, dst: |_| true},

                effect: card_rivers_bounty,
            },
            PowerCardDescription {
                name: "Wash Away",
                kind: PowerCardKind::Spirit(spirit_index),
                elements: ElementMap::from_slice(&[Element::Water, Element::Earth]),
                cost: 1, speed: PowerSpeed::Slow,
                target_filter: PowerTargetFilter::Land{range: 1, src: |_| true, dst: |_| true},

                effect: card_wash_away,
            },
        ]
    }
    fn get_power_progression(&self) -> Vec<&'static str> {
        vec![
            "Uncanny Melting",
            "Nature's Resilence",
            "Pull Beneath the Hungry Earth",
            "Acelerated Rot",
            "Song of Sanctity",
            "Tsunami",
            "Encompassing Ward"
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

        game.do_effect(AddPresenceEffect{land_index, spirit_index: si as u8, presence_index: 0})?;

        let spirit = game.get_spirit_mut(si as u8)?;
        
        for i in 1..13 {
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
                    game.do_effect(AddPresenceDecision{ spirit_index, distance: 1 })?;
                    game.do_effect(AddPresenceDecision{ spirit_index, distance: 1 })?;

                    Ok(())
                },
                |game, spirit_index| {
                    // Growth C
                    game.do_effect(AddPresenceDecision{ spirit_index, distance: 2 })?;
                    game.do_effect(GainPowerCardDecision{ spirit_index })?;
                    
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
        let mut reclaim_one = false;

        match top_track_min {
            1 | 2 => energy = 2,
            3 => energy = 3,
            4 | 5 => energy = 4,
            6 => energy = 5,
            _ => energy = 1,
        }

        match bot_track_min {
            1 | 2 => card_plays = 2,
            3 => card_plays = 3,
            4 => { card_plays = 3; reclaim_one = true },
            5 => { card_plays = 4; reclaim_one = true },
            6 => { card_plays = 5; reclaim_one = true },
            _ => card_plays = 1,
        }

        spirit.plays = card_plays; // TODO: effect maybe?
        
        game.do_effect(GenerateEnergyEffect{ spirit_index: spirit_index as u8, energy })?;
        if reclaim_one {
            // TODO: reclaim one decision
            game.do_effect(NotImplementedEffect { what: "Reclaim One" })?;
        }

        Ok(())
    }
}

impl SpiritDescriptionRiver {
    pub fn new() -> SpiritDescriptionRiver {
        SpiritDescriptionRiver {

        }
    }
}
