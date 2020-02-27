
use std::{
    fmt,
};

use super::board::{BoardDescription};
use super::step::{StepFailure};
use super::game::{GameState};

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

