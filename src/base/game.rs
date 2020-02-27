// This file contains copyrighted assets owned by Greater Than Games.

use std::{
    rc::Rc,
    clone::Clone,
    collections::VecDeque,
};

use rand::prelude::*;
use rand_chacha::{ChaChaRng};

use super::*;


/* 
    Our RNG needs to be deterministic and copyable.
*/

pub trait DeterministicRng {
    fn get_rng<'a>(&'a mut self) -> &'a mut dyn RngCore;
    fn box_clone(&self) -> Box<dyn DeterministicRng>;
}

pub struct DeterministicChaCha {
    rng: ChaChaRng
}

impl DeterministicChaCha {
    pub fn new(rng: ChaChaRng) -> Self {
        DeterministicChaCha {
            rng
        }
    }
}

impl DeterministicRng for DeterministicChaCha {
    fn get_rng<'a>(&'a mut self) -> &'a mut dyn RngCore {
        &mut self.rng
    }
    fn box_clone(&self) -> Box<dyn DeterministicRng> {
        Box::new(DeterministicChaCha::new(self.rng.clone()))
    }
}

impl Clone for Box<dyn DeterministicRng> {
    fn clone(&self) -> Self {
        self.box_clone()
    }
}


/*
    In general decks of cards organized as Vecs will follow physical card rules:

    * Push a card means put it on top of the stack.
    * Pop means take off the top of the stack.

    This does however mean that the first card when iterating is the _bottom_ card which might be the last one poped.
*/

pub trait Deck<T> {
    fn shuffle_draw(&mut self, rng: &mut dyn RngCore);
    fn draw(&mut self, count: usize) -> Vec<T>;
}

#[derive(Clone)]
pub struct SimpleDeck<T> {
    draw: Vec<T>,
    discard: Vec<T>,
}

impl<T> SimpleDeck<T> {
    pub fn new() -> Self {
        SimpleDeck::<T> {
            draw: Vec::new(),
            discard: Vec::new(),
        }
    }

    pub fn set_state(&mut self, draw: Vec<T>, discard: Vec<T>) {
        self.draw = draw;
        self.discard = discard;
    }
}

impl<T> Deck<T> for SimpleDeck<T> {
    fn shuffle_draw(&mut self, mut rng: &mut dyn RngCore) {
        self.draw.shuffle(&mut rng);
    }

    fn draw(&mut self, count: usize) -> Vec<T> {
        let mut res = Vec::new();
        for _ in 0..count {
            res.insert(0, self.draw.pop().unwrap());
        }

        res
    }
}

#[derive(Clone)]
pub struct InvaderDeck {
    pub draw: Vec<InvaderCard>,
    discard: Vec<InvaderCard>,
    pending: VecDeque<Vec<InvaderCard>>,
    sequence: Vec<InvaderActionKind>,
}

impl InvaderDeck {
    pub fn new() -> InvaderDeck {
        InvaderDeck {
            draw: Vec::new(),
            discard: Vec::new(),
            pending: VecDeque::new(),
            sequence: Vec::new(),
        }
    }

    pub fn step_count(&self) -> u8 {
        self.sequence.len() as u8
    }

    pub fn get_step_kind(&self, index: u8) -> InvaderActionKind {
        return self.sequence[index as usize];
    }

    pub fn set_state(&mut self, draw: Vec<InvaderCard>, discard: Vec<InvaderCard>, sequence: Vec<InvaderActionKind>) {
        self.draw = draw;
        self.discard = discard;
        self.sequence = sequence;

        for step in self.sequence.iter() { self.pending.push_back(Vec::new()); }
    }

    pub fn draw_into_pending(&mut self) {
        let draw = self.draw(1);
        let card = draw.first().unwrap();

        self.pending.back_mut().unwrap().push(*card);
    }

    pub fn advance(&mut self) {
        self.discard.append(&mut self.pending.pop_front().unwrap());
        self.pending.push_back(Vec::new());
    }
}

