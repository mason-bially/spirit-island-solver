use std::{
    any::Any,
};

use super::*;

pub trait Effect {
    fn apply_effect(&self, game: &mut GameState) -> Result<(), StepFailure>;

    fn box_clone(&self) -> Box<dyn Effect>;
    fn as_any(&self) -> Box<dyn Any>;

    fn as_decision(&self) -> Option<Box<dyn Decision>> {
        None
    }
}

impl Clone for Box<dyn Effect> {
    fn clone(&self) -> Self {
        self.box_clone()
    }
}

mod add_piece;
mod do_damage;
mod fear;
mod growth;
mod invader_action;
mod meta;
mod remove_piece;

pub use self::add_piece::{AddBlightEffect, AddPresenceEffect, AddInvaderEffect};
pub use self::do_damage::{DoDamageToLandEffect, DoInvaderAttackEffect, DoDahanAttackEffect};
pub use self::fear::{GenerateFearEffect};
pub use self::growth::{};
pub use self::invader_action::{ExploreEffect, BuildEffect, RavageEffect};
pub use self::meta::{NotImplementedEffect};
pub use self::remove_piece::{RemoveDahanEffect, RemoveInvaderEffect};

