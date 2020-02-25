use std::{
    iter::*,
};

use super::*;

pub trait Effect {
    fn apply_effect(&self, game: &mut GameState) -> Result<(), ()>;
    fn box_clone(&self) -> Box<dyn Effect>;

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