impl Deck<InvaderCard> for InvaderDeck {
    fn shuffle_draw(&mut self, mut rng: &mut dyn RngCore) {
        // have to shuffle specific parts
        let partition2 = self.draw.iter().position(|&x| if let InvaderCard::Phase2(_) = x { true } else { false }).unwrap();
        let partition1 = self.draw.iter().position(|&x| if let InvaderCard::Phase1(_) = x { true } else { false }).unwrap();

        assert!(partition2 < partition1);

        self.draw[0..partition2].shuffle(&mut rng);
        self.draw[partition2..partition1].shuffle(&mut rng);
        self.draw[partition1..15].shuffle(&mut rng);
    }

    fn draw(&mut self, count: usize) -> Vec<InvaderCard> {
        let mut res = Vec::new();
        for _ in 0..count {
            res.insert(0, self.draw.pop().unwrap());
        }

        res
    }
}

pub struct GameDescription {
    pub content: Vec<Box<dyn ContentPack>>,
    pub adversary: Box<dyn AdversaryDescription>,
    pub spirits: Vec<Box<dyn SpiritDescription>>,
    pub map: Rc<MapDescription>,
}

impl GameDescription {
    pub fn new(
        content: Vec<Box<dyn ContentPack>>,
        adversary: Box<dyn AdversaryDescription>,
        spirits: Vec<Box<dyn SpiritDescription>>,
        map: Box<MapDescription>,
    ) -> GameDescription {
        GameDescription {
            content,
            adversary,
            spirits,
            map: Rc::from(map)
        }
    }
}

// The state of the game state is invalid
pub enum StepFailure {
    InternalError(String),
    RulesViolation(String),
    GameOverVictory,
    GameOverDefeat,
    DecisionRequired,
    DecisionMismatch,
}

impl From<StepFailure> for Box<dyn std::error::Error> {
    fn from(failure: StepFailure) -> Self {
        match failure {
            StepFailure::GameOverVictory => 
                Box::<dyn std::error::Error>::from("Game Over Victory".to_string()),
            StepFailure::GameOverDefeat =>
                Box::<dyn std::error::Error>::from("Game Over Defeat".to_string()),
            StepFailure::InternalError(msg) =>
                Box::<dyn std::error::Error>::from(format!("Internal: {}", msg)),
            StepFailure::RulesViolation(msg) =>
                Box::<dyn std::error::Error>::from(format!("Rules Violation - {}", msg)),
            StepFailure::DecisionRequired => 
                Box::<dyn std::error::Error>::from("Decision Required".to_string()),
            StepFailure::DecisionMismatch => 
                Box::<dyn std::error::Error>::from("Decision Mismatch".to_string()),
        }
    }
}


#[derive(Clone)]
pub struct GameState {
    pub desc: Rc<GameDescription>,

    rng: Box<dyn DeterministicRng>,

    pub step: GameStep,
    pub next_step: GameStep,
    pub game_over_reason: Option<String>,

    pub choices: VecDeque<DecisionChoice>,
    pub effect_stack: Vec<Box<dyn Effect>>,

    pub invader: InvaderDeck,

    pub map: MapState,

    pub blight_remaining: u8,

    /*
    fears: SimpleDeck<Box<dyn Fear>>,
    fears_pending: Vec<Box<dyn Fear>>,
    fear_pool: u8,
    fear_generated: u8,
    fear_counts: (u8, u8, u8),

    minor_powers: SimpleDeck<Box<dyn Power>>,
    major_powers: SimpleDeck<Box<dyn Power>>,
    */
}

