
use std::{
    any::Any,
    iter::*,
};

use super::*;


#[derive(Clone)]
pub struct ChooseEffectDecision {
    pub choices: Vec<SubEffect>,
}

impl Effect for ChooseEffectDecision {
    fn apply_effect(&self, game: &mut GameState) -> Result<(), StepFailure> {
        // 1. Get the decision
        let choice = match game.consume_choice()?
        {
            DecisionChoice::Choice(res) => Ok(res),
            _ => Err(StepFailure::DecisionMismatch),
        }?;

        if !(choice < self.choices.len()) {
            return Err(StepFailure::InternalError("choice out of range".to_string()));
        }
        
        game.log_decision(format_args!("choosing effect..."));

        // Run the choice as outself
        (self.choices[choice])(game)
    }

    fn box_clone(&self) -> Box<dyn Effect> { Box::new(self.clone()) }
    fn as_any(&self) -> Box<dyn Any> { Box::new(self.clone()) }
    
    fn as_decision(&self) -> Option<Box<dyn Decision>> { Some(Box::new(self.clone())) }
}

impl Decision for ChooseEffectDecision {
    fn valid_choices(&self, game: &GameState) -> Vec<DecisionChoice> {
        game.choices.iter().enumerate().map(|(index, _)| DecisionChoice::Choice(index)).collect()
    }
}
