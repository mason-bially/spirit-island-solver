use std::{
    error::Error,
    sync::{Arc, Weak, Mutex},
    collections::{VecDeque},
};

use crate::base::{GameState, StepFailure, Decision, DecisionChoice};



pub trait SolveStrategy {
    // The list of decisions to try in order
    fn decide(&self, state: &GameState) -> Vec<DecisionChoice>;
}


#[derive(Hash, Copy, Clone)]
pub struct BasicStatistics {
    pub victories: usize,
    pub defeats: usize,
    pub errors: usize,
}

impl BasicStatistics {
    pub fn new() -> BasicStatistics {
        BasicStatistics {
            victories: 0,
            defeats: 0,
            errors: 0,
        }
    }

    pub fn merge(&self, other: &BasicStatistics) -> BasicStatistics {
        BasicStatistics {
            victories: self.victories + other.victories,
            defeats: self.defeats + other.defeats,
            errors: self.errors + other.errors,
        }
    }

    pub fn consume(&mut self, terminal: StepFailure) {
        match terminal {
            StepFailure::GameOverVictory => {
                self.victories += 1;
            },
            StepFailure::GameOverDefeat => {
                self.defeats += 1;
            },
            _ => {
                self.errors += 1;
            },
        };
    }
}


#[derive(PartialEq, Eq, Hash, Copy, Clone)]
pub enum SolveBranchState {
    Inited,
    Executed,
    Expanded,
    Completed,
    Finalized,
}


pub struct SolveBranch {
    pub parent: Weak<Mutex<SolveBranch>>,
    pub decision_edge: VecDeque<DecisionChoice>,

    pub game_state: GameState,
    pub state: SolveBranchState,

    pub branches: Vec<Arc<Mutex<SolveBranch>>>,
    pub terminal: Option<StepFailure>,

    pub stats: BasicStatistics,
}

impl SolveBranch {
    pub fn new(
        parent: Weak<Mutex<SolveBranch>>,
        game_state: &GameState,
        decision_edge: VecDeque<DecisionChoice>,
    ) -> SolveBranch {
        SolveBranch {
            parent: parent,
            decision_edge,

            game_state: game_state.clone(),
            state: SolveBranchState::Inited,

            branches: Vec::new(),
            terminal: None,

            stats: BasicStatistics::new(),
        }
    }
}

pub struct SolveEngine {
    init_branch: Arc<Mutex<SolveBranch>>,

    strategy: Box<dyn SolveStrategy>
}

impl SolveEngine {
    pub fn new(init_state: &GameState, strategy: Box<dyn SolveStrategy>) -> SolveEngine {
        SolveEngine {
            init_branch: Arc::new(Mutex::new(SolveBranch::new(Weak::new(), init_state, VecDeque::new()))),

            strategy: strategy,
        }
    }

    pub fn do_branch_execute(&self, branch: &mut SolveBranch) -> Result<(), Box<dyn Error>> {
        loop {
            let mut working_state = branch.game_state.clone();
            let res = working_state.step();

            match res {
                Ok(_) => {
                    branch.game_state = working_state;
                    branch.game_state.advance()?;
                    continue;
                },
                Err(StepFailure::DecisionRequired) => {
                    branch.state = SolveBranchState::Executed;
                    break;
                },
                Err(terminal) => {
                    branch.terminal = Some(terminal);
                    branch.state = SolveBranchState::Completed;
                    break;
                },
            }
        };

        Ok(())
    }

