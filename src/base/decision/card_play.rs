
use std::{
    any::Any,
    iter::*,
    collections::{HashSet},
};

use super::*;


#[derive(Clone)]
pub struct CardPlaysDecision {
    pub spirit_index: u8,
}

impl Effect for CardPlaysDecision {
    fn apply_effect(&self, game: &mut GameState) -> Result<(), StepFailure> {
        // 1. Get the decision
        let mut choice = match game.consume_choice()?
        {
            DecisionChoice::Sequence(res) => Ok(res),
            _ => Err(StepFailure::DecisionMismatch),
        }?;

        let hand_size = game.get_spirit(self.spirit_index)?.deck.hand.len();

        // 1a. Verify it's contents
        for schoice in choice.iter() {
            if !(*schoice < hand_size) {
                return Err(StepFailure::InternalError("choice out of range".to_string()));
            }
        }

        // 1b. Verify uniqueness
        let mut uniq = HashSet::new();
        choice.iter().all(|x| uniq.insert(x));

        if uniq.len() != choice.len() {
            return Err(StepFailure::InternalError("duplicate play choices!".to_string()));
        }

        game.log_decision("choosing card plays...".to_string());

        // 2. Move the cards to pending
        choice.sort();
        choice.reverse();

        let spirit = game.get_spirit_mut(self.spirit_index)?;
        for schoice in choice {
            spirit.deck.pending.push(spirit.deck.hand.remove(schoice));
        }

        Ok(())
    }

    fn box_clone(&self) -> Box<dyn Effect> { Box::new(self.clone()) }
    fn as_any(&self) -> Box<dyn Any> { Box::new(self.clone()) }
    
    fn as_decision(&self) -> Option<Box<dyn Decision>> { Some(Box::new(self.clone())) }
}

impl Decision for CardPlaysDecision {
    fn valid_choices(&self, game: &GameState) -> Vec<DecisionChoice> {
        let hand_size = game.get_spirit(self.spirit_index).ok().unwrap().deck.hand.len();
        (0..hand_size)
            .map(|index| DecisionChoice::Sequence(vec![index]))
            .collect()
    }
}


#[derive(Clone)]
pub struct DoCardPlayDecision {
    pub spirit_index: u8,
    pub pending_index: usize,
}

impl Effect for DoCardPlayDecision {
    fn apply_effect(&self, game: &mut GameState) -> Result<(), StepFailure> {
        // 1. Get the decision target
        game.log_decision("NOT IMPLEMENTED CHOOSING TARGET".to_string());

        Ok(())
    }

    fn box_clone(&self) -> Box<dyn Effect> { Box::new(self.clone()) }
    fn as_any(&self) -> Box<dyn Any> { Box::new(self.clone()) }
    
    fn as_decision(&self) -> Option<Box<dyn Decision>> { Some(Box::new(self.clone())) }
}

impl Decision for DoCardPlayDecision {
    fn valid_choices(&self, game: &GameState) -> Vec<DecisionChoice> {
        vec![]
    }
}


#[derive(Clone)]
pub struct DoCardPlaysDecision {
    pub power_speed: PowerSpeed,
}

impl DoCardPlaysDecision {
    fn pending_size(&self, game: &GameState) -> usize {
        game.spirits.iter()
            .map(|s| s.deck.pending.iter().filter(|c| c.desc.speed == self.power_speed).count())
            .sum()
    }
}

impl Effect for DoCardPlaysDecision {
    fn apply_effect(&self, game: &mut GameState) -> Result<(), StepFailure> {
        // 1. Sanity check
        let pend_size = self.pending_size(game);
        if pend_size == 0 {
            game.log_effect(format!("playing cards... (but no cards to play)"));
            return Ok(());
        }

        // 2. Get the decision
        let mut choice = match game.consume_choice()?
        {
            DecisionChoice::Sequence(res) => Ok(res),
            _ => Err(StepFailure::DecisionMismatch),
        }?;

        // 2a. Verify it's contents
        for schoice in choice.iter() {
            if !(*schoice < pend_size) {
                return Err(StepFailure::InternalError("choice out of range".to_string()));
            }
        }

        // 2b. Verify uniqueness
        let mut uniq = HashSet::new();
        choice.iter().all(|x| uniq.insert(x));

        if uniq.len() != choice.len() {
            return Err(StepFailure::InternalError("duplicate play choices!".to_string()));
        }

        game.log_decision("playing cards...".to_string());

        // 3. run the cards
        // TODO: hack, only one spirit
        for schoice in choice.iter() {
            game.do_effect(DoCardPlayDecision{spirit_index: 0, pending_index: *schoice})?;
        }

        Ok(())
    }

    fn box_clone(&self) -> Box<dyn Effect> { Box::new(self.clone()) }
    fn as_any(&self) -> Box<dyn Any> { Box::new(self.clone()) }
    
    fn as_decision(&self) -> Option<Box<dyn Decision>> { Some(Box::new(self.clone())) }
}

impl Decision for DoCardPlaysDecision {
    fn valid_choices(&self, game: &GameState) -> Vec<DecisionChoice> {
        let pend_size = self.pending_size(game);
        (0..pend_size)
            .map(|index| DecisionChoice::Sequence(vec![index]))
            .collect()
    }
}
