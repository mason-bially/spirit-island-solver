use std::{
    any::Any,
    iter::*,
};

use super::*;


#[derive(Clone)]
pub struct DoDamageToLandEffect {
    pub land_index: u8,
    pub damage: u16,
}

impl Effect for DoDamageToLandEffect {
    fn apply_effect(&self, game: &mut GameState) -> Result<(), StepFailure> {
        game.log(format!("{} damage to land {}.", self.damage, self.land_index));

        //let land = game.table.lands.get_mut(self.land_index as usize).unwrap();

        // 1. Damage to land
        let blight_threshold = 2;

        if self.damage >= blight_threshold {
            game.do_effect(AddBlightEffect { land_index: self.land_index, count: 1 })?;
        }

        Ok(())
    }

    fn box_clone(&self) -> Box<dyn Effect> { Box::new(self.clone()) }
    fn as_any(&self) -> Box<dyn Any> { Box::new(self.clone()) }
}


#[derive(Clone)]
pub struct DoInvaderDamageEffect {
    pub land_index: u8,
    pub count: u16,
}

impl Effect for DoInvaderDamageEffect {
    fn apply_effect(&self, game: &mut GameState) -> Result<(), StepFailure> {
        game.log(format!("invader damage in {}.", self.land_index));

        let land = game.table.lands.get_mut(self.land_index as usize).unwrap();
        let mut invader_damage: u16 = land.invaders.iter().map(|p| p.attack).sum();

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
        invader_damage -= land.defense;

        // 2.Damage is done in two steps, one to the land and one to the dahan
        // 2a. Damage to dahan
        game.do_effect(DoDamageToDahanEffect{land_index: self.land_index, damage: invader_damage, efficent: true})?;

        // 2b. Damage to land
        game.do_effect(DoDamageToLandEffect{land_index: self.land_index, damage: invader_damage})?;

        Ok(())
    }

    fn box_clone(&self) -> Box<dyn Effect> { Box::new(self.clone()) }
    fn as_any(&self) -> Box<dyn Any> { Box::new(self.clone()) }
}
