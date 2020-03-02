
use std::{
    any::Any
};

use super::*;


#[derive(Clone)]
pub struct CascadeBlightDecision {
    pub src_land_index: u8
}

impl Effect for CascadeBlightDecision {
    fn apply_effect(&self, game: &mut GameState) -> Result<(), StepFailure> {
        let dst_land_index
            = match game.consume_choice()?
            {
                DecisionChoice::TargetLand(land) => Ok(land),
                _ => Err(StepFailure::DecisionMismatch),
            }?;

        if !game.get_land_desc(self.src_land_index)?.adjacent.contains(&dst_land_index) {
            return Err(StepFailure::RulesViolation("Cascade Blight: Destination land is not adjacent to source land!".to_string()))
        }
        
        if !game.get_land(dst_land_index)?.is_in_play {
            return Err(StepFailure::RulesViolation("Cascade Blight: Blight must be placed on lands that are in play!".to_string()))
        }

        game.log_decision(format_args!("cascading blight to: {}", dst_land_index));

        game.do_effect(AddBlightEffect{ land_index: dst_land_index })?;
        
        Ok(())
    }

    fn box_clone(&self) -> Box<dyn Effect> { Box::new(self.clone()) }
    fn as_any(&self) -> Box<dyn Any> { Box::new(self.clone()) }
        
    fn as_decision(&self) -> Option<Box<dyn Decision>> { Some(Box::new(self.clone())) }
}

impl Decision for CascadeBlightDecision {
    fn valid_choices(&self, game: &GameState) -> Vec<DecisionChoice> {
        // TODO resultify
        game.get_land_desc(self.src_land_index).ok().unwrap()
            .adjacent.iter()
                // TODO resultify
                .filter(|l| game.get_land(**l).ok().unwrap().is_in_play)
                .map(|l| DecisionChoice::TargetLand(*l))
                .collect()
    }
}
