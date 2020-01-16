// This file contains copyrighted assets owned by Greater Than Games.

use std::rc::Rc;
use std::collections::VecDeque;
use rand::prelude::*;

use super::concept::{AdversaryDescription, SpiritDescription, Power, Fear, ContentPack, InvaderActionKind};
use super::board::{LandKind, MapDescription, MapState};
use super::step::{GameStep, TurnStep, InvaderStep, InvaderCard, generate_invader_deck};

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

pub struct GameState {
    pub desc: Rc<GameDescription>,

    rng: Box<dyn RngCore>,

    pub step: GameStep,
    pub game_over_reason: Option<String>,

    pub invader: InvaderDeck,

    pub map: MapState,

    pub blight_remaining: u8,

    fears: SimpleDeck<Box<dyn Fear>>,
    fears_pending: Vec<Box<dyn Fear>>,
    fear_pool: u8,
    fear_generated: u8,
    fear_counts: (u8, u8, u8),

    minor_powers: SimpleDeck<Box<dyn Power>>,
    major_powers: SimpleDeck<Box<dyn Power>>,

}

impl GameState {
    pub fn new(desc: Rc<GameDescription>, rng: Box<dyn RngCore>) -> GameState {
        GameState {
            desc: desc.clone(),
            rng,

            step: GameStep::Init,
            game_over_reason: None,

            invader: InvaderDeck::new(),

            map: MapState::new(desc.map.clone()),

            blight_remaining: 5,

            fears: SimpleDeck::new(),
            fears_pending: Vec::new(),
            fear_pool: 0,
            fear_generated: 0,
            fear_counts: (3, 3, 3),

            minor_powers: SimpleDeck::new(),
            major_powers: SimpleDeck::new(),
        }
    }

    pub fn is_over(&self) -> bool {
        match self.step {
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

    pub fn do_add_blight_effect(&mut self, land_index: u8) -> Result<(), ()> {
        println!("   |    `===> blighting land {}.", land_index);

        let mut land = self.map.lands.get_mut(land_index as usize).unwrap();

        // 1. Remove blight from card
        if self.blight_remaining == 0 {
            self.do_defeat("No blight is left.")?;
        }

        self.blight_remaining -= 1;

        // 2. Add blight to the land
        //land.add_blight();

        // 3. Kill presence
        // TODO

        // 4. Check for cascade
        // This is a decision point... ugh
        
        Ok(())
    }

    pub fn do_ravage_effect(&mut self, land_index: u8) -> Result<(), ()> {
        println!("   | `--. > Ravaging in land {}.", land_index);

        let mut land = self.map.lands.get_mut(land_index as usize).unwrap();
        let invader_damage: u16 = land.pieces.iter().map(|p| p.invader_damage()).sum();

        let blight_threshold = 2;

        // TODO intercept and modify this damage:
        // * Adversary manipulations
        // * Spirit manipulations
        // * Powers and other effects
        //   * defense
        //   * modify invader damage
        //   * modify dahan health
        //   * modify blight threshold
        // * ...

        // Damage is done in two steps, one to the land and one to the dahan
        if invader_damage >= blight_threshold {
            self.do_add_blight_effect(land_index)?;
        }

        Ok(())
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

        let desc = self.desc.clone();

        self.step = match step {
            GameStep::Init => {
                let invaders = generate_invader_deck();
                self.invader.set_state(invaders, Vec::new(), self.desc.adversary.invader_steps());
                self.invader.shuffle_draw(&mut self.rng);

                desc.adversary.setup(self);

                GameStep::SetupSpirit
            }
            GameStep::SetupSpirit => {

                GameStep::SetupExplore
            }
            GameStep::SetupExplore => {

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
                            InvaderStep::InvaderAction(inv_action, inv_kind) => {
                                if self.invader.step_count() == inv_action + 1 {
                                    if self.invader.draw.len() == 0 {
                                        self.do_defeat("Invader deck empty!")?;
                                    } else {
                                        self.invader.draw_last_step();
                                    }
                                }

                                let cards = self.invader.pending.get(*inv_action as usize).unwrap().clone();

                                // BaC pg. 14, we go bottom to top
                                for card in cards {
                                    let lands = desc.map.lands_iter().filter(|l| card.can_target(l));

                                    match &inv_kind {
                                        InvaderActionKind::Explore => {
                                            println!("   |-. > Invader Action Card: {}", card);
                                        }
                                        InvaderActionKind::Build => {
                                            println!("   |-. > Invader Action Card: {}", card);
                                        }
                                        InvaderActionKind::Ravage => {
                                            println!("   |-. > Invader Action Card: {}", card);
                                            for land in lands {
                                                self.do_ravage_effect(land.map_index)?;
                                            }
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

        Ok(())
    }
}
