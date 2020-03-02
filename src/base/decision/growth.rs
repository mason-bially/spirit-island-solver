
use std::{
    any::Any,
    collections::HashSet
};

use super::*;

pub type GrowthSubEffect = fn (&mut GameState, u8) -> Result<(), StepFailure>;

#[derive(Clone)]
pub struct ChooseGrowthDecision {
    pub spirit_index: u8,
    pub count: usize,
    pub choices: Vec<GrowthSubEffect>,
}

impl Effect for ChooseGrowthDecision {
    fn apply_effect(&self, game: &mut GameState) -> Result<(), StepFailure> {
        // 1. Get the decision
        let choice = match game.consume_choice()?
        {
            DecisionChoice::Sequence(res) => Ok(res),
            _ => Err(StepFailure::DecisionMismatch),
        }?;

        // 1a. Verify it's size
        if choice.len() != self.count {
            return Err(StepFailure::InternalError("must grow exact amount of times.".to_string()));
        }

        // 1b. Verify it's contents
        for schoice in choice.iter() {
            if !(*schoice < self.choices.len()) {
                return Err(StepFailure::InternalError("choice out of range".to_string()));
            }
        }

        // 1c. Verify uniqueness
        let mut uniq = HashSet::new();
        choice.iter().all(|x| uniq.insert(x));

        if uniq.len() != choice.len() {
            return Err(StepFailure::InternalError("duplicate growth choices!".to_string()));
        }
        
        game.log_decision("choosing growths...".to_string());

        // 2. Run the choice as outself
        for schoice in choice {
            (self.choices[schoice])(game, self.spirit_index)?;
        }

        Ok(())
    }

    fn box_clone(&self) -> Box<dyn Effect> { Box::new(self.clone()) }
    fn as_any(&self) -> Box<dyn Any> { Box::new(self.clone()) }

    fn as_decision(&self) -> Option<Box<dyn Decision>> { Some(Box::new(self.clone())) }
}

impl Decision for ChooseGrowthDecision {
    // TODO: x choose y
    fn valid_choices(&self, game: &GameState) -> Vec<DecisionChoice> {
        (0..self.choices.len()).map(|index| DecisionChoice::Sequence(vec![index])).collect()
    }
}


#[derive(Clone)]
pub struct GainMinorPowerCardDecision {
    pub spirit_index: u8,
    pub draw_count: usize,
}

impl Effect for GainMinorPowerCardDecision {
    fn apply_effect(&self, game: &mut GameState) -> Result<(), StepFailure> {
        game.log_decision(format!("gain minor power card for {} (from {}).", self.spirit_index, self.draw_count));

        // 1. Setup the draw/pending state
        game.minor_powers.draw_into_pending(game.rng.get_rng(), self.draw_count);

        // 2. Pick the power
        let choice = match game.consume_choice()?
        {
            DecisionChoice::Choice(res) => Ok(res),
            _ => Err(StepFailure::DecisionMismatch),
        }?;

        if !(choice < self.draw_count) {
            return Err(StepFailure::InternalError("choice out of range".to_string()));
        }

        // 3. Move card
        let card = game.minor_powers.pending.remove(choice);
        game.log_subeffect(format!("drafted |{}|.", card.desc));
        game.get_spirit_mut(self.spirit_index)?.deck.hand.push(card);

        game.minor_powers.discard_pending();


        Ok(())
    }

    fn box_clone(&self) -> Box<dyn Effect> { Box::new(self.clone()) }
    fn as_any(&self) -> Box<dyn Any> { Box::new(self.clone()) }
    
    fn as_decision(&self) -> Option<Box<dyn Decision>> { Some(Box::new(self.clone())) }
}

impl Decision for GainMinorPowerCardDecision {
    fn valid_choices(&self, game: &GameState) -> Vec<DecisionChoice> {
        (0..self.draw_count).map(|index| DecisionChoice::Choice(index)).collect()
    }
}


#[derive(Clone)]
pub struct GainMajorPowerCardDecision {
    pub spirit_index: u8,
    pub draw_count: usize,
}

impl Effect for GainMajorPowerCardDecision {
    fn apply_effect(&self, game: &mut GameState) -> Result<(), StepFailure> {
        game.log_decision(format!("gain major power card for {} (from {}).", self.spirit_index, self.draw_count));

        return game.do_effect(NotImplementedEffect { what: "MAJOR POWER DRAFTING, no sacrifice, no major powers" });

        // 1. Setup the draw/pending state
        game.major_powers.draw_into_pending(game.rng.get_rng(), self.draw_count);

        // 2. Pick the power
        let choice = match game.consume_choice()?
        {
            DecisionChoice::Choice(res) => Ok(res),
            _ => Err(StepFailure::DecisionMismatch),
        }?;

        if !(choice < self.draw_count) {
            return Err(StepFailure::InternalError("choice out of range".to_string()));
        }
        
        // 3. Move card
        let card = game.major_powers.pending.remove(choice);
        game.get_spirit_mut(self.spirit_index)?.deck.hand.push(card);

        game.major_powers.discard_pending();

        // 4. Sacrifice a card
        // TODO! sacrifice card effect
        
        Ok(())
    }

    fn box_clone(&self) -> Box<dyn Effect> { Box::new(self.clone()) }
    fn as_any(&self) -> Box<dyn Any> { Box::new(self.clone()) }
    
    fn as_decision(&self) -> Option<Box<dyn Decision>> { Some(Box::new(self.clone())) }
}

impl Decision for GainMajorPowerCardDecision {
    fn valid_choices(&self, game: &GameState) -> Vec<DecisionChoice> {
        (0..self.draw_count).map(|index| DecisionChoice::Choice(index)).collect()
    }
}


#[derive(Clone)]
pub struct GainPowerCardDecision {
    pub spirit_index: u8,
}

impl Effect for GainPowerCardDecision {
    fn apply_effect(&self, game: &mut GameState) -> Result<(), StepFailure> {
        game.log_decision(format!("gain power card for {}.", self.spirit_index));

        // 1. Choose minor or major
        let choice = match game.consume_choice()?
        {
            DecisionChoice::Choice(res) => Ok(res),
            _ => Err(StepFailure::DecisionMismatch),
        }?;

        match choice {
            0 => {
                game.do_effect(GainMinorPowerCardDecision{spirit_index: self.spirit_index, draw_count: 4})?;
            }
            1 => {
                game.do_effect(GainMajorPowerCardDecision{spirit_index: self.spirit_index, draw_count: 4})?;
            }
            _ => {
                return Err(StepFailure::InternalError("choice out of range".to_string()));
            }
        }
        
        Ok(())
    }

    fn box_clone(&self) -> Box<dyn Effect> { Box::new(self.clone()) }
    fn as_any(&self) -> Box<dyn Any> { Box::new(self.clone()) }
    
    fn as_decision(&self) -> Option<Box<dyn Decision>> { Some(Box::new(self.clone())) }
}

impl Decision for GainPowerCardDecision {
    fn valid_choices(&self, game: &GameState) -> Vec<DecisionChoice> {
        vec![
            DecisionChoice::Choice(0), // Minor
            DecisionChoice::Choice(1), // Major
        ]
    }
}
