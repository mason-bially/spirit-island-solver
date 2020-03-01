
use super::*;

pub trait Decision : Effect {
    fn valid_choices(&self, game: &GameState) -> Vec<DecisionChoice>;
}

#[derive(Clone)]
pub enum DecisionChoice {
    Sequence(Vec<usize>),
    Damage(Vec<u16>),
    TargetLand(u8),
    TargetPresence(u8),
    // for push/gather
    // tuple of (land to target, kind of piece, index of piece)
    PieceSequence(Vec<(u8, PieceKind, usize)>),
}

mod cascade_blight;
mod do_damage;
mod move_piece;

pub use self::cascade_blight::{CascadeBlightDecision};
pub use self::do_damage::{DoDamageToDahanDecision, DoDamageToInvadersDecision};
pub use self::move_piece::{PushDecision, GatherDecision};

