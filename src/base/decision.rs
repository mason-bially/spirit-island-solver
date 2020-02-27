
use super::*;

pub trait Decision : Effect {
    fn valid_choices(&self, game: &GameState) -> Vec<DecisionChoice>;
}

#[derive(Clone)]
pub enum DecisionChoice {
    Sequence(Vec<u8>),
    Damage(Vec<u16>),
    TargetLand(u8),
    TargetPresence(u8),
}

mod cascade_blight;
mod do_damage;

pub use self::cascade_blight::{CascadeBlightDecision};
pub use self::do_damage::{DoDamageToDahanEffect};

