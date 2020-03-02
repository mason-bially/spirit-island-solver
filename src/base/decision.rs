
use super::*;

pub trait Decision : Effect {
    fn valid_choices(&self, game: &GameState) -> Vec<DecisionChoice>;
}

#[derive(Clone)]
pub enum DecisionChoice {
    Sequence(Vec<usize>),
    Choice(usize),
    
    TargetLand(u8),
    TargetPresence(u8),

    Damage(Vec<u16>),
    // for push/gather
    // tuple of (land to target, kind of piece, index of piece)
    PieceSequence(Vec<(u8, PieceKind, usize)>),
}

mod cascade_blight;
mod do_damage;
mod growth;
mod meta;
mod move_piece;

pub use self::cascade_blight::{CascadeBlightDecision};
pub use self::do_damage::{DoDamageToDahanDecision, DoDamageToInvadersDecision};
pub use self::growth::{ChooseGrowthDecision, GainMinorPowerCardDecision, GainMajorPowerCardDecision, GainPowerCardDecision};
pub use self::meta::{ChooseEffectDecision};
pub use self::move_piece::{PushDecision, GatherDecision};

