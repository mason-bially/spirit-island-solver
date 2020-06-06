
use std::{
    any::Any
};

use super::*;


#[derive(Clone)]
pub struct AddBlightEffect {
    pub land_index: u8
}

impl Effect for AddBlightEffect {
    fn apply_effect(&self, game: &mut GameState) -> Result<(), StepFailure> {
        game.log_effect(format_args!("blighting land {}.", self.land_index));

        // 1. Remove blight from card
        if game.blight_remaining == 0 {
            game.do_defeat("No blight is left.")?;
        }

        game.blight_remaining -= 1;

        // 2. Add blight to the land
        let land = game.get_land_mut(self.land_index)?;
        land.tokens[TokenKind::Blight] += 1;

        // 3. Kill presence
        //land.destroy_presence();

        // 4. Check for cascade
        if land.tokens[TokenKind::Blight] > 1 {
            game.do_effect(CascadeBlightDecision {src_land_index: self.land_index})?;
        }
        
        Ok(())
    }

    fn box_clone(&self) -> Box<dyn Effect> { Box::new(self.clone()) }
    fn as_any(&self) -> Box<dyn Any> { Box::new(self.clone()) }
}


// This "adds" presence from somewhere else
#[derive(Clone)]
pub struct AddPresenceEffect {
    pub land_index: u8,
    pub spirit_index: u8,
    pub presence_index: u8,
}

impl Effect for AddPresenceEffect {
    fn apply_effect(&self, game: &mut GameState) -> Result<(), StepFailure> {
        // 1. "Pickup" the presence
        let presence = game.get_spirit(self.spirit_index)?.presence[self.presence_index as usize];

        game.log_effect(format_args!("adding presence from {} to land {} for {}.", presence, self.land_index, self.spirit_index));

        game.get_spirit_mut(self.spirit_index)?.presence[self.presence_index as usize] = PresenceState::OnBoard(self.land_index);

        // 2. Add presence to the land
        let land = game.get_land_mut(self.land_index)?;
        land.presence[self.spirit_index] += 1;
        
        Ok(())
    }

    fn box_clone(&self) -> Box<dyn Effect> { Box::new(self.clone()) }
    fn as_any(&self) -> Box<dyn Any> { Box::new(self.clone()) }
}


#[derive(Clone)]
pub struct AddInvaderEffect {
    pub land_index: u8,
    pub kind: InvaderKind,
    pub count: u8,
}

impl Effect for AddInvaderEffect {
    fn apply_effect(&self, game: &mut GameState) -> Result<(), StepFailure> {
        game.log_effect(format_args!("adding {} {} invader(s) to {}.", self.count, self.kind, self.land_index));

        let land = game.get_land_mut(self.land_index)?;

        // 1. Add the invaders
        for _ in 0..self.count {
            land.invaders.push(Invader::new(self.kind));
        }
        
        Ok(())
    }

    fn box_clone(&self) -> Box<dyn Effect> { Box::new(self.clone()) }
    fn as_any(&self) -> Box<dyn Any> { Box::new(self.clone()) }
}


#[derive(Clone)]
pub struct AddDahanEffect {
    pub land_index: u8,
    pub count: u8,
}

impl Effect for AddDahanEffect {
    fn apply_effect(&self, game: &mut GameState) -> Result<(), StepFailure> {
        game.log_effect(format_args!("adding {} dahan to {}.", self.count, self.land_index));

        let land = game.get_land_mut(self.land_index)?;

        // 1. Add the invaders
        for _ in 0..self.count {
            land.dahan.push(Dahan::new());
        }
        
        Ok(())
    }

    fn box_clone(&self) -> Box<dyn Effect> { Box::new(self.clone()) }
    fn as_any(&self) -> Box<dyn Any> { Box::new(self.clone()) }
}