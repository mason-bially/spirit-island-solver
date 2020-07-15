use crate::base::rng::DeterministicRng;
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

use rand::prelude::*;

use crossbeam::{unbounded, atomic::AtomicCell};

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

    fn collect_first_best_game(branch: &SolveBranch, choices: Option<VecDeque<DecisionChoice>>) -> Result<Vec<VecDeque<DecisionChoice>>, Box<dyn Error>> {
        // TODO make this not deadlock (though we currently are try_locking for a reason)
        let mut first_best_game = Vec::new();
        if let Some(choices) = choices {
            first_best_game.push(choices);
        }
        first_best_game.push(branch.decision_edge.clone());
        let mut parent = Weak::clone(&branch.parent);
        while let Some(ptr) = parent.upgrade() {
            first_best_game.push(ptr.decision_edge.clone());
            parent = Weak::clone(&ptr.parent);
        }
        first_best_game.pop(); // the root node is an empty choice
        first_best_game.reverse();
        return Ok(first_best_game);
    }

    pub fn consume(
        &mut self,
        branch: &SolveBranch,
        choices: Option<VecDeque<DecisionChoice>>,
        game_state: &GameState,
        terminal: &StepFailure
    ) -> Result<(), Box<dyn Error>> {
        let do_score;
        match terminal {
            StepFailure::GameOverVictory => {
                do_score = true;
                self.victories += 1;
            },
            StepFailure::GameOverDefeat => {
                do_score = true;
                self.defeats += 1;
            },
            fail => {
                println!("FAILURE: {}", fail); // HACK
                do_score = false;
                self.errors += 1;
            },
        };

        if do_score {
            let score = game_state.score_game();

            if score > self.max_score {
                if let Ok(first_best_game) = BasicStatistics::collect_first_best_game(branch, choices) {
                    self.first_best_game = first_best_game;
                }
            }

            self.min_score = min(self.min_score, score);
            self.max_score = max(self.max_score, score);
        }

        Ok(())
    }
}


#[derive(PartialEq, Eq, Hash, Copy, Clone)]
pub enum SolveBranchState {
    Inited,
    Executed,
    Expanded,
    Pending,
    Completed,
    Finalized,
}


struct SolveBranchInternal {
    pub game_state: GameState,

    pub branches: Vec<Arc<SolveBranch>>,
    pub terminal: Option<StepFailure>,

    pub stats: BasicStatistics,

    pub pending_children: usize,
    pub finalized_children: usize,
}

pub struct SolveBranch {
    pub parent: Weak<SolveBranch>,
    pub decision_edge: VecDeque<DecisionChoice>,

    // Only store to this if you hold the below mutex!!
    state: AtomicCell<SolveBranchState>,
    
    internal: Mutex<SolveBranchInternal>,
}

impl SolveBranch {
    pub fn new(
        parent: Weak<SolveBranch>,
        game_state: GameState,
        decision_edge: VecDeque<DecisionChoice>,
    ) -> SolveBranch {
        SolveBranch {
            parent: parent,
            decision_edge,

            state: AtomicCell::new(SolveBranchState::Inited),

            internal: Mutex::new(SolveBranchInternal {
                game_state: game_state,
    
                branches: Vec::new(),
                terminal: None,
    
                stats: BasicStatistics::new(),
    
                pending_children: 0,
                finalized_children: 0,
            })
        }
    }
}

struct SolveEngineShared {
    pub init_branch: Arc<SolveBranch>,
    pub strategy: Box<dyn SolveStrategy>,

    pub branches: AtomicUsize,
    pub steps: AtomicUsize,
    pub branches_finalized: AtomicUsize,

    pub last_update: Mutex<Instant>,
}

impl SolveEngineShared {
    
    pub fn do_branch_execute(&self, branch: &SolveBranch, branch_internal: &mut SolveBranchInternal) -> Result<(), Box<dyn Error>> {
        loop {
            let mut working_state = branch_internal.game_state.clone();
            //working_state.enable_logging = true; // HACK
            let res = working_state.step();
            self.steps.fetch_add(1, Ordering::Relaxed);

            match res {
                Ok(_) => {
                    branch_internal.game_state = working_state;
                    branch_internal.game_state.advance()?;
                    continue;
                },
                Err(StepFailure::DecisionRequired) => {
                    branch.state.store(SolveBranchState::Executed);
                    break;
                },
                Err(terminal) => {
                    branch_internal.stats.consume(branch, None, &working_state, &terminal)?;
                    branch_internal.terminal = Some(terminal);
                    branch.state.store(SolveBranchState::Completed);
                    break;
                },
            }
        };

        Ok(())
    }

