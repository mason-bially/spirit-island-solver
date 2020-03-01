// This file contains copyrighted assets owned by Greater Than Games.

use std::{
    rc::Rc,
    clone::Clone,
    any::Any,
    fmt,
    iter::*,
};

use rand::prelude::*;

use crate::base::{
    board::{LandState},
    concept::{PowerSpeed},
    effect::{Effect, SubEffect},
    spirit::{ElementMap, SpiritState},

    step::{StepFailure},
    game::{GameState},
};


#[derive(PartialEq, Eq, Hash, Copy, Clone)]
pub enum PowerCardKind {
    Minor,
    Major,
    Spirit(u8),
}

impl fmt::Display for PowerCardKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            PowerCardKind::Minor => write!(f, "Minor "),
            PowerCardKind::Major => write!(f, "Major "),
            PowerCardKind::Spirit(_) => write!(f, "Spirit"),
       }
    }
}

#[derive(Copy, Clone)]
pub enum PowerTargetFilter {
    None,
    Spirit(fn(&SpiritState) -> bool),
    Land(fn(&LandState) -> bool),
}

#[derive(PartialEq, Eq, Hash, Copy, Clone)]
pub enum PowerTarget {
    Spirit(u8),
    Land(u8),
}

#[derive(PartialEq, Eq, Hash, Copy, Clone)]
pub struct PowerUsage {
    pub target: PowerTarget,
    pub using_spirit_index: u8,
    pub src_land_index: Option<u8>,
}

impl PowerUsage {
    pub fn target_land(&self) -> Result<u8, StepFailure> {
        match self.target {
            PowerTarget::Land(land_index) => Ok(land_index),
            _ => Err(StepFailure::RulesViolation("Power must target a land.".to_string())),
        }
    }
    pub fn target_spirit(&self) -> Result<u8, StepFailure> {
        match self.target {
            PowerTarget::Spirit(spirit_index) => Ok(spirit_index),
            _ => Err(StepFailure::RulesViolation("Power must target a spirit.".to_string())),
        }
    }
}


#[derive(Clone)]
pub struct PowerCardDescription {
    pub name: &'static str,
    
    pub kind: PowerCardKind,
    pub elements: ElementMap<bool>,

    pub cost: u8,
    pub speed: PowerSpeed,
    pub range: Option<u8>,
    pub target_filter: PowerTargetFilter,

    pub effect: SubEffect,
}

impl fmt::Display for PowerCardDescription {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {} - {}", self.elements, self.kind, self.name)
    }
}

impl Effect for PowerCardDescription {
    fn apply_effect(&self, game: &mut GameState) -> Result<(), StepFailure> {
        game.log(format!("playing power card |{}|", self));

        // actually run the effect as "ourself"
        (self.effect)(game)
    }

    fn box_clone(&self) -> Box<dyn Effect> { Box::new(self.clone()) }
    fn as_any(&self) -> Box<dyn Any> { Box::new(self.clone()) }
}


#[derive(Clone)]
pub struct PowerCard {
    pub desc: Rc<PowerCardDescription>,
    pub index: usize,
}


#[derive(Clone)]
pub struct PowerDeck {
    pub draw: Vec<PowerCard>,
    pub discard: Vec<PowerCard>,
}

impl PowerDeck {
    pub fn new() -> PowerDeck {
        PowerDeck {
            draw: Vec::new(),
            discard: Vec::new(),
        }
    }

    pub fn init(&mut self, 
            desc: Vec<Rc<PowerCardDescription>>,
            mut rng: &mut dyn RngCore) {
        self.draw
            = desc.into_iter()
                .enumerate()
                .map(|(index, desc)| PowerCard{ index, desc })
                .collect();
        self.draw.shuffle(&mut rng);
    }

    pub fn draw(&mut self, count: usize) -> Vec<PowerCard> {
        let mut res = Vec::new();
        for _ in 0..count {
            res.insert(0, self.draw.pop().unwrap());
        }

        res
    }
}


#[derive(Clone)]
pub struct SpiritPowerDeck {
    pub draw: Vec<PowerCard>,
    pub hand: Vec<PowerCard>,
    pub pending: Vec<PowerCard>,
    pub discard: Vec<PowerCard>,
    pub forgotten: Vec<PowerCard>,
}

impl SpiritPowerDeck {
    pub fn new() -> SpiritPowerDeck {
        SpiritPowerDeck {
            draw: Vec::new(),
            hand: Vec::new(),
            pending: Vec::new(),
            discard: Vec::new(),
            forgotten: Vec::new(),
        }
    }

    pub fn init(&mut self, 
            desc: Vec<Rc<PowerCardDescription>>) {
        self.hand
            = desc.into_iter()
                .enumerate()
                .map(|(index, desc)| PowerCard{ index, desc })
                .collect();
    }
}
