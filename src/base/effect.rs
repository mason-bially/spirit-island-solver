use std::{
    any::Any,
};

use super::*;

pub trait Effect : Send + Sync {
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


pub type SubEffect = fn (&mut GameState) -> Result<(), StepFailure>;


mod add_piece;
mod do_damage;
mod fear;
mod growth;
mod invader_action;
mod meta;
mod persist;
mod remove_piece;

pub use self::add_piece::{AddBlightEffect, AddPresenceEffect, AddInvaderEffect, AddDahanEffect};
pub use self::do_damage::{DoDamageToLandEffect, DoInvaderAttackEffect, DoDahanAttackEffect};
pub use self::fear::{GenerateFearEffect};
pub use self::growth::{GenerateEnergyEffect};
pub use self::invader_action::{ExploreEffect, BuildEffect, RavageEffect};
pub use self::meta::{NotImplementedEffect, ForAllLandsDoEffect};
pub use self::persist::{PersistDefenseEffect};
pub use self::remove_piece::{RemoveBlightEffect, RemoveDahanEffect, RemoveInvaderEffect};

