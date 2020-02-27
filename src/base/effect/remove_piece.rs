
use std::{
    any::Any
};

use super::*;



#[derive(Clone)]
pub struct RemoveDahanEffect {
    pub land_index: u8,
    pub dahan_index: usize,
    pub destroyed: bool,
}

impl Effect for RemoveDahanEffect {
    fn apply_effect(&self, game: &mut GameState) -> Result<(), StepFailure> {
        if self.destroyed {
            game.log(format!("destoying dahan {} in {}.", self.dahan_index, self.land_index));
        } else {
            game.log(format!("removing dahan {} in {}.", self.dahan_index, self.land_index));
        }

        let land = game.table.lands.get_mut(self.land_index as usize).unwrap();

        // 1. Remove the dahan
        if !self.dahan_index < land.dahan.len() {
            return Err(StepFailure::InternalError("Bad Index!".to_string()));
        }
        
        land.dahan.remove(self.dahan_index);

        // 2. Destroy triggers
        // TODO
        
        Ok(())
    }

    fn box_clone(&self) -> Box<dyn Effect> { Box::new(self.clone()) }
    fn as_any(&self) -> Box<dyn Any> { Box::new(self.clone()) }
}
