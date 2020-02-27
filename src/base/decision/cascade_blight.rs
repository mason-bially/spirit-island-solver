
use std::{
    any::Any
};

use super::{GameState, StepFailure, Effect, Decision, DecisionChoice};
use super::{AddBlightEffect};

#[derive(Clone)]
pub struct CascadeBlightDecision {
    pub src_land_index: u8
}

impl Effect for CascadeBlightDecision {
    fn apply_effect(&self, game: &mut GameState) -> Result<(), StepFailure> {
        let dst_land_index = match game.consume_choice()?
        {
            DecisionChoice::TargetLand(land) => Ok(land),
            _ => Err(StepFailure::DecisionMismatch),
        }?;

        let src_land_desc = game.desc.map.lands.get(self.src_land_index as usize).unwrap();
        let dst_land = game.desc.map.lands.get(dst_land_index as usize).unwrap();

        if !src_land_desc.adjacent.contains(&dst_land_index) {
            return Err(StepFailure::RulesViolation("Cascade Blight: Destination land is not adjacent to source land!".to_string()))
        }
        if !game.map.lands.get(dst_land_index as usize).unwrap().is_in_play {
            return Err(StepFailure::RulesViolation("Cascade Blight: Blight must be placed on lands that are in play!".to_string()))
        }

        game.log(format!("cascading blight to: {}", dst_land_index));

        game.do_effect(AddBlightEffect{ land_index: dst_land_index, count: 1 })?;
        
        Ok(())
    }

    fn box_clone(&self) -> Box<dyn Effect> {
        Box::new(self.clone())
    }
    fn as_any(&self) -> Box<dyn Any> {
        Box::new(self.clone())
    }
        
    fn as_decision(&self) -> Option<Box<dyn Decision>> {
        Some(Box::new(self.clone()))
    }
}

impl Decision for CascadeBlightDecision {
    fn valid_choices(&self, game: &GameState) -> Vec<DecisionChoice> {
        game.desc.map.lands
            .get(self.src_land_index as usize).unwrap()
            .adjacent.iter()
                .filter(|l| game.map.lands.get(**l as usize).unwrap().is_in_play)
                .map(|l| DecisionChoice::TargetLand(*l))
                .collect()
    }
}
