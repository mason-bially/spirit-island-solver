// This file contains copyrighted assets owned by Greater Than Games.

use std::{
    rc::Rc,
    clone::Clone,
    iter::*,
};

use rand::prelude::*;

use crate::base::{
    concept::{TerrorLevel},
};


pub trait FearCardDescription {

}


#[derive(Copy, Clone)]
pub struct FearCard {
    index: usize
}


#[derive(Clone)]
pub struct FearDeck {
    // List of all possible fear cards
    pub desc: Rc<Vec<Box<dyn FearCardDescription>>>,
            
    tier2_count: usize,
    tier3_count: usize,

    pub draw: Vec<FearCard>,
    pub pending: Vec<FearCard>,
    pub discard: Vec<FearCard>,
}

impl FearDeck {
    pub fn new(desc: Rc<Vec<Box<dyn FearCardDescription>>>) -> FearDeck {
        FearDeck {
            desc,

            tier2_count: 0,
            tier3_count: 0,

            draw: Vec::new(),
            pending: Vec::new(),
            discard: Vec::new(),
        }
    }

    pub fn init(&mut self, mut rng: &mut dyn RngCore, fear_card_counts: (u8, u8, u8)) {
        let mut all_cards: Vec<FearCard> = self.desc.iter()
            .enumerate()
            .map(|(i, _)| FearCard{ index: i })
            .collect();
        all_cards.shuffle(&mut rng);

        let (t1, t2, t3) = fear_card_counts;

        self.tier3_count = t3 as usize;
        self.tier2_count = (t3 + t2) as usize;

        for _ in 0..(t3 + t2 + t1) {
            self.draw.push(all_cards.pop().unwrap());
        }
    }

    pub fn terror_level(&self) -> TerrorLevel {
        let remaining_cards = self.draw.len();
        if remaining_cards <= self.tier3_count {
            TerrorLevel::III
        } else if remaining_cards <= self.tier2_count {
            TerrorLevel::II
        } else {
            TerrorLevel::I
        }
    }

    pub fn draw(&mut self, count: usize) -> Vec<FearCard> {
        let mut res = Vec::new();
        for _ in 0..count {
            res.insert(0, self.draw.pop().unwrap());
        }

        res
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
