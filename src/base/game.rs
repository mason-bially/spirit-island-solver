use std::collections::VecDeque;
use rand::prelude::*;

use super::concept::{Adversary, Spirit, Power, Fear, ContentPack, InvaderActionKind};
use super::board::{LandKind};
use super::step::{GameStep, TurnStep, InvaderStep, InvaderCard, generate_invader_deck};

pub trait Deck<T> {
    fn shuffle_draw(&mut self, rng: &mut dyn RngCore);
    fn draw(&mut self, count: usize) -> Vec<T>;
}

pub struct SimpleDeck<T> {
    draw: Vec<T>,
    discard: Vec<T>,
}

impl<T> SimpleDeck<T> {
    pub fn new() -> SimpleDeck<T> {
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

pub struct InvaderDeck {
    draw: Vec<InvaderCard>,
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

    pub fn draw_last_step(&mut self) {
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
    // have to shuffle specific parts
    fn shuffle_draw(&mut self, mut rng: &mut dyn RngCore) {
        self.draw.shuffle(&mut rng);
    }

    fn draw(&mut self, count: usize) -> Vec<InvaderCard> {
        let mut res = Vec::new();
        for _ in 0..count {
            res.insert(0, self.draw.pop().unwrap());
        }

        res
    }
}

pub struct GameState {
    rng: Box<dyn RngCore>,
    content: Vec<Box<dyn ContentPack>>,
    adversary: Box<dyn Adversary>,
    spirits: Vec<Box<dyn Spirit>>,

    minor_powers: SimpleDeck<Box<dyn Power>>,
    major_powers: SimpleDeck<Box<dyn Power>>,

    fears: SimpleDeck<Box<dyn Fear>>,
    fears_pending: Vec<Box<dyn Fear>>,
    fear_pool: u8,
    fear_generated: u8,
    fear_counts: (u8, u8, u8),

    invader: InvaderDeck,

    pub step: GameStep,
    pub game_over_reason: Option<String>,
}

impl GameState {
    pub fn new(rng: Box<dyn RngCore>, 
                content: Vec<Box<dyn ContentPack>>,
                adversary: Box<dyn Adversary>,
                spirits: Vec<Box<dyn Spirit>>
            ) -> GameState {
        GameState {
            rng,
            content,
            adversary,
            spirits,

            minor_powers: SimpleDeck::new(),
            major_powers: SimpleDeck::new(),

            fears: SimpleDeck::new(),
            fears_pending: Vec::new(),
            fear_pool: 0,
            fear_generated: 0,
            fear_counts: (3, 3, 3),

            invader: InvaderDeck::new(),

            step: GameStep::Init,
            game_over_reason: None,
        }
    }

    pub fn is_over(&self) -> bool {
        match &self.step {
            GameStep::Victory | GameStep::Defeat => true,
            _ => false,
        }
    }

    pub fn do_defeat(&mut self, defeat_reason: &str) -> Result<(), ()> {
        self.game_over_reason = Some(defeat_reason.to_string());
        self.step = GameStep::Defeat;

        Err(())
    }
    
    pub fn do_victory(&mut self, victory_reason: &str) -> Result<(), ()> {
        self.game_over_reason = Some(victory_reason.to_string());
        self.step = GameStep::Victory;

        Err(())
    }

    pub fn step_until_decision(&mut self) -> Result<(), ()> {
        while !self.is_over() {
            self.step()?;
        }

        Ok(())
    }

    pub fn step_to_next_event(&self) -> Result<InvaderStep, ()> {
        Ok(self.step_to_next_fear()?)
    }
    pub fn step_to_next_fear(&self) -> Result<InvaderStep, ()> {
        let next_card = match &self.step {
            GameStep::Turn(_, TurnStep::Invader(InvaderStep::FearEffect(card))) => *card + 1,
            _ => 0,
        };

        if (next_card as usize) < self.fears_pending.len() {
            Ok(InvaderStep::FearEffect(next_card))
        }
        else {
            Ok(self.step_to_next_invader()?)
        }
    }
    pub fn step_to_next_invader(&self) -> Result<InvaderStep, ()> {
        let next_action = match &self.step {
            GameStep::Turn(_, TurnStep::Invader(InvaderStep::InvaderAction(action, _))) => *action + 1,
            _ => 0,
        };

        if next_action < self.invader.step_count() {
            Ok(InvaderStep::InvaderAction(next_action, self.invader.get_step_kind(next_action)))
        }
        else {
            Ok(InvaderStep::InvaderAdvance)
        }
    }

    pub fn step(&mut self) -> Result<(), ()> {
        let step = self.step;
        println!("---+-{:-^70}-----", format!("-  {}  -", step));

        self.step = match step {
            GameStep::Init => {
                let invaders = generate_invader_deck();
                self.invader.set_state(invaders, Vec::new(), self.adversary.invader_steps());
                self.invader.shuffle_draw(&mut self.rng);

                GameStep::SetupSpirit
            }
            GameStep::SetupSpirit => {

                GameStep::SetupExplore
            }
            GameStep::SetupExplore => {

                GameStep::Turn(0, TurnStep::Spirit)
            }
            GameStep::Turn(turn, turn_Step) => {
                match &turn_Step {
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
                            InvaderStep::InvaderAction(inv_action, inv_kind) => {
                                if self.invader.step_count() == inv_action + 1 {
                                    if self.invader.draw.len() == 0 {
                                        self.do_defeat("Invader deck empty!")?;
                                    } else {
                                        self.invader.draw_last_step();
                                    }
                                }

                                match &inv_kind {
                                    InvaderActionKind::Explore => {

                                    }
                                    InvaderActionKind::Ravage => {
                                        
                                    }
                                    InvaderActionKind::Build => {
                                        
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

        Ok(())
    }
}
