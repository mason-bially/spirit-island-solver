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

#[derive(Copy, Clone)]
pub enum InvaderCard {
    Phase1(LandKind),
    Phase2(LandKind),
    Phase3(LandKind, LandKind),
}

impl fmt::Display for InvaderCard {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
       match &*self {
            InvaderCard::Phase1(a) => write!(f, "Phase I {}", a),
            InvaderCard::Phase2(LandKind::Ocean) => write!(f, "Phase II Coastal"),
            InvaderCard::Phase2(a) => write!(f, "Phase II {} +", a),
            InvaderCard::Phase3(a, b) => write!(f, "Phase III {}/{}", a, b),
       }
    }
}

impl InvaderCard {
    pub fn can_target(&self, land: &Rc<LandDescription>) -> bool {
        match *self {
            InvaderCard::Phase1(kind) => kind == land.kind,
            InvaderCard::Phase2(LandKind::Ocean) => land.kind != LandKind::Ocean && land.is_coastal,
            InvaderCard::Phase2(kind) => kind == land.kind,
            InvaderCard::Phase3(kind_a, kind_b) => kind_a == land.kind || kind_b == land.kind,
        }
    }
}

pub fn generate_invader_deck() -> Vec<InvaderCard> {
    vec![
        InvaderCard::Phase3(LandKind::Jungle, LandKind::Mountain),
        InvaderCard::Phase3(LandKind::Jungle, LandKind::Sands),
        InvaderCard::Phase3(LandKind::Jungle, LandKind::Wetlands),
        InvaderCard::Phase3(LandKind::Mountain, LandKind::Sands),
        InvaderCard::Phase3(LandKind::Mountain, LandKind::Wetlands),
        InvaderCard::Phase3(LandKind::Sands, LandKind::Wetlands),
        
        InvaderCard::Phase2(LandKind::Ocean),
        InvaderCard::Phase2(LandKind::Jungle),
        InvaderCard::Phase2(LandKind::Mountain),
        InvaderCard::Phase2(LandKind::Sands),
        InvaderCard::Phase2(LandKind::Wetlands),

        InvaderCard::Phase1(LandKind::Jungle),
        InvaderCard::Phase1(LandKind::Mountain),
        InvaderCard::Phase1(LandKind::Sands),
        InvaderCard::Phase1(LandKind::Wetlands),
    ]
}



#[derive(Clone)]
pub struct InvaderDeck {
    pub draw: Vec<InvaderCard>,
    pub discard: Vec<InvaderCard>,
    pub pending: VecDeque<Vec<InvaderCard>>,
    pub sequence: Vec<InvaderActionKind>,
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

        for _step in self.sequence.iter() { self.pending.push_back(Vec::new()); }
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