    pub fn do_branch_expand(&self, branch: &mut SolveBranch, parent: Weak<Mutex<SolveBranch>>, choices_so_far: VecDeque<DecisionChoice>) -> Result<(), Box<dyn Error>> {
        let mut working_state = branch.game_state.clone();
        working_state.choices = choices_so_far.clone();
        match working_state.step() {
            Err(StepFailure::DecisionRequired) => { },
            _ => {
                return Err(Box::<dyn std::error::Error>::from("Expand did not begin with a DecisionRequired state."));
            },
        };

        let possible_decisions = self.strategy.decide(&working_state);
        for possible_decision in possible_decisions.into_iter() {
            let mut choices = choices_so_far.clone();
            choices.push_back(possible_decision);
            
            let mut working_state = branch.game_state.clone();
            working_state.choices = choices.clone();

            match working_state.step() {
                Ok(_) => {
                    working_state.advance()?;
                    branch.branches.push(Arc::new(Mutex::new(SolveBranch::new(Weak::clone(&parent), &working_state, choices))));
                }
                Err(StepFailure::DecisionRequired) => {
                    self.do_branch_expand(branch, Weak::clone(&parent), choices)?;
                },
                Err(terminal) => {
                    branch.stats.consume(terminal);
                },
            };
        }

        // only the base case sets the state
        if choices_so_far.len() == 0 {
            branch.state = SolveBranchState::Expanded;
        }

        Ok(())
    }

    pub fn do_branch_finalize(&self, branch: &mut SolveBranch) -> Result<(), Box<dyn Error>> {
        let mut current_stats = branch.stats;

        for sub_branch in branch.branches.iter() {
            self.do_branch(sub_branch)?;

            let sub_branch = sub_branch.lock()
                .or(Err(Box::<dyn std::error::Error>::from("Could not obtain branch lock.")))?;

            if sub_branch.state != SolveBranchState::Finalized {
                return Err(Box::<dyn std::error::Error>::from("Sub branch did not finalize!."));
            }

            current_stats = current_stats.merge(&sub_branch.stats);
        }

        branch.stats = current_stats;
        branch.branches.clear();
        branch.state = SolveBranchState::Finalized;

        Ok(())
    }

    pub fn do_branch(&self, branch: &Arc<Mutex<SolveBranch>>) -> Result<(), Box<dyn Error>> {
        let weak = Arc::downgrade(&branch);
        let mut branch = branch.lock()
            .or(Err(Box::<dyn std::error::Error>::from("Could not obtain branch lock.")))?;

        // We have to move to decision point or finish
        if branch.state == SolveBranchState::Inited {
            self.do_branch_execute(&mut branch)?;
        }

        // We need to expand the state to all possible decisions
        if branch.state == SolveBranchState::Executed {
            self.do_branch_expand(&mut branch, weak, VecDeque::new())?;
        }

        // We need to consume all the child branches
        if branch.state == SolveBranchState::Completed {
            self.do_branch_finalize(&mut branch)?;
        }

        Ok(())
    }

    pub fn recurse_branches(&self, branch: &Arc<Mutex<SolveBranch>>) -> Result<(), Box<dyn Error>> {
        self.do_branch(branch)?;

        {
            let mut branch = branch.lock()
                .or(Err(Box::<dyn std::error::Error>::from("Could not obtain branch lock.")))?;
    
            for sub_branch in branch.branches.iter() {
                self.recurse_branches(sub_branch)?;
            }

            branch.state = SolveBranchState::Completed;
        }
        
        self.do_branch(branch)?;

        Ok(())
    }

    pub fn main(&mut self) -> Result<(), Box<dyn Error>> {
        // TODO use strtaegy to decide on execution order
        // TODO have parallel worker threads
        self.recurse_branches(&self.init_branch)?;

        {
            let branch = self.init_branch.lock()
                .or(Err(Box::<dyn std::error::Error>::from("Could not obtain branch lock.")))?;
    
            println!("v: {}, d: {}, e: {}", branch.stats.victories, branch.stats.defeats, branch.stats.errors);
        }

        Ok(())
    }
}



pub struct SimpleDecisionMaker {

}

impl SimpleDecisionMaker {
    pub fn new() -> Box<SimpleDecisionMaker> {
        Box::new(SimpleDecisionMaker {

        })
    }
}

impl SolveStrategy for SimpleDecisionMaker {
    fn decide(&self, state: &GameState) -> Vec<DecisionChoice> {
        let undecided_decision = state.effect_stack.last().unwrap();
        let decision = undecided_decision.as_decision().unwrap();

        let choices = decision.valid_choices(state);

        choices
    }
}
