use std::{
    any::Any,
    iter::*,
    cmp::{min}
};

use super::*;


#[derive(Clone)]
pub struct DoDamageToLandEffect {
    pub land_index: u8,
    pub damage: u16,
}

impl Effect for DoDamageToLandEffect {
    fn apply_effect(&self, game: &mut GameState) -> Result<(), StepFailure> {
        game.log_effect(format_args!("{} damage to land {}.", self.damage, self.land_index));

        // 1. Damage to land
        let blight_threshold = 2;

        if self.damage >= blight_threshold {
            game.do_effect(AddBlightEffect { land_index: self.land_index })?;
        }

        Ok(())
    }

    fn box_clone(&self) -> Box<dyn Effect> { Box::new(self.clone()) }
    fn as_any(&self) -> Box<dyn Any> { Box::new(self.clone()) }
}


#[derive(Clone)]
pub struct DoInvaderAttackEffect {
    pub land_index: u8,
}

impl Effect for DoInvaderAttackEffect {
    fn apply_effect(&self, game: &mut GameState) -> Result<(), StepFailure> {
        let land = game.get_land(self.land_index)?;
        if land.invaders.len() == 0 {
            return Ok(());
        }

        game.log_effect(format_args!("invaders attack in {}.", self.land_index));

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
        invader_damage = if land.defense >= invader_damage { 0 } else { invader_damage - land.defense };
        game.log_subeffect(format_args!("defense {} lowers damage to {}.", land.defense, invader_damage));

        // 2.Damage is done in two steps, one to the land and one to the dahan
        // 2a. Damage to dahan
        game.do_effect(DoDamageToDahanDecision{land_index: self.land_index, damage: invader_damage, efficent: true})?;

        // 2b. Damage to land
        game.do_effect(DoDamageToLandEffect{land_index: self.land_index, damage: invader_damage})?;

        Ok(())
    }

    fn box_clone(&self) -> Box<dyn Effect> { Box::new(self.clone()) }
    fn as_any(&self) -> Box<dyn Any> { Box::new(self.clone()) }
}


#[derive(Clone)]
pub struct DoDahanAttackEffect {
    pub land_index: u8,
}

impl Effect for DoDahanAttackEffect {
    fn apply_effect(&self, game: &mut GameState) -> Result<(), StepFailure> {
        let land = game.get_land(self.land_index)?;
        if land.dahan.len() == 0 {
            return Ok(());
        }

        game.log_effect(format_args!("dahan attack in {}.", self.land_index));

        let dahan_damage: u16 = land.dahan.iter().map(|p| p.attack).sum();

        // 1. Do the damage
        game.do_effect(DoDamageToInvadersDecision{land_index: self.land_index, damage: dahan_damage})?;

        Ok(())
    }

    fn box_clone(&self) -> Box<dyn Effect> { Box::new(self.clone()) }
    fn as_any(&self) -> Box<dyn Any> { Box::new(self.clone()) }
}


#[derive(Clone)]
pub struct DoDamageToEachInvaderEffect {
    pub land_index: u8,
    pub damage: u16,
    pub kinds: InvaderMap<bool>,
}

impl Effect for DoDamageToEachInvaderEffect {
    fn apply_effect(&self, game: &mut GameState) -> Result<(), StepFailure> {
        let invaders = game.get_land(self.land_index)?.invaders.clone();

        game.log_effect(format_args!("{} damage to all invaders in {}.", self.damage, self.land_index));

        // 2. Actually perform the damage
        let mut total_damage: u16 = 0;
        let mut destroyed_invaders: Vec<usize> = Vec::new();
        {
            let invaders_mut = &mut game.get_land_mut(self.land_index)?.invaders;
            
            for invader_index in 0..invaders.len() {
                if !self.kinds[invaders[invader_index].kind] {
                    continue;
                }

                let invader_to_damage = &mut invaders_mut[invader_index];
                let health_cur = invader_to_damage.health_cur as u16;

                if self.damage >= health_cur {
                    total_damage += health_cur;
                    invader_to_damage.health_cur = 0;
                    destroyed_invaders.push(invader_index);
                } else {
                    total_damage += self.damage;
                    invader_to_damage.health_cur -= self.damage as u8;
                }
            }
        }

        // 3. Clean up pending destroys
        destroyed_invaders.sort();
        destroyed_invaders.reverse(); // so that higher indexes are first
        for invader_index in destroyed_invaders {
            game.do_effect(RemoveInvaderEffect{land_index: self.land_index, invader_index, destroyed: true})?;
        }

        game.log_subeffect(format_args!("{} total damage in {}.", total_damage, self.land_index));

        Ok(())
    }

    fn box_clone(&self) -> Box<dyn Effect> { Box::new(self.clone()) }
    fn as_any(&self) -> Box<dyn Any> { Box::new(self.clone()) }
}
