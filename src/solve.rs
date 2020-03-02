use std::{
    error::Error,
    cmp::*,
    thread,
    sync::{
        Arc, Weak, Mutex,
        atomic::{Ordering, AtomicUsize},
    },
    collections::{VecDeque},
    time::Instant,
};

use crossbeam::unbounded;

use crate::base::{GameState, StepFailure, DecisionChoice};


pub trait SolveStrategy : Send + Sync {
    // The list of decisions to try in order
    fn decide(&self, state: &GameState) -> Vec<DecisionChoice>;
}


#[derive(Clone)]
pub struct BasicStatistics {
    pub victories: usize,
    pub defeats: usize,
    pub errors: usize,

    pub min_score: i16,
    pub max_score: i16,

    pub first_best_game: Vec<VecDeque<DecisionChoice>>,
}

impl BasicStatistics {
    pub fn new() -> BasicStatistics {
        BasicStatistics {
            victories: 0,
            defeats: 0,
            errors: 0,

            min_score: std::i16::MAX,
            max_score: std::i16::MIN,

            first_best_game: Vec::new(),
        }
    }

    pub fn merge(&mut self, other: &BasicStatistics) {
        self.victories += other.victories;
        self.defeats += other.defeats;
        self.errors += other.errors;

        if other.max_score > self.max_score && other.first_best_game.len() != 0 {
            self.first_best_game = other.first_best_game.clone();
        }

        self.min_score = min(self.min_score, other.min_score);
        self.max_score = max(self.max_score, other.max_score);
    }

    fn collect_first_best_game(branch: &mut SolveBranch, choices: VecDeque<DecisionChoice>) -> Result<Vec<VecDeque<DecisionChoice>>, Box<dyn Error>> {
        // TODO make this not deadlock (though we currently are try_locking for a reason)
        let mut first_best_game = Vec::new();
        first_best_game.push(choices);
        first_best_game.push(branch.decision_edge.clone());
        let mut parent = Weak::clone(&branch.parent);
        while let Some(ptr) = parent.upgrade() {
            let ptr = ptr.try_lock()
                .or(Err(Box::<dyn std::error::Error>::from("Could not obtain parent lock.")))?;
            first_best_game.push(ptr.decision_edge.clone());
            parent = Weak::clone(&ptr.parent);
        }
        first_best_game.pop(); // the root node is an empty choice
        first_best_game.reverse();
        return Ok(first_best_game);
    }

    pub fn consume(branch: &mut SolveBranch, choices: VecDeque<DecisionChoice>, game_state: &GameState, terminal: StepFailure) -> Result<(), Box<dyn Error>> {
        let do_score;
        match terminal {
            StepFailure::GameOverVictory => {
                do_score = true;
                branch.stats.victories += 1;
            },
            StepFailure::GameOverDefeat => {
                do_score = true;
                branch.stats.defeats += 1;
            },
            fail => {
                println!("FAILURE: {}", fail); // HACK
                do_score = false;
                branch.stats.errors += 1;
            },
        };

        if do_score {
            let score = game_state.score_game();

            if score > branch.stats.max_score {
                if let Ok(first_best_game) = BasicStatistics::collect_first_best_game(branch, choices) {
                    branch.stats.first_best_game = first_best_game;
                }
            }

            branch.stats.min_score = min(branch.stats.min_score, score);
            branch.stats.max_score = max(branch.stats.max_score, score);
        }

        Ok(())
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
        game_state: GameState,
        decision_edge: VecDeque<DecisionChoice>,
    ) -> SolveBranch {
        SolveBranch {
            parent: parent,
            decision_edge,

            game_state: game_state,
            state: SolveBranchState::Inited,

            branches: Vec::new(),
            terminal: None,

            stats: BasicStatistics::new(),
        }
    }
}

struct SolveEngineShared {
    pub init_branch: Arc<Mutex<SolveBranch>>,
    pub strategy: Box<dyn SolveStrategy>,

    pub branches: AtomicUsize,
    pub steps: AtomicUsize,

    pub last_update: Mutex<Instant>,
}

impl SolveEngineShared {
    
