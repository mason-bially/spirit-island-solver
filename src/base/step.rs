// This file contains copyrighted assets owned by Greater Than Games.

use std::{
    rc::Rc,
    fmt,
};

use super::{
    concept::{InvaderActionKind},
    board::{LandKind, LandDescription},
};

impl fmt::Display for InvaderActionKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &*self {
            InvaderActionKind::Ravage => write!(f, "Ravage"),
            InvaderActionKind::Build => write!(f, "Build"),
            InvaderActionKind::Explore => write!(f, "Explore"),
       }
    }
}

#[derive(Copy, Clone)]
pub enum InvaderStep {
    BlightedIsland,
    Event(u8, u8),
    FearEffect(u8),
    InvaderAction(u8, InvaderActionKind),
    InvaderAdvance,
}

#[derive(Copy, Clone)]
pub enum TurnStep {
    Spirit,
    FastPower,
    Invader(InvaderStep),
    SlowPower,
    TimePasses,
}

#[derive(Copy, Clone)]
pub enum GameStep {
    Init,
    SetupSpirit,
    SetupExplore,
    Turn(u8, TurnStep),
    Victory,
    Defeat,
}

impl fmt::Display for InvaderStep {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &*self {
            InvaderStep::BlightedIsland => write!(f, "Blighted Island Effect"),
            InvaderStep::Event(card, part) => write!(f, "Event Card {} - Part {}", card, part),
            InvaderStep::FearEffect(card) => write!(f, "Fear Card {}", card),
            InvaderStep::InvaderAction(step, kind) => write!(f, "Invader {} - {}", step, kind),
            InvaderStep::InvaderAdvance => write!(f, "Invader Advance"),
       }
    }
}

impl fmt::Display for TurnStep {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &*self {
            TurnStep::Spirit => write!(f, "Spirit"),
            TurnStep::FastPower => write!(f, "Fast Powers"),
            TurnStep::Invader(step) => write!(f, "Invader - {}", step),
            TurnStep::SlowPower => write!(f, "Slow Powers"),
            TurnStep::TimePasses =>write!(f, "Time Passes"),
       }
    }
}

impl fmt::Display for GameStep {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &*self {
            GameStep::Init => write!(f, "Init"),
            GameStep::SetupSpirit => write!(f, "Setup Spirit"),
            GameStep::SetupExplore => write!(f, "Setup Explore"),
            GameStep::Turn(turn, step) => write!(f, "Turn {} - {}", turn + 1, step),
            GameStep::Victory => write!(f, "Victory"),
            GameStep::Defeat =>write!(f, "Defeat"),
       }
    }
}


#[derive(Copy, Clone)]
pub enum InvaderCard {
    Phase1(LandKind),
    Phase2(LandKind),
    Phase3(LandKind, LandKind),
}

impl fmt::Display for InvaderCard {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
       match &*self {
            InvaderCard::Phase1(a) => write!(f, "Phase I {}", a),
            InvaderCard::Phase2(LandKind::Ocean) => write!(f, "Phase II Coastal"),
            InvaderCard::Phase2(a) => write!(f, "Phase II {} +", a),
            InvaderCard::Phase3(a, b) => write!(f, "Phase III {}/{}", a, b),
       }
    }
}

impl InvaderCard {
    pub fn can_target(&self, land: &Rc<LandDescription>) -> bool {
        match *self {
            InvaderCard::Phase1(kind) => kind == land.kind,
            InvaderCard::Phase2(LandKind::Ocean) => land.kind != LandKind::Ocean && land.is_coastal,
            InvaderCard::Phase2(kind) => kind == land.kind,
            InvaderCard::Phase3(kind_a, kind_b) => kind_a == land.kind || kind_b == land.kind,
        }
    }
}

pub fn generate_invader_deck() -> Vec<InvaderCard> {
    vec![
        InvaderCard::Phase3(LandKind::Jungle, LandKind::Mountain),
        InvaderCard::Phase3(LandKind::Jungle, LandKind::Sands),
        InvaderCard::Phase3(LandKind::Jungle, LandKind::Wetlands),
        InvaderCard::Phase3(LandKind::Mountain, LandKind::Sands),
        InvaderCard::Phase3(LandKind::Mountain, LandKind::Wetlands),
        InvaderCard::Phase3(LandKind::Sands, LandKind::Wetlands),
        
        InvaderCard::Phase2(LandKind::Ocean),
        InvaderCard::Phase2(LandKind::Jungle),
        InvaderCard::Phase2(LandKind::Mountain),
        InvaderCard::Phase2(LandKind::Sands),
        InvaderCard::Phase2(LandKind::Wetlands),

        InvaderCard::Phase1(LandKind::Jungle),
        InvaderCard::Phase1(LandKind::Mountain),
        InvaderCard::Phase1(LandKind::Sands),
        InvaderCard::Phase1(LandKind::Wetlands),
    ]
}

pub fn invader_deck_setup_standard(cards: &mut Vec<InvaderCard>)
{
    let phase3 = cards.iter().position(|&x| if let InvaderCard::Phase3(_, _) = x { true } else { false }).unwrap();
    cards.remove(phase3);

    let phase2 = cards.iter().position(|&x| if let InvaderCard::Phase2(_) = x { true } else { false }).unwrap();
    cards.remove(phase2);

    let phase1 = cards.iter().position(|&x| if let InvaderCard::Phase1(_) = x { true } else { false }).unwrap();
    cards.remove(phase1);
}
