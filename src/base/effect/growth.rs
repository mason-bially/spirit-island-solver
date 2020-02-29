
use std::{
    any::Any,
};

use super::*;


#[derive(Clone)]
pub struct GenerateEnergyEffect {
    pub spirit_index: u8,
    pub energy: u8,
}

impl Effect for GenerateEnergyEffect {
    fn apply_effect(&self, game: &mut GameState) -> Result<(), StepFailure> {
        game.log(format!("adding {} energy to {}.", self.energy, self.spirit_index));

        // 1. Add energy to the spirit
        game.get_spirit_mut(self.spirit_index)?.energy += self.energy;
        
        Ok(())
    }

    fn box_clone(&self) -> Box<dyn Effect> { Box::new(self.clone()) }
    fn as_any(&self) -> Box<dyn Any> { Box::new(self.clone()) }
}
