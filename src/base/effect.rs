use std::{
    any::Any,
    iter::*,
};

use super::*;

pub trait Effect {
    fn apply_effect(&self, game: &mut GameState) -> Result<(), StepFailure>;

    fn box_clone(&self) -> Box<dyn Effect>;
    fn as_any(&self) -> Box<dyn Any>;

    fn is_decision(&self) -> bool {
        false
    }
}

impl Clone for Box<dyn Effect> {
    fn clone(&self) -> Self {
        self.box_clone()
    }
}

mod add_piece;
mod do_damage;
mod invader_action;

pub use self::add_piece::{AddBlightEffect, AddInvaderEffect};
pub use self::do_damage::{DoSpiritDamageEffect, DoInvaderDamageEffect};
pub use self::invader_action::{ExploreEffect, BuildEffect, RavageEffect};