    pub fn do_branch_expand(&self, branch: &Arc<SolveBranch>, branch_internal: &mut SolveBranchInternal, choices_so_far: VecDeque<DecisionChoice>) -> Result<(), Box<dyn Error>> {
        let mut working_state = branch_internal.game_state.clone();
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
            
            let mut working_state = branch_internal.game_state.clone();
            working_state.choices = choices.clone();

            match working_state.step() {
                Ok(_) => {
                    working_state.advance()?;

                    let new_branch = SolveBranch::new(Arc::downgrade(branch), working_state, choices);

                    // agressively prune these branches
                    let keep_branch
                        = {
                            let mut new_branch_internal = new_branch.internal.lock()
                                .or(Err(Box::<dyn std::error::Error>::from("Could not obtain branch lock.")))?;
                            self.do_branch_execute(&new_branch, &mut new_branch_internal)?;
        
                            if new_branch.state.load() == SolveBranchState::Completed {
                                branch_internal.stats.merge(&new_branch_internal.stats);

                                false
                            } else {
                                let prev = self.branches.fetch_add(1, Ordering::Relaxed);
                                if prev % 100000 == 0 {
                                    let mut last_update = self.last_update.lock()
                                        .or(Err(Box::<dyn std::error::Error>::from("Could not obtain branch lock.")))?;
        
                                    let seconds = last_update.elapsed().as_secs_f64();
                                    *last_update = Instant::now();
                                    let steps = self.steps.swap(0, Ordering::Relaxed);
                                    let steps_per_second = (steps as f64) / seconds;
                                    println!("{} branches ({:.0} steps/s). at {}.", prev, steps_per_second, new_branch_internal.game_state.step);
                                }

                                true
                            }
                        };

                    if keep_branch {
                        branch_internal.branches.push(Arc::new(new_branch));
                    }
                }
                Err(StepFailure::DecisionRequired) => {
                    self.do_branch_expand(branch, branch_internal, choices)?;
                },
                Err(terminal) => {
                    branch_internal.stats.consume(branch, Some(choices), &working_state, &terminal)?;
                },
            };
        }

        // only the base case sets the state
        if choices_so_far.len() == 0 {
            if branch_internal.branches.len() != 0 {
                branch.state.store(SolveBranchState::Expanded);
            } else {
                branch.state.store(SolveBranchState::Completed);
            }
        }

        Ok(())
    }

    pub fn do_branch_finalize(&self, branch: &Arc<SolveBranch>, branch_internal: &mut SolveBranchInternal) -> Result<(), Box<dyn Error>> {
        let current_stats = &mut branch_internal.stats;

        for sub_branch in branch_internal.branches.iter() {
            if sub_branch.state.load() != SolveBranchState::Finalized {
                self.do_branch(&sub_branch)?;

                if sub_branch.state.load() != SolveBranchState::Finalized {
                    return Err(Box::<dyn std::error::Error>::from("Sub branch did not finalize!."));
                }
            }

            let sub_branch_internal = sub_branch.internal.lock()
                .or(Err(Box::<dyn std::error::Error>::from("Could not obtain branch lock.")))?;

            current_stats.merge(&sub_branch_internal.stats);
        }

        branch_internal.branches.clear();
        let prev = self.branches_finalized.fetch_add(1, Ordering::Relaxed);
        if prev % 100000 == 0 {
            println!("{} branches finalized.", prev);
        }

        branch.state.store(SolveBranchState::Finalized);

        Ok(())
    }

    pub fn do_branch(&self, branch: &Arc<SolveBranch>) -> Result<SolveBranchState, Box<dyn Error>> {
        let mut branch_internal = branch.internal.lock()
            .or(Err(Box::<dyn std::error::Error>::from("Could not obtain branch lock.")))?;

        // We have to move to decision point or finish
        if branch.state.load() == SolveBranchState::Inited {
            self.do_branch_execute(&branch, &mut branch_internal)?;
        }

        // We need to expand the state to all possible decisions
        if branch.state.load() == SolveBranchState::Executed {
            self.do_branch_expand(&branch, &mut branch_internal, VecDeque::new())?;
        }

        // We need to check all sub states
        let state = branch.state.load();
        if state == SolveBranchState::Expanded
            || state == SolveBranchState::Pending {
            branch_internal.finalized_children = branch_internal.branches.iter().filter(|sb| sb.state.load() == SolveBranchState::Finalized).count();

            if branch_internal.pending_children == branch_internal.finalized_children {
                branch.state.store(SolveBranchState::Completed);
            }
        }

        // We need to consume all the child branches
        if branch.state.load() == SolveBranchState::Completed
            && branch_internal.finalized_children == branch_internal.branches.len() {
            self.do_branch_finalize(&branch, &mut branch_internal)?;
        }

        Ok(branch.state.load())
    }

    pub fn get_branch_partial_expand(&self, branch: &Arc<SolveBranch>, more: usize) -> Result<Vec<Arc<SolveBranch>>, Box<dyn Error>> {
        let mut branch_internal = branch.internal.lock()
            .or(Err(Box::<dyn std::error::Error>::from("Could not obtain branch lock.")))
            .ok().unwrap();

        let orig_pending = branch_internal.pending_children;
        branch_internal.pending_children = min(orig_pending + more, branch_internal.branches.len());

        if orig_pending != branch_internal.pending_children {
            branch.state.store(SolveBranchState::Pending);
        } else {
            branch.state.store(SolveBranchState::Completed);
        }

        Ok(branch_internal.branches[orig_pending..branch_internal.pending_children].iter().map(Arc::clone).collect())
    }
}


