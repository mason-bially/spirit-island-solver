use std::{
    any::Any,
    iter::*,
};

use super::*;


#[derive(Clone)]
pub struct ExploreEffect {
    pub land_index: u8,
}

impl Effect for ExploreEffect {
    fn apply_effect(&self, game: &mut GameState) -> Result<(), StepFailure> {
        game.log(format!("Exploring in land {}.", self.land_index));

        let adj_lands = game.map.desc.lands_adjacent(self.land_index);
        let will_explore = adj_lands.iter().any(|l|
            game.map.lands.get(l.index_in_map as usize).unwrap().invaders.iter().any(|i| i.is_building())
        );

        if will_explore {
            game.do_effect(AddInvaderEffect {
                land_index: self.land_index,
                invader_kind: InvaderKind::Explorer,
                count: 1
            })?;
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


#[derive(Clone)]
pub struct BuildEffect {
    pub land_index: u8,
}

impl Effect for BuildEffect {
    fn apply_effect(&self, game: &mut GameState) -> Result<(), StepFailure> {
        game.log(format!("Building in land {}.", self.land_index));

        let land = game.map.lands.get(self.land_index as usize).unwrap();

        if land.invaders.len() != 0 {
            let building_type_distance : i8 = land.invaders.iter().map(|i|
                match i.kind {
                    InvaderKind::Town => -1,
                    InvaderKind::City => 1,
                    _ => 0,
                }).sum();

            game.do_effect(AddInvaderEffect {
                land_index: self.land_index,
                invader_kind: if building_type_distance > 0 { InvaderKind::Town } else { InvaderKind::City },
                count: 1
            })?;
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


#[derive(Clone)]
pub struct RavageEffect {
    pub land_index: u8,
}

impl Effect for RavageEffect {
    fn apply_effect(&self, game: &mut GameState) -> Result<(), StepFailure> {
        game.log(format!("Ravaging in land {}.", self.land_index));

        let land = game.map.lands.get_mut(self.land_index as usize).unwrap();

        if land.invaders.len() != 0 {
            // 1. Invaders to damage
            game.do_effect(DoInvaderDamageEffect { land_index: self.land_index, count: 1 })?;

            // 2. Dahan counter attack

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