impl GameState {
    pub fn new(desc: Rc<GameDescription>, rng: Box<dyn DeterministicRng>) -> GameState {
        GameState {
            desc: desc.clone(),
            rng,

            step: GameStep::Init,
            next_step: GameStep::Init,
            game_over_reason: None,

            choices: VecDeque::new(),
            effect_stack: Vec::new(),

            invader: InvaderDeck::new(),

            map: MapState::new(desc.map.clone()),

            blight_remaining: 5,

            /*
            fears: SimpleDeck::new(),
            fears_pending: Vec::new(),
            fear_pool: 0,
            fear_generated: 0,
            fear_counts: (3, 3, 3),

            minor_powers: SimpleDeck::new(),
            major_powers: SimpleDeck::new(),
            */
        }
    }

    pub fn is_over(&self) -> bool {
        match self.step {
            GameStep::Victory | GameStep::Defeat => true,
            _ => false,
        }
    }

    pub fn log(&self, s: String) {
        println!("   |{}- {}", "  ".repeat(self.effect_stack.len()), s);
    }

    pub fn do_effect<T : Effect>(&mut self, effect: T) -> Result<(), StepFailure> {
        self.effect_stack.push(effect.box_clone());
        let res = effect.apply_effect(self)?;
        self.effect_stack.pop();

        Ok(res)
    }

    pub fn consume_choice(&mut self) -> Result<DecisionChoice, StepFailure> {
        match self.choices.pop_front() {
            Some(v) => Ok(v),
            None => Err(StepFailure::DecisionRequired)
        }
    }

    pub fn do_defeat(&mut self, defeat_reason: &str) -> Result<(), StepFailure> {
        self.game_over_reason = Some(defeat_reason.to_string());
        self.step = GameStep::Defeat;

        Err(StepFailure::GameOverDefeat)
    }
    
    pub fn do_victory(&mut self, victory_reason: &str) -> Result<(), StepFailure> {
        self.game_over_reason = Some(victory_reason.to_string());
        self.step = GameStep::Victory;

        Err(StepFailure::GameOverVictory)
    }

    pub fn step_to_next_event(&mut self) -> Result<InvaderStep, StepFailure> {
        Ok(self.step_to_next_fear()?)
    }
    pub fn step_to_next_fear(&mut self) -> Result<InvaderStep, StepFailure> {
        let next_card = match &self.step {
            GameStep::Turn(_, TurnStep::Invader(InvaderStep::FearEffect(card))) => *card + 1,
            _ => 0,
        };

        //if (next_card as usize) < self.fears_pending.len() {
        //    Ok(InvaderStep::FearEffect(next_card))
        //}
        //else {
            Ok(self.step_to_next_invader()?)
        //}
    }
    pub fn step_to_next_invader(&mut self) -> Result<InvaderStep, StepFailure> {
        let original_next = match &self.step {
            GameStep::Turn(_, TurnStep::Invader(InvaderStep::InvaderAction(action, part))) => (*action, *part + 1),
            _ => (0, 0),
        };
        let (mut next_action, mut next_part) = original_next;

        // BaC pg. 14, we go bottom to top
        while next_action < self.invader.step_count() {
            if (next_part as usize) < self.invader.pending.get(next_action as usize).unwrap().len() {
                return Ok(InvaderStep::InvaderAction(next_action, next_part));
            }
            else {
                next_action += 1;
                next_part = 0;
            }
        }

        // last action is a draw! the pending will be empty at first
        // so we backtrack here
        if next_action == self.invader.step_count()
            && 0 == self.invader.pending.back().unwrap().len() {
                
            // TODO: (the draw function must account for additional draws! this only happens once!)
            if self.invader.draw.len() == 0 {
                self.do_defeat("Invader deck empty!")?;
            } else {
                self.invader.draw_into_pending();
            }

            return Ok(InvaderStep::InvaderAction(next_action - 1, 0));
        }

        Ok(InvaderStep::InvaderAdvance)
    }

