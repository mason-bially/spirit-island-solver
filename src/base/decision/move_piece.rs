
use std::{
    any::Any,
    iter::*,
    cmp::*,
    collections::HashSet,
};

use itertools::Itertools;

use super::*;


pub fn piece_kind_vec_to_string(pkl: &Vec<PieceKind>) -> String {
    pkl.iter()
        .map(|pk| format!("{}", pk))
        .intersperse(",".to_string())
        .collect::<Vec<_>>()
        .concat()
}


#[derive(Clone)]
pub struct PushDecision {
    pub land_index: u8,
    pub count: u8,
    pub kinds: Vec<PieceKind>,
    pub may: bool,
}

impl Effect for PushDecision {
    fn apply_effect(&self, game: &mut GameState) -> Result<(), StepFailure> {

        // 1. Get possible source count
        let total_source_count: usize;
        {
            let src_land = game.get_land(self.land_index)?;
            total_source_count = self.kinds.iter().map(|pk| src_land.get_count(pk)).sum();

            // 1a. Sanity check
            if total_source_count == 0 {
                game.log_effect(format!("push {} {} from {} (but no sources!).", self.count, piece_kind_vec_to_string(&self.kinds), self.land_index));
                return Ok(());
            }
        }

        if self.may {
            game.log_decision(format!("may push {} {} from {}.", self.count, piece_kind_vec_to_string(&self.kinds), self.land_index));
        }
        else {
            game.log_decision(format!("push {} {} from {}.", self.count, piece_kind_vec_to_string(&self.kinds), self.land_index));
        }

        // 2. Get the decision
        let sequence = match game.consume_choice()?
        {
            DecisionChoice::PieceSequence(res) => Ok(res),
            _ => Err(StepFailure::DecisionMismatch),
        }?;

        // 2a. Verify it's size
        if sequence.len() > total_source_count {
            return Err(StepFailure::InternalError("attempting to push more pieces than valid sources.".to_string()));
        }
        if !self.may && sequence.len() != min(total_source_count, self.count as usize) {
            return Err(StepFailure::RulesViolation("Must push all valid sources.".to_string()));
        }

        // 2b. Verify uniqueness
        let mut uniq = HashSet::new();
        sequence.iter().all(|x| uniq.insert(x));

        if uniq.len() != sequence.len() {
            return Err(StepFailure::InternalError("duplicate push sources!".to_string()));
        }

        // 2c. Verify it's sequence of operations
        let src_land = game.get_land_mut(self.land_index)?;

        for (l, pk, i) in sequence.iter() {
            if !src_land.desc.adjacent.contains(l) {
                return Err(StepFailure::RulesViolation("Can only push to adjacent lands.".to_string()));
            }
            if *i < src_land.get_count(pk) {
                return Err(StepFailure::InternalError("push source index out of bounds.".to_string()));
            }
        }

        // 3. Perform it
        // TODO

        Ok(())
    }

    fn box_clone(&self) -> Box<dyn Effect> { Box::new(self.clone()) }
    fn as_any(&self) -> Box<dyn Any> { Box::new(self.clone()) }

    fn as_decision(&self) -> Option<Box<dyn Decision>> { Some(Box::new(self.clone())) }
}

impl Decision for PushDecision {
    fn valid_choices(&self, game: &GameState) -> Vec<DecisionChoice> {
        vec![
            DecisionChoice::PieceSequence(vec![])
        ]
    }
}


#[derive(Clone)]
pub struct GatherDecision {
    pub land_index: u8,
    pub count: u8,
    pub kinds: Vec<PieceKind>,
    pub may: bool,
}

impl Effect for GatherDecision {
    fn apply_effect(&self, game: &mut GameState) -> Result<(), StepFailure> {
        // 1. Get possible target count
        let total_target_count: usize;
        {
            let src_lands = game.get_adjacent_lands(self.land_index)?;
            total_target_count
                = src_lands.iter()
                    .map(|l| self.kinds.iter().map(|pk| l.get_count(pk)).sum::<usize>())
                    .sum();

            // 1a. Sanity check
            if total_target_count == 0 {
                game.log_effect(format!("gather {} {} from {} (but no targets!).", self.count, piece_kind_vec_to_string(&self.kinds), self.land_index));
                return Ok(());
            }
        }

        if self.may {
            game.log_decision(format!("may gather {} {} from {}.", self.count, piece_kind_vec_to_string(&self.kinds), self.land_index));
        }
        else {
            game.log_decision(format!("gather {} {} from {}.", self.count, piece_kind_vec_to_string(&self.kinds), self.land_index));
        }


        // 2. Get the decision
        let sequence = match game.consume_choice()?
        {
            DecisionChoice::PieceSequence(res) => Ok(res),
            _ => Err(StepFailure::DecisionMismatch),
        }?;

        // 2a. Verify it's size
        if sequence.len() > total_target_count {
            return Err(StepFailure::InternalError("attempting to gather more pieces than valid sources.".to_string()));
        }
        if !self.may && sequence.len() != min(total_target_count, self.count as usize) {
            return Err(StepFailure::RulesViolation("Must gather all valid targets.".to_string()));
        }

        // 2b. Verify uniqueness
        let mut uniq = HashSet::new();
        sequence.iter().all(|x| uniq.insert(x));

        if uniq.len() != sequence.len() {
            return Err(StepFailure::InternalError("duplicate gather targets!".to_string()));
        }

        // 2c. Verify it's sequence of operations
        let dst_land_desc = game.get_land_desc(self.land_index)?;

        for (l, pk, i) in sequence.iter() {
            if !dst_land_desc.adjacent.contains(l) {
                return Err(StepFailure::RulesViolation("Can only gather from adjacent lands.".to_string()));
            }

            let src_land = game.get_land(*l)?;
            if *i < src_land.get_count(pk) {
                return Err(StepFailure::InternalError("gather target index out of bounds.".to_string()));
            }
        }

        // 3. Perform it
        // TODO

        Ok(())
    }

    fn box_clone(&self) -> Box<dyn Effect> { Box::new(self.clone()) }
    fn as_any(&self) -> Box<dyn Any> { Box::new(self.clone()) }

    fn as_decision(&self) -> Option<Box<dyn Decision>> { Some(Box::new(self.clone())) }
}

impl Decision for GatherDecision {
    fn valid_choices(&self, game: &GameState) -> Vec<DecisionChoice> {
        vec![
            DecisionChoice::PieceSequence(vec![])
        ]
    }
}
