
use std::{
    any::Any
};

use super::*;

pub trait Decision : Effect {
    fn valid_choices(&self, game: &GameState) -> Vec<DecisionChoice>;
}

#[derive(Clone)]
pub enum DecisionChoice {
    TargetLand(u8),
    SequenceDecision(Vec<u8>),
}

mod cascade_blight;

pub use self::cascade_blight::{CascadeBlightDecision};

