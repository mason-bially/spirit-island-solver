
use std::{
    any::Any,
};

use super::*;


#[derive(Clone)]
pub struct MayPlaySlowsAsFastsEffect {
    pub spirit_index: u8,
    pub amount: u8,
}

impl Effect for MayPlaySlowsAsFastsEffect {
    fn apply_effect(&self, game: &mut GameState) -> Result<(), StepFailure> {
        game.log_effect(format_args!("{} may use {} slow as fast.", self.spirit_index, self.amount));

        // 1. Reclaim all cards
        game.get_spirit_mut(self.spirit_index)?.may_play_slows_as_fasts += self.amount;
        
        Ok(())
    }

    fn box_clone(&self) -> Box<dyn Effect> { Box::new(self.clone()) }
    fn as_any(&self) -> Box<dyn Any> { Box::new(self.clone()) }
}
