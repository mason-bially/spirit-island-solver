
use std::{
    any::Any,
    iter::*,
    cmp::*,
};

use super::*;


pub fn allocate_efficent_damage(damage: u16, targets: Vec<u8>) -> Vec<u16> {
    let enumerated: Vec<(usize, &u8)> = targets.iter().enumerate().collect();

    let mut res = Vec::<u16>::new();
    res.resize(targets.len(), 0);

    let mut damage_remaining = damage;
    for enum_index in 0..targets.len() {
        let (target_index, &target) = enumerated[enum_index];

        let damage_done = min(target as u16, damage_remaining);

        res[target_index] = damage_done;
        damage_remaining -= damage_done;
    };

    res
}


#[derive(Clone)]
pub struct DoDamageToDahanDecision {
    pub land_index: u8,
    pub damage: u16,
    pub efficent: bool,
}

impl Effect for DoDamageToDahanDecision {
    fn apply_effect(&self, game: &mut GameState) -> Result<(), StepFailure> {
        if self.efficent {
            game.log(format!("{} damage to dahan in {} (efficently).", self.damage, self.land_index));
        }
        else {
            game.log(format!("{} damage to dahan in {}.", self.damage, self.land_index));
        }

        let dahan = game.table.lands[self.land_index as usize].dahan.clone();

        // 1. Sanity check
        if dahan.len() == 0 {
            game.log_subeffect("no dahan!".to_string());
            return Ok(());
        }

        // 2. Get the damage decision
        let damage_layout: Vec<u16> = if self.efficent {
            allocate_efficent_damage(self.damage, dahan.iter().map(|d| d.health_cur).collect())
        } else {
            match game.consume_choice()?
            {
                DecisionChoice::Damage(res) => Ok(res),
                _ => Err(StepFailure::DecisionMismatch),
            }?
        };

        // 3. Actually perform the damage
        let mut destroyed_dahan: Vec<usize> = Vec::new();
        let mut damage_remaining = self.damage;
        {
            let dahan = &mut game.table.lands[self.land_index as usize].dahan;
            
            for dahan_index in 0..dahan.len() {
                let damage_to_do = damage_layout[dahan_index] as u8;
                let dahan_to_damage = &mut dahan[dahan_index];
    
                if damage_to_do > dahan_to_damage.health_cur {
                    return Err(StepFailure::RulesViolation("Cannot do more damage than current health.".to_string()))
                }
                if damage_to_do as u16 > damage_remaining {
                    return Err(StepFailure::RulesViolation("Cannot allocate more damage than is pending.".to_string()))
                }
    
                dahan_to_damage.health_cur -= damage_to_do;
                damage_remaining -= damage_to_do as u16;
    
                if dahan_to_damage.health_cur == 0 {
                    destroyed_dahan.push(dahan_index);
                }
            }
        }

        // 4. Clean up pending destroys
        destroyed_dahan.reverse(); // so that higher indexes are first
        for dahan_index in destroyed_dahan {
            game.do_effect(RemoveDahanEffect{land_index: self.land_index, dahan_index, destroyed: true})?;
        }

        if damage_remaining != 0 {
            game.log_subeffect(format!("{} damage to dahan in {} spilled over.", damage_remaining, self.land_index));
        }

        Ok(())
    }

    fn box_clone(&self) -> Box<dyn Effect> { Box::new(self.clone()) }
    fn as_any(&self) -> Box<dyn Any> { Box::new(self.clone()) }

    fn as_decision(&self) -> Option<Box<dyn Decision>> { Some(Box::new(self.clone())) }
}

impl Decision for DoDamageToDahanDecision {
    fn valid_choices(&self, game: &GameState) -> Vec<DecisionChoice> {
        vec![
            // TODO, this is litterally the worst choice:
            DecisionChoice::Damage(allocate_efficent_damage(self.damage, game.table.lands.get(self.land_index as usize).unwrap().dahan.iter().map(|d| d.health_cur).collect()))
        ]
    }
}


#[derive(Clone)]
pub struct DoDamageToInvadersDecision {
    pub land_index: u8,
    pub damage: u16,
}

impl Effect for DoDamageToInvadersDecision {
    fn apply_effect(&self, game: &mut GameState) -> Result<(), StepFailure> {
        game.log(format!("{} damage to invaders in {}.", self.damage, self.land_index));

        let invaders = game.table.lands[self.land_index as usize].invaders.clone();

        // 1. Sanity check
        if invaders.len() == 0 {
            game.log_subeffect("no invaders!".to_string());
            return Ok(());
        }

        // 2. Get the damage decision
        let damage_layout: Vec<u16>
            = match game.consume_choice()?
            {
                DecisionChoice::Damage(res) => Ok(res),
                _ => Err(StepFailure::DecisionMismatch),
            }?;

        // 3. Actually perform the damage
        let mut destroyed_invaders: Vec<usize> = Vec::new();
        let mut damage_remaining = self.damage;
        {
            let invaders = &mut game.table.lands[self.land_index as usize].invaders;
            
            for invader_index in 0..invaders.len() {
                let damage_to_do = damage_layout[invader_index] as u8;
                let invader_to_damage = &mut invaders[invader_index];
    
                if damage_to_do > invader_to_damage.health_cur {
                    return Err(StepFailure::RulesViolation("Cannot do more damage than current health.".to_string()))
                }
                if damage_to_do as u16 > damage_remaining {
                    return Err(StepFailure::RulesViolation("Cannot allocate more damage than is pending.".to_string()))
                }
    
                invader_to_damage.health_cur -= damage_to_do;
                damage_remaining -= damage_to_do as u16;
    
                if invader_to_damage.health_cur == 0 {
                    destroyed_invaders.push(invader_index);
                }
            }
        }

        // 4. Clean up pending destroys
        destroyed_invaders.reverse(); // so that higher indexes are first
        for invader_index in destroyed_invaders {
            game.do_effect(RemoveInvaderEffect{land_index: self.land_index, invader_index, destroyed: true})?;
        }

        if damage_remaining != 0 {
            game.log_subeffect(format!("{} damage to dahan in {} spilled over.", damage_remaining, self.land_index));
        }

        Ok(())
    }

    fn box_clone(&self) -> Box<dyn Effect> { Box::new(self.clone()) }
    fn as_any(&self) -> Box<dyn Any> { Box::new(self.clone()) }

    fn as_decision(&self) -> Option<Box<dyn Decision>> { Some(Box::new(self.clone())) }
}

impl Decision for DoDamageToInvadersDecision {
    fn valid_choices(&self, game: &GameState) -> Vec<DecisionChoice> {
        vec![
            // TODO, this is probably the best choice:
            DecisionChoice::Damage(allocate_efficent_damage(self.damage, game.table.lands.get(self.land_index as usize).unwrap().invaders.iter().map(|d| d.health_cur).collect()))
        ]
    }
}
