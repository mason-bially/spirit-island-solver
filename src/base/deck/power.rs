// This file contains copyrighted assets owned by Greater Than Games.

use std::{
    rc::Rc,
    clone::Clone,
    iter::*,
};

use rand::prelude::*;

use crate::base::{
    board::{LandState},
    concept::{PowerSpeed},
    effect::{Effect},
    spirit::{ElementMap, SpiritState},

    step::{StepFailure},
    game::{GameState},
};


#[derive(Copy, Clone)]
pub enum PowerCardKind {
    Minor,
    Major,
    Spirit(u8),
}

pub enum PowerTargetFilter {
    None,
    Spirit(fn(&SpiritState) -> bool),
    Land(fn(&LandState) -> bool),
}

#[derive(Copy, Clone)]
pub enum PowerTarget {
    Spirit(u8),
    Land(u8),
}

#[derive(Copy, Clone)]
pub struct PowerUsage {
    pub target: PowerTarget,
    pub using_spirit_index: u8,
    pub src_land_index: Option<u8>,
}

pub struct PowerCardDescription {
    pub name: &'static str,
    
    pub kind: PowerCardKind,
    pub elements: ElementMap<bool>,

    pub cost: u8,
    pub speed: PowerSpeed,
    pub range: Option<u8>,
    pub target_filter: PowerTargetFilter,

    pub effect_builder: fn (&GameState) -> Result<Box<dyn Effect>, StepFailure>,
}


#[derive(Copy, Clone)]
pub struct PowerCard {
    pub index: usize
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
            desc: &Rc<Vec<PowerCardDescription>>,
            mut rng: &mut dyn RngCore) {
        self.draw
            = desc.iter()
                .enumerate()
                .map(|(i, _)| PowerCard{ index: i })
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
}