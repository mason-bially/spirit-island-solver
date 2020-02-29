
use std::{
    any::Any
};

use super::*;


#[derive(Clone)]
pub struct PersistDefenseEffect {
    pub land_index: u8,
    pub defense: u16
}

impl Effect for PersistDefenseEffect {
    fn apply_effect(&self, game: &mut GameState) -> Result<(), StepFailure> {
        game.log(format!("defending {} in land {}.", self.defense, self.land_index));

        // 1. Add defense to the land
        let land = game.get_land_mut(self.land_index)?;
        land.defense += self.defense;
        
        Ok(())
    }

    fn box_clone(&self) -> Box<dyn Effect> { Box::new(self.clone()) }
    fn as_any(&self) -> Box<dyn Any> { Box::new(self.clone()) }
}