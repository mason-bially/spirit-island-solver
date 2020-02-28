// This file contains copyrighted assets owned by Greater Than Games.

use std::{
    rc::Rc,
    clone::Clone,
    iter::*,
};

use rand::prelude::*;

use crate::base::{
    concept::{PowerSpeed},
    effect::{Effect},
    spirit::{ElementMap}
};


#[derive(Copy, Clone)]
pub enum PowerCardKind {
    Minor,
    Major,
    Spirit(u8),
}


pub struct PowerCardDescription {
    pub name: &'static str,
    
    pub kind: PowerCardKind,
    pub speed: PowerSpeed,
    pub elements: ElementMap<bool>,

    pub effect: Box<dyn Effect>,
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
pub struct PowerSpiritDeck {
    pub draw: Vec<PowerCard>,
    pub hand: Vec<PowerCard>,
    pub pending: Vec<PowerCard>,
    pub discard: Vec<PowerCard>,
    pub forgotten: Vec<PowerCard>,
}
