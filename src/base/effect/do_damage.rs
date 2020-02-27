use std::{
    any::Any,
    iter::*,
};

use super::*;


#[derive(Clone)]
pub struct DoSpiritDamageEffect {
    pub land_index: u8,
    pub count: u16,
}


#[derive(Clone)]
pub struct DoInvaderDamageEffect {
    pub land_index: u8,
    pub count: u16,
}

impl Effect for DoInvaderDamageEffect {
    fn apply_effect(&self, game: &mut GameState) -> Result<(), StepFailure> {
        game.log(format!("invader damage in {}.", self.land_index));

        let land = game.map.lands.get_mut(self.land_index as usize).unwrap();
        let invader_damage: u16 = land.invaders.iter().map(|p| p.attack).sum();

        // TODO intercept and modify this damage:
        // * Adversary manipulations
        // * Spirit manipulations
        // * Powers and other effects
        //   * defense
        //   * modify invader damage
        //   * modify dahan health
        //   * modify blight threshold
        // * ...

        // 1. Defense

        // 2.Damage is done in two steps, one to the land and one to the dahan
        // 2a. Damage to dahan

        // 2b. Damage to land
        let blight_threshold = 2;

        if invader_damage >= blight_threshold {
            game.do_effect(AddBlightEffect { land_index: self.land_index, count: 1 })?;
        }

        Ok(())
    }

    fn box_clone(&self) -> Box<dyn Effect> {
        Box::new(self.clone())
    }
    fn as_any(&self) -> Box<dyn Any> {
        Box::new(self.clone())
    }
}
