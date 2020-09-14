
use std::{
    any::Any,
    iter::*,
    cmp::{min},
    collections::{HashSet},
    sync::{Arc}
};

use itertools::*;

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

        // 1c. Verify energy expenditure:
        let spirit = game.get_spirit(self.spirit_index)?;
        let cost = choice.iter().map(|card| spirit.deck.hand[*card].desc.cost).sum::<u8>();

        game.log_decision(format_args!("choosing card plays..."));

        // 2. Actually "play" the cards
        let spirit_mut = game.get_spirit_mut(self.spirit_index)?;

        // 2a. Spend the energy
        spirit_mut.energy -= cost;

        // 2b. Gain elements
        // TODO: gain elements

        // 2c. Place the cards into pending
        choice.sort();
        choice.reverse();

        for schoice in choice {
            spirit_mut.deck.pending.push(spirit_mut.deck.hand.remove(schoice));
        }

        Ok(())
    }

    fn box_clone(&self) -> Box<dyn Effect> { Box::new(self.clone()) }
    fn as_any(&self) -> Box<dyn Any> { Box::new(self.clone()) }
    
    fn as_decision(&self) -> Option<Box<dyn Decision>> { Some(Box::new(self.clone())) }
}

impl Decision for CardPlaysDecision {
    fn valid_choices(&self, game: &GameState) -> Vec<DecisionChoice> {
        let spirit = game.get_spirit(self.spirit_index).ok().unwrap();

        let hand_size = spirit.deck.hand.len();
        let plays = spirit.plays as usize;

        // for each possible play length
        (0..(min(hand_size, plays)+1))
            // generate the possible combinations
            .map(|cards_to_play| (0..hand_size).combinations(cards_to_play))
            // into one list
            .fold(Vec::new(), |mut acc, v| { acc.extend(v); acc })
            .into_iter()
            // filter for those that can actually be played in combination energy wise
            .filter(|cards| {cards.iter().map(|card| spirit.deck.hand[*card].desc.cost).sum::<u8>() <= spirit.energy})
            // and turn them into decision objects
            .map(|cards| DecisionChoice::Sequence(cards))
            .collect()
    }
}


#[derive(Clone)]
pub struct DoCardPlayDecision {
    pub spirit_index: u8,
    pub pending_index: usize,
}

impl DoCardPlayDecision {
    pub fn get_valid_targets(&self, game: &GameState, card_desc: &PowerCardDescription) -> Result<Vec<PowerTarget>, StepFailure> {

        match card_desc.target_filter {
            PowerTargetFilter::Land{range, src, dst} => {
                // TODO src filter and ranges
                Ok(game.table.lands.iter().enumerate()
                    .filter(|(_, state)| dst(state))
                    .map(|(index, _)| PowerTarget::Land(index as u8))
                    .collect())
            },
            PowerTargetFilter::Spirit(target) => {
                Ok(game.spirits.iter().enumerate()
                    .filter(|(_, state)| target(state))
                    .map(|(index, _)| PowerTarget::Spirit(index as u8))
                    .collect())
            }
        }
    }
}

impl Effect for DoCardPlayDecision {
    fn apply_effect(&self, game: &mut GameState) -> Result<(), StepFailure> {
        let card_desc = Arc::clone(&game.get_spirit(self.spirit_index)?.deck.pending[self.pending_index].desc);

        // 1. Figure out the kind of card we are dealing with to get the decision
        let target = match card_desc.target_filter {
            PowerTargetFilter::Land{..} => {
                PowerTarget::Land(
                    match game.consume_choice()?
                    {
                        DecisionChoice::TargetLand{target_land, ..} => Ok(target_land),
                        _ => Err(StepFailure::DecisionMismatch),
                    }?)
            },
            PowerTargetFilter::Spirit(_) => {
                PowerTarget::Spirit(
                    match game.consume_choice()?
                    {
                        DecisionChoice::TargetSpirit{target_spirit} => Ok(target_spirit),
                        _ => Err(StepFailure::DecisionMismatch),
                    }?)
            }
        };

        // 1a. Find it in the valid possible decisions
        if !self.get_valid_targets(game, &card_desc)?.contains(&target) {
            return Err(StepFailure::RulesViolation("not given a valid target".to_string()));
        }

        game.log_decision(format_args!("playing card (targeting {})...", target));

        // 2. Invoke it! Finally!
        game.power_usages.push(PowerUsage {
            target,
            using_spirit_index: self.spirit_index,
            src_land_index: None,
        });

        game.do_effect_box(card_desc.box_clone())?;

        game.power_usages.pop();

        Ok(())
    }

    fn box_clone(&self) -> Box<dyn Effect> { Box::new(self.clone()) }
    fn as_any(&self) -> Box<dyn Any> { Box::new(self.clone()) }
    
    fn as_decision(&self) -> Option<Box<dyn Decision>> { Some(Box::new(self.clone())) }
}

impl Decision for DoCardPlayDecision {
    fn valid_choices(&self, game: &GameState) -> Vec<DecisionChoice> {
        let card_desc = Arc::clone(&game.get_spirit(self.spirit_index).ok().unwrap().deck.pending[self.pending_index].desc);

        self.get_valid_targets(game, &card_desc).ok().unwrap().into_iter()
            .map(|power_target| match power_target {
                PowerTarget::Spirit(index) => DecisionChoice::TargetSpirit{target_spirit: index},
                PowerTarget::Land(index) => DecisionChoice::TargetLand{target_land: index, source_land: 0},
            }).collect()
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
            game.log_effect(format_args!("playing cards... (but no cards to play)"));
            return Ok(());
        }

        // 2. Get the decision
        let choice = match game.consume_choice()?
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

        game.log_decision(format_args!("playing cards..."));

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
