
use std::{
    any::Any,
    cmp::*,
};

use super::*;


#[derive(Clone)]
pub struct RemoveBlightEffect {
    pub land_index: u8,
    pub count: u8,
}

impl Effect for RemoveBlightEffect {
    fn apply_effect(&self, game: &mut GameState) -> Result<(), StepFailure> {
        game.log_effect(format_args!("removing {} blight from land {}.", self.count, self.land_index));

        // 1. Remove blight from land
        let land = game.get_land_mut(self.land_index)?;
        let blight_removed = min(self.count, land.tokens[TokenKind::Blight]);
        land.tokens[TokenKind::Blight] -= blight_removed;

        // 2. Add blight to the land
        game.blight_remaining += blight_removed;

        Ok(())
    }

    fn box_clone(&self) -> Box<dyn Effect> { Box::new(self.clone()) }
    fn as_any(&self) -> Box<dyn Any> { Box::new(self.clone()) }
}



#[derive(Clone)]
pub struct RemoveDahanEffect {
    pub land_index: u8,
    pub dahan_index: usize,
    pub destroyed: bool,
}

impl Effect for RemoveDahanEffect {
    fn apply_effect(&self, game: &mut GameState) -> Result<(), StepFailure> {
        if self.destroyed {
            game.log_effect(format_args!("destoying dahan {} in {}.", self.dahan_index, self.land_index));
        } else {
            game.log_effect(format_args!("removing dahan {} in {}.", self.dahan_index, self.land_index));
        }

        let land = game.get_land_mut(self.land_index)?;

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



#[derive(Clone)]
pub struct RemoveInvaderEffect {
    pub land_index: u8,
    pub invader_index: usize,
    pub destroyed: bool,
}

impl Effect for RemoveInvaderEffect {
    fn apply_effect(&self, game: &mut GameState) -> Result<(), StepFailure> {
        if self.destroyed {
            game.log_effect(format_args!("destoying invader {} in {}.", self.invader_index, self.land_index));
        } else {
            game.log_effect(format_args!("removing invader {} in {}.", self.invader_index, self.land_index));
        }

        let land = game.get_land_mut(self.land_index)?;

        // 1. Remove the dahan
        if !self.invader_index < land.invaders.len() {
            return Err(StepFailure::InternalError("Bad Index!".to_string()));
        }
        
        let removed = land.invaders.remove(self.invader_index);

        // 2. Fear
        match removed.kind {
            InvaderKind::City => { game.do_effect(GenerateFearEffect{fear: 2, land_index: Some(self.land_index)})?; },
            InvaderKind::Town => { game.do_effect(GenerateFearEffect{fear: 1, land_index: Some(self.land_index)})?; },
            _ => {},
        }

        // 3. Destroy triggers
        // TODO
        
        Ok(())
    }

    fn box_clone(&self) -> Box<dyn Effect> { Box::new(self.clone()) }
    fn as_any(&self) -> Box<dyn Any> { Box::new(self.clone()) }
}