    pub fn do_branch_execute(&self, branch: &mut SolveBranch) -> Result<(), Box<dyn Error>> {
        loop {
            let mut working_state = branch.game_state.clone();
            //working_state.enable_logging = true; // HACK
            let res = working_state.step();
            self.steps.fetch_add(1, Ordering::Relaxed);

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
                    let prev = self.branches.fetch_add(1, Ordering::Relaxed);
                    if prev % 10000 == 0 {
                        let mut last_update = self.last_update.lock()
                            .or(Err(Box::<dyn std::error::Error>::from("Could not obtain branch lock.")))?;

                        let seconds = last_update.elapsed().as_secs_f64();
                        *last_update = Instant::now();
                        let steps = self.steps.swap(0, Ordering::Relaxed);
                        let steps_per_second = (steps as f64) / seconds;
                        println!("passing {} branches ({:.0} steps/s). at {}.", prev, steps_per_second, working_state.step);

                    }
                    branch.branches.push(Arc::new(Mutex::new(SolveBranch::new(Weak::clone(&parent), working_state, choices))));
                }
                Err(StepFailure::DecisionRequired) => {
                    self.do_branch_expand(branch, Weak::clone(&parent), choices)?;
                },
                Err(terminal) => {
                    BasicStatistics::consume(branch, choices, &working_state, terminal)?;
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
        let current_stats = &mut branch.stats;

        for sub_branch in branch.branches.iter() {
            self.do_branch(&sub_branch)?;

            let sub_branch = sub_branch.lock()
                .or(Err(Box::<dyn std::error::Error>::from("Could not obtain branch lock.")))?;

            if sub_branch.state != SolveBranchState::Finalized {
                return Err(Box::<dyn std::error::Error>::from("Sub branch did not finalize!."));
            }

            current_stats.merge(&sub_branch.stats);
        }

        branch.branches.clear();
        branch.state = SolveBranchState::Finalized;

        Ok(())
    }

    pub fn do_branch(&self, branch: &Arc<Mutex<SolveBranch>>) -> Result<SolveBranchState, Box<dyn Error>> {
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

        // We need to check all sub states
        if branch.state == SolveBranchState::Expanded {
            let mut sub_complete = true;

            for sub_branch in branch.branches.iter() {
                let sub_branch = sub_branch.lock()
                    .or(Err(Box::<dyn std::error::Error>::from("Could not obtain sub-branch lock.")))?;

                if sub_branch.state != SolveBranchState::Finalized {
                    sub_complete = false;
                    break;
                }
            }

            if sub_complete {
                branch.state = SolveBranchState::Completed;
            }
        }

        // We need to consume all the child branches
        if branch.state == SolveBranchState::Completed {
            self.do_branch_finalize(&mut branch)?;
        }

        Ok(branch.state)
    }

}


pub struct SolveEngine {
    init_state: GameState,

    shared: Arc<SolveEngineShared>,
}

enum SolveWork {
    Execute(Arc<Mutex<SolveBranch>>),
    Finalize(Arc<Mutex<SolveBranch>>),
    Terminate,
}

impl SolveEngine {
    pub fn new(init_state: &GameState, strategy: Box<dyn SolveStrategy>) -> SolveEngine {
        SolveEngine {
            init_state: init_state.clone(),

            shared: Arc::new(SolveEngineShared {
                strategy: strategy,
                init_branch: Arc::new(Mutex::new(SolveBranch::new(Weak::new(), init_state.clone(), VecDeque::new()))),

                steps: AtomicUsize::new(0),
                branches: AtomicUsize::new(1),
    
                last_update: Mutex::new(Instant::now()),
            }),
        }
    }

    // single threaded recursive
    pub fn recurse_branches(&self, branch: &Arc<Mutex<SolveBranch>>) -> Result<(), Box<dyn Error>> {
        self.shared.do_branch(branch)?;

        // We do it this way to prevent recursive downlocking
        let sub_branches;
        {
            let branch = branch.lock()
                .or(Err(Box::<dyn std::error::Error>::from("Could not obtain branch lock.")))?;
    
            sub_branches = branch.branches.clone();
        }

        for sub_branch in sub_branches.iter() {
            self.recurse_branches(sub_branch)?;
        }
        
        self.shared.do_branch(branch)?;

        Ok(())
    }

