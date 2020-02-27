// This file contains copyrighted assets owned by Greater Than Games.

use std::{
    rc::Rc,
    fmt,
    clone::Clone,
    collections::VecDeque,
};

use rand::prelude::*;

use crate::base::{
    concept::{LandKind, InvaderActionKind},
    deck::{Deck},
    board::{LandDescription},
};


pub trait FearCardDescription {

}


#[derive(Copy, Clone)]
pub struct FearCard {

}


#[derive(Clone)]
pub struct FearDeck {
    // List of all possible fear cards
    pub desc: Vec<Rc<Box<dyn FearCardDescription>>>,

    pub draw: Vec<FearCard>,
    pub pending: Vec<FearCard>,
    pub discard: Vec<FearCard>,
}

impl FearDeck {
    pub fn new() -> FearDeck {
        FearDeck {
            desc: Vec::new(),

            draw: Vec::new(),
            pending: Vec::new(),
            discard: Vec::new(),
        }
    }

    pub fn set_state(&mut self, draw: Vec<FearCard>) {
        self.draw = draw;
    }

    pub fn draw_into_pending(&mut self) {
        let draw = self.draw(1);
        let card = draw.first().unwrap();

        self.pending.push(*card);
    }

    pub fn advance(&mut self) {
        self.discard.append(&mut self.pending);
    }
}

impl Deck<FearCard> for FearDeck {
    fn shuffle_draw(&mut self, mut rng: &mut dyn RngCore) {
        self.draw.shuffle(&mut rng);
    }

    fn draw(&mut self, count: usize) -> Vec<FearCard> {
        let mut res = Vec::new();
        for _ in 0..count {
            res.insert(0, self.draw.pop().unwrap());
        }

        res
    }
}