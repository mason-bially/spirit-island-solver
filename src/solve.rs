use crate::base::GameState;

pub trait DecisionMaker {
    fn decide(&self, solver: &mut SolveEngine);
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

    pub fn main() -> Result<(), ()> {
        Ok(())
    }
}



pub struct SimpleDecisionMaker {

}

impl SimpleDecisionMaker {
    pub fn decide(&self, solver: &mut SolveEngine) {

    }
}