    pub fn resimulate_game(&self, mut choices: Vec<VecDeque<DecisionChoice>>) -> Result<(), Box<dyn Error>> {
        choices.reverse();

        let mut current_state = self.init_state.clone();
        current_state.enable_logging = true;

        let mut pull_decision = false;
        loop {
            let mut working_state = current_state.clone();
            if pull_decision {
                pull_decision = false;
                // TODO fix my fucking errors
                working_state.choices = choices.pop().unwrap();
            }

            let res = working_state.step();

            match res {
                Ok(_) => {
                    current_state = working_state;
                    current_state.advance()?;
                    continue;
                },
                Err(StepFailure::DecisionRequired) => {
                    pull_decision = true;
                    continue;
                },
                Err(StepFailure::GameOverVictory) => {
                    println!("Victory!    {}", working_state.game_over_reason.as_ref().unwrap());
                    return Ok(());
                }
                Err(StepFailure::GameOverDefeat) => {
                    println!("Defeat :(   {}", working_state.game_over_reason.as_ref().unwrap());
                    return Ok(());
                }
                Err(fail) => {
                    return Err(Box::<dyn std::error::Error>::from(fail));
                }
            }
        };
    }

    pub fn main(&mut self, threads: usize) -> Result<(), Box<dyn Error>> {
        // TODO use strtaegy to decide on execution order
        // TODO have parallel worker threads

        {
            let (sender, reciever) = unbounded::<SolveWork>();
    
            // TODO proper Result<> handling
            let mut thread_handles = Vec::new();
            for _id in 0..threads {
                let shared = Arc::clone(&self.shared);
                let (sender, reciever) = (sender.clone(), reciever.clone());
                thread_handles.push(thread::spawn(move || {
                    while let Ok(work) = reciever.recv() {
                        match work {
                            SolveWork::Execute(branch) => {
                                shared.do_branch(&branch).unwrap();

                                let sub_branches;
                                {
                                    let branch = branch.lock()
                                        .or(Err(Box::<dyn std::error::Error>::from("Could not obtain branch lock.")))
                                        .ok().unwrap();
                            
                                    sub_branches = branch.branches.clone();
                                }

                                for sub_branch in sub_branches {
                                    sender.send(SolveWork::Execute(sub_branch)).unwrap();
                                }

                                sender.send(SolveWork::Finalize(branch)).unwrap();
                            }
                            SolveWork::Finalize(branch) => {
                                // TODO attempt reschedule according to strategy
                                let res = shared.do_branch(&branch).unwrap();

                                if res != SolveBranchState::Finalized {
                                    sender.send(SolveWork::Execute(branch)).unwrap();
                                } else if Arc::ptr_eq(&branch, &shared.init_branch) {
                                    // This is a finalized root:
                                    for _ in 0..threads {
                                        // TODO use a Mutex or Signal or something
                                        sender.send(SolveWork::Terminate).unwrap();
                                    }
                                }
                            }
                            SolveWork::Terminate => {
                                break;
                            }
                        }
                    }
                }));
            }
    
            let start = Instant::now();
            {
                sender.send(SolveWork::Execute(Arc::clone(&self.shared.init_branch)));

                for thread_handle in thread_handles {
                    thread_handle.join();
                }
            }
            let elapsed = start.elapsed();
            println!("Elapsed: {:.2}s", elapsed.as_secs_f64());
        }

        {
            let branch = self.shared.init_branch.lock()
                .or(Err(Box::<dyn std::error::Error>::from("Could not obtain branch lock.")))?;

            self.resimulate_game(branch.stats.first_best_game.clone())?;
    
            println!("");
            println!(" ^^  ^^  ^^  ^^  ^^  ^^  ^^  ^^  ^^  ^^  ^^  ^^  ^^  ^^  ^^  ^^  ^^  ^^  ^^ ");
            println!("  first best game replay above    ({} branches, {} choices)",
                branch.stats.first_best_game.len(), branch.stats.first_best_game.iter().map(|s| s.len()).sum::<usize>());
            println!("");
            println!("  v: {},  d: {},  e: {}", branch.stats.victories, branch.stats.defeats, branch.stats.errors);
            println!("    min: {},  max: {}  ", branch.stats.min_score, branch.stats.max_score);
            println!("");
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

        decision.valid_choices(state)
    }
}