pub struct SolveEngine {
    init_state: GameState,

    shared: Arc<SolveEngineShared>,
}

enum SolveWork {
    Execute(Arc<SolveBranch>),
    Finalize(Arc<SolveBranch>),
    Terminate,
}

impl SolveEngine {
    pub fn new(init_state: &GameState, strategy: Box<dyn SolveStrategy>) -> SolveEngine {
        SolveEngine {
            init_state: init_state.clone(),

            shared: Arc::new(SolveEngineShared {
                strategy: strategy,
                init_branch: Arc::new(SolveBranch::new(Weak::new(), init_state.clone(), VecDeque::new())),

                steps: AtomicUsize::new(0),
                branches: AtomicUsize::new(1),
                branches_finalized: AtomicUsize::new(0),
    
                last_update: Mutex::new(Instant::now()),
            }),
        }
    }

    // single threaded recursive
    pub fn recurse_branches(&self, branch: &Arc<SolveBranch>) -> Result<(), Box<dyn Error>> {
        self.shared.do_branch(branch)?;

        // We do it this way to prevent recursive downlocking
        let sub_branches;
        {
            let branch_internal = branch.internal.lock()
                .or(Err(Box::<dyn std::error::Error>::from("Could not obtain branch lock.")))?;
    
            sub_branches = branch_internal.branches.clone();
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
                                let res = shared.do_branch(&branch).unwrap();

                                if res != SolveBranchState::Finalized {
                                    for sub_branch in shared.get_branch_partial_expand(&branch, 3).ok().unwrap() {
                                        sender.send(SolveWork::Execute(sub_branch)).unwrap();
                                    }
                                }

                                sender.send(SolveWork::Finalize(branch)).unwrap();
                            }
                            SolveWork::Finalize(branch) => {
                                // TODO attempt reschedule according to strategy
                                let res = shared.do_branch(&branch).unwrap();

                                if res != SolveBranchState::Finalized {
                                    if res == SolveBranchState::Completed {
                                        for sub_branch in shared.get_branch_partial_expand(&branch, 3).ok().unwrap() {
                                            sender.send(SolveWork::Execute(sub_branch)).unwrap();
                                        }
                                    }

                                    sender.send(SolveWork::Finalize(branch)).unwrap();
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

                //self.recurse_branches(&self.shared.init_branch)?;
            }
            let elapsed = start.elapsed();
            println!("Elapsed: {:.2}s", elapsed.as_secs_f64());
        }

        {
            let branch_internal = self.shared.init_branch.internal.lock()
                .or(Err(Box::<dyn std::error::Error>::from("Could not obtain branch lock.")))?;
            let stats = &branch_internal.stats;

            let res = self.resimulate_game(stats.first_best_game.clone());
    
            println!("");
            println!(" ^^  ^^  ^^  ^^  ^^  ^^  ^^  ^^  ^^  ^^  ^^  ^^  ^^  ^^  ^^  ^^  ^^  ^^  ^^ ");
            println!("  first best game replay above    ({} branches, {} choices)",
                stats.first_best_game.len(), stats.first_best_game.iter().map(|s| s.len()).sum::<usize>());
            println!("");
            println!("  v: {},  d: {},  e: {}", stats.victories, stats.defeats, stats.errors);
            println!("    min: {},  max: {}  ", stats.min_score, stats.max_score);
            println!("");

            res
        }
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

        let descisions = decision.valid_choices(state);
        if descisions.len() == 0 {
            Vec::new()
        } else {
            vec![descisions[0].clone()]
        }
    }
}



pub struct StochasticDecisionMaker {
    pub rng: Box<dyn DeterministicRng>,
}

impl StochasticDecisionMaker {
    pub fn new(rng: Box<dyn DeterministicRng>) -> Box<StochasticDecisionMaker> {
        Box::new(StochasticDecisionMaker {
            rng
        })
    }
}

impl SolveStrategy for StochasticDecisionMaker {
    fn decide(&self, state: &GameState) -> Vec<DecisionChoice> {
        let undecided_decision = state.effect_stack.last().unwrap();
        let decision = undecided_decision.as_decision().unwrap();

        let previous_decisions = state.choice_count;

        let mut temp_rng = self.rng.clone();
        for _ in 0..previous_decisions
        {
            temp_rng.get_rng().next_u64();
        }

        let mut descisions = decision.valid_choices(state).clone();
        descisions.shuffle(&mut temp_rng.get_rng());

        if descisions.len() == 0 {
            Vec::new()
        } else {
            vec![descisions[0].clone()]
        }
    }
}
