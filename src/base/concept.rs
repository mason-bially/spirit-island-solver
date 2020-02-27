
use std::{
    fmt,
    iter::*,
};

use super::board::{BoardDescription};
use super::step::{StepFailure};
use super::game::{GameState};
use super::deck::{FearCardDescription};

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

#[derive(Copy, Clone, PartialEq)]
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

#[derive(Copy, Clone)]
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


pub trait SpiritDescription {
    fn name(&self) -> &'static str;
    fn all_names(&self) -> &'static [&'static str];

    fn do_setup(&self, game: &mut GameState, spirit_index: usize) -> Result<(), StepFailure>;
}

pub trait PowerCardDescription {

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

    fn get_fear_cards(&self) -> Vec<Box<dyn FearCardDescription>>;
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

pub fn join_fear_cards(content: &Vec<Box<dyn ContentPack>>) -> Vec<Box<dyn FearCardDescription>>
{
    let mut result = Vec::new();
    for c in content.iter() {
        result.extend(c.get_fear_cards());
    }
    
    result
}