    pub fn step(&mut self) -> Result<(), StepFailure> {
        let step = self.step;
        println!("---+-{:-^70}-----", format!("-  {}  -", step));

        let desc = self.desc.clone();

        self.next_step = match step {
            GameStep::Init => {
                let invaders = generate_invader_deck();
                self.invader.set_state(invaders, Vec::new(), self.desc.adversary.invader_steps());
                self.invader.shuffle_draw(&mut self.rng.get_rng());

                desc.adversary.setup(self);

                GameStep::SetupSpirit
            }
            GameStep::SetupSpirit => {

                GameStep::SetupExplore
            }
            GameStep::SetupExplore => {
                // The initial explore
                self.invader.draw_into_pending();

                let &card = self.invader.pending.back().unwrap().first().unwrap();
                self.log(format!("Invader Action Card: {}", card));

                let lands = desc.map.lands.iter().filter(|l| card.can_target(l));
                for land in lands {
                    self.do_effect(ExploreEffect { land_index: land.map_index })?;
                }

                self.invader.advance();

                // TODO: Post explore adversary setup?

                GameStep::Turn(0, TurnStep::Spirit)
            }
            GameStep::Turn(turn, turn_step) => {
                match &turn_step {
                    TurnStep::Spirit => {

                        GameStep::Turn(turn, TurnStep::FastPower)
                    }
                    TurnStep::FastPower => {

                        GameStep::Turn(turn, TurnStep::Invader(InvaderStep::BlightedIsland))
                    }
                    TurnStep::Invader(inv_step) => {
                        match &inv_step {
                            InvaderStep::BlightedIsland => {

                                GameStep::Turn(turn, TurnStep::Invader(self.step_to_next_event()?))
                            }
                            InvaderStep::Event(event_card, event_part) => {

                                GameStep::Turn(turn, TurnStep::Invader(self.step_to_next_event()?))
                            }
                            InvaderStep::FearEffect(fear_card) => {

                                GameStep::Turn(turn, TurnStep::Invader(self.step_to_next_fear()?))
                            }
                            InvaderStep::InvaderAction(inv_action, inv_card) => {
                                let inv_kind = self.invader.get_step_kind(*inv_action);
                                let &card = self.invader.pending
                                    .get(*inv_action as usize).unwrap()
                                    .get(*inv_card as usize).unwrap();

                                // TODO: Technically the order here is a decision...
                                let lands = desc.map.lands.iter().filter(|l| card.can_target(l));

                                self.log(format!("Invader Action Card: {}", card));
                                match &inv_kind {
                                    InvaderActionKind::Explore => {
                                        for land in lands {
                                            self.do_effect(ExploreEffect { land_index: land.map_index })?;
                                        }
                                    }
                                    InvaderActionKind::Build => {
                                        for land in lands {
                                            self.do_effect(BuildEffect { land_index: land.map_index })?;
                                        }
                                    }
                                    InvaderActionKind::Ravage => {
                                        for land in lands {
                                            self.do_effect(RavageEffect { land_index: land.map_index })?;
                                        }
                                    }
                                }

                                GameStep::Turn(turn, TurnStep::Invader(self.step_to_next_invader()?))
                            }
                            InvaderStep::InvaderAdvance => {
                                self.invader.advance();

                                GameStep::Turn(turn, TurnStep::SlowPower)
                            }
                        }
                    }
                    TurnStep::SlowPower => {

                        GameStep::Turn(turn, TurnStep::TimePasses)
                    }
                    TurnStep::TimePasses => {

                        GameStep::Turn(turn + 1, TurnStep::Spirit)
                    }
                }
            }
            GameStep::Victory => {

                panic!("Cannot step victory state!");
            }
            GameStep::Defeat => {

                panic!("Cannot step defeat state!");
            }
        };

        if self.choices.len() > 0 {
            return Err(StepFailure::InternalError("There are unconsumed choices!".to_string()));
        }

        Ok(())
    }

    pub fn advance(&mut self) -> Result<(), StepFailure> {
        self.step = self.next_step;

        Ok(())
    }
}
