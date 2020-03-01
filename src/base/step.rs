// This file contains copyrighted assets owned by Greater Than Games.

use std::{
    fmt,
};

#[derive(Copy, Clone)]
pub enum InvaderStep {
    BlightedIsland,
    Event(u8, u8),
    FearEffect(u8),
    InvaderAction(u8, u8), // Index of action, index of card
    InvaderAdvance,
}

#[derive(Copy, Clone)]
pub enum SpiritStep {
    Growth,
    Income,
    Play,
}

#[derive(Copy, Clone)]
pub enum TurnStep {
    Spirit(SpiritStep),
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
            InvaderStep::InvaderAction(step, card) => write!(f, "Invader Step {} Card {}", step, card),
            InvaderStep::InvaderAdvance => write!(f, "Invader Advance"),
       }
    }
}

impl fmt::Display for SpiritStep {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &*self {
            SpiritStep::Growth => write!(f, "Growth"),
            SpiritStep::Income => write!(f, "Income"),
            SpiritStep::Play => write!(f, "Play"),
       }
    }
}

impl fmt::Display for TurnStep {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &*self {
            TurnStep::Spirit(step) => write!(f, "Spirit - {}", step),
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


// The state of the game state is invalid
pub enum StepFailure {
    InternalError(String),
    RulesViolation(String),
    GameOverVictory,
    GameOverDefeat,
    DecisionRequired,
    DecisionMismatch,
}

impl From<StepFailure> for Box<dyn std::error::Error> {
    fn from(failure: StepFailure) -> Self {
        match failure {
            StepFailure::GameOverVictory => 
                Box::<dyn std::error::Error>::from("Game Over Victory".to_string()),
            StepFailure::GameOverDefeat =>
                Box::<dyn std::error::Error>::from("Game Over Defeat".to_string()),
            StepFailure::InternalError(msg) =>
                Box::<dyn std::error::Error>::from(format!("Internal: {}", msg)),
            StepFailure::RulesViolation(msg) =>
                Box::<dyn std::error::Error>::from(format!("Rules Violation - {}", msg)),
            StepFailure::DecisionRequired => 
                Box::<dyn std::error::Error>::from("Decision Required".to_string()),
            StepFailure::DecisionMismatch => 
                Box::<dyn std::error::Error>::from("Decision Mismatch".to_string()),
        }
    }
}

