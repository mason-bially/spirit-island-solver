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
        game.log_effect(format_args!("Exploring in land {}.", self.land_index));

        let adj_lands = game.table.desc.get_adjacent_lands(self.land_index);
        let will_explore = adj_lands.iter().any(|l|
            game.get_land(l.index_on_table).ok().unwrap()
                .invaders.iter()
                .any(|i| i.is_building())
        );

        if will_explore {
            game.do_effect(AddInvaderEffect {
                land_index: self.land_index,
                kind: InvaderKind::Explorer,
                count: 1
            })?;
        }

        Ok(())
    }

    fn box_clone(&self) -> Box<dyn Effect> { Box::new(self.clone()) }
    fn as_any(&self) -> Box<dyn Any> { Box::new(self.clone()) }
}


#[derive(Clone)]
pub struct BuildEffect {
    pub land_index: u8,
}

impl Effect for BuildEffect {
    fn apply_effect(&self, game: &mut GameState) -> Result<(), StepFailure> {
        game.log_effect(format_args!("Building in land {}.", self.land_index));

        let land = game.get_land(self.land_index)?;

        if land.invaders.len() != 0 {
            let building_type_distance : i8 = land.invaders.iter().map(|i|
                match i.kind {
                    InvaderKind::Town => -1,
                    InvaderKind::City => 1,
                    _ => 0,
                }).sum();

            game.do_effect(AddInvaderEffect {
                land_index: self.land_index,
                kind: if building_type_distance >= 0 { InvaderKind::Town } else { InvaderKind::City },
                count: 1
            })?;
        }

        Ok(())
    }

    fn box_clone(&self) -> Box<dyn Effect> { Box::new(self.clone()) }
    fn as_any(&self) -> Box<dyn Any> { Box::new(self.clone()) }
}


#[derive(Clone)]
pub struct RavageEffect {
    pub land_index: u8,
}

impl Effect for RavageEffect {
    fn apply_effect(&self, game: &mut GameState) -> Result<(), StepFailure> {
        game.log_effect(format_args!("Ravaging in land {}.", self.land_index));

        let land = game.get_land(self.land_index)?;

        if land.invaders.len() != 0 {
            // 1. Invaders to damage
            game.do_effect(DoInvaderAttackEffect { land_index: self.land_index })?;

            // 2. Dahan counter attack
            game.do_effect(DoDahanAttackEffect { land_index: self.land_index })?;
        }

        Ok(())
    }

    fn box_clone(&self) -> Box<dyn Effect> { Box::new(self.clone()) }
    fn as_any(&self) -> Box<dyn Any> { Box::new(self.clone()) }
}
