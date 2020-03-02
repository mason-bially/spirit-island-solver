
use std::{
    fmt,
    iter::*,
};

use super::{
    board::{BoardDescription},
    deck::{FearCardDescription, PowerCardDescription},
    spirit::{SpiritDescription},
    game::{GameState},
};


#[derive(Copy, Clone)]
pub enum InvaderActionKind {
    Ravage,
    Build,
    Explore
}

impl fmt::Display for InvaderActionKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &*self {
            InvaderActionKind::Ravage => write!(f, "Ravage"),
            InvaderActionKind::Build => write!(f, "Build"),
            InvaderActionKind::Explore => write!(f, "Explore"),
       }
    }
}


#[derive(PartialEq, Eq, Hash, Copy, Clone)]
pub enum LandKind {
    Ocean,
    Jungle,
    Mountain,
    Sands,
    Wetlands,
}

impl fmt::Display for LandKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
       match *self {
            LandKind::Ocean => write!(f, "Ocean"),
            LandKind::Jungle => write!(f, "Jungle"),
            LandKind::Mountain => write!(f, "Mountain"),
            LandKind::Sands => write!(f, "Sands"),
            LandKind::Wetlands => write!(f, "Wetlands"),
       }
    }
}


#[derive(PartialEq, Eq, Hash, Copy, Clone)]
pub enum TerrorLevel {
    I,
    II,
    III
}

impl fmt::Display for TerrorLevel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &*self {
            TerrorLevel::I => write!(f, "Terror I"),
            TerrorLevel::II => write!(f, "Terror II"),
            TerrorLevel::III => write!(f, "Terror III"),
       }
    }
}


#[derive(PartialEq, Eq, Hash, Copy, Clone)]
pub enum PowerSpeed {
    Fast,
    Slow
}


pub trait EventCardDescription {

}

pub trait AdversaryDescription {
    fn fear_cards(&self) -> (u8, u8, u8);
    fn invader_steps(&self) -> Vec<InvaderActionKind>;

    fn setup(&self, game: &mut GameState);
}

pub trait ContentPack {
    fn get_spirits(&self) -> Vec<Box<dyn SpiritDescription>>;
    fn get_boards(&self) -> Vec<BoardDescription>;

    fn get_fear_cards(&self) -> Vec<FearCardDescription>;

    fn get_power_cards(&self) -> Vec<PowerCardDescription>;
}


pub fn search_for_spirit(content: &Vec<Box<dyn ContentPack>>, name: &str) -> Option<Box<dyn SpiritDescription>>
{
    for c in content.iter() {
        for s in c.get_spirits().into_iter() {
            for n in s.all_names() {
                if n == &name {
                    return Some(s);
                }
            }
        }
    }

    None
}

pub fn search_for_board(content: &Vec<Box<dyn ContentPack>>, name: &str) -> Option<BoardDescription>
{
    for c in content.iter() {
        for b in c.get_boards().into_iter() {
            if b.name == name {
                return Some(b);
            }
        }
    }

    None
}

pub fn join_fear_cards(content: &Vec<Box<dyn ContentPack>>) -> Vec<FearCardDescription>
{
    let mut result = Vec::new();
    for c in content.iter() {
        result.extend(c.get_fear_cards());
    }
    
    result
}

pub fn join_power_cards(content: &Vec<Box<dyn ContentPack>>) -> Vec<PowerCardDescription>
{
    let mut result = Vec::new();
    for c in content.iter() {
        result.extend(c.get_power_cards());
    }
    
    result
}

