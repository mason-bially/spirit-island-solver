use std::{
    error::Error
};
use crate::base::{GameState, StepFailure, Decision, DecisionChoice};

pub trait DecisionMaker {
    fn decide(&self, state: &GameState) -> DecisionChoice;
}


pub struct SolveEngine {
    init_state: GameState,
    current_state: GameState,

    decision_maker: Box<dyn DecisionMaker>
}

impl SolveEngine {
    pub fn new(init_state: &GameState, strategy: Box<dyn DecisionMaker>) -> SolveEngine {
        SolveEngine {
            init_state: init_state.clone(),
            current_state: init_state.clone(),
            decision_maker: strategy,
        }
    }

    pub fn main(&mut self) -> Result<(), Box<dyn Error>> {
        loop {
            let mut working_state = self.current_state.clone();
            let res = working_state.step();
            match res {
                Ok(_) => {
                    self.current_state = working_state;
                    self.current_state.advance()?;
                    continue;
                }
                Err(StepFailure::GameOverVictory) => {
                    println!("Victory!    {}", working_state.game_over_reason.as_ref().unwrap());
                    return Ok(());
                }
                Err(StepFailure::GameOverDefeat) => {
                    println!("Defeat :(   {}", working_state.game_over_reason.as_ref().unwrap());
                    return Ok(());
                }
                Err(StepFailure::DecisionRequired) => {
                    let new_choice = self.decision_maker.decide(&working_state);
                    self.current_state.choices.push_back(new_choice);
                    continue;
                }
                Err(fail) => {
                    return Err(Box::<dyn std::error::Error>::from(fail));
                }
            }
        };
    }
}



pub struct SimpleDecisionMaker {

}

impl SimpleDecisionMaker {
    pub fn new() -> SimpleDecisionMaker {
        SimpleDecisionMaker {

        }
    }
}

impl DecisionMaker for SimpleDecisionMaker {
    fn decide(&self, state: &GameState) -> DecisionChoice {
        let undecided_decision = state.effect_stack.last().unwrap();
        if !undecided_decision.is_decision() {
            panic!();
        }

        let choices = undecided_decision.as_any().downcast_ref::<dyn Decision>();
    }
}
