
use super::*;

pub trait Decision : Effect {
    fn valid_choices(&self, game: &GameState) -> Vec<DecisionChoice>;
}

#[derive(Clone)]
pub enum DecisionChoice {
    Sequence(Vec<usize>),
    Choice(usize),
    
    // targetings
    TargetLand{target_land: u8, source_land: u8},
    //TargetPresence{target_land: u8, target_spirit: u8, source_land: u8},
    TargetSpirit{target_spirit: u8},

    // presence placement
    PlacePresence{spirit: u8, target_land: u8, source_presence: u8},

    Damage(Vec<u16>),
    // for push/gather
    // tuple of (land to target, kind of piece, index of piece)
    PieceSequence(Vec<(u8, PieceKind, usize)>),
}

mod card_play;
mod cascade_blight;
mod do_damage;
mod growth;
mod meta;
mod move_piece;

pub use self::card_play::{DoCardPlayDecision, DoCardPlaysDecision, CardPlaysDecision};
pub use self::cascade_blight::{CascadeBlightDecision};
pub use self::do_damage::{DoDamageToDahanDecision, DoDamageToInvadersDecision};
pub use self::growth::{AddPresenceDecision, ChooseGrowthDecision, GainMinorPowerCardDecision, GainMajorPowerCardDecision, GainPowerCardDecision};
pub use self::meta::{ChooseEffectDecision};
pub use self::move_piece::{PushDecision, GatherDecision};

