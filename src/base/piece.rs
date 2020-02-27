// This file contains copyrighted assets owned by Greater Than Games.

use std::{
    fmt,
    iter::*,
};

#[derive(Copy, Clone, PartialEq)]
pub enum TokenKind {
    Blight,
    Beast,
    Wilds,
    Disease,
    Strife,
    Badlands,
}

#[derive(Copy, Clone, PartialEq)]
pub enum InvaderKind {
    Explorer,
    Town,
    City,
}

impl InvaderKind {
    pub fn damage(&self) -> u16 {
        match *self {
            InvaderKind::Explorer => 1,
            InvaderKind::Town => 2,
            InvaderKind::City => 3,
            _ => 0,
        }
    }

    pub fn health(&self) -> u8 {
        match *self {
            InvaderKind::Explorer => 1,
            InvaderKind::Town => 2,
            InvaderKind::City => 3,
            _ => 0,
        }
    }

    pub fn is_building(&self) -> bool {
        match *self {
            InvaderKind::Town | InvaderKind::City => true,
            _ => false,
        }
    }
}

impl fmt::Display for InvaderKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
       match *self {
            InvaderKind::Explorer => write!(f, "Explorer"),
            InvaderKind::Town => write!(f, "Town"),
            InvaderKind::City => write!(f, "City"),
       }
    }
}


#[derive(Copy, Clone)]
pub enum Piece {
    Token {kind: TokenKind, count: u8},
    Scenario {index: u8},
    Presence {spirit: u8, count: u8},
    Dahan {health: u8},
    Invader {kind: InvaderKind, health: u8},
}

impl Piece {
    pub fn is_invader(&self) -> bool {
        match *self {
            Piece::Invader { .. } => true,
            _ => false,
        }
    }

    pub fn is_building(&self) -> bool {
        match *self {
            Piece::Invader { kind, .. } => kind.is_building(),
            _ => false,
        }
    }

    pub fn is_explorer(&self) -> bool {
        match *self {
            Piece::Invader { kind: InvaderKind::Explorer, .. } => true,
            _ => false,
        }
    }

    pub fn is_town(&self) -> bool {
        match *self {
            Piece::Invader { kind: InvaderKind::Town, .. } => true,
            _ => false,
        }
    }

    pub fn is_city(&self) -> bool {
        match *self {
            Piece::Invader { kind: InvaderKind::City, .. } => true,
            _ => false,
        }
    }

    pub fn invader_damage(&self) -> u16 {
        match *self {
            Piece::Invader { kind, .. } => kind.damage(),
            _ => 0,
        }
    }

    pub fn invader_health(&self) -> u8 {
        match *self {
            Piece::Invader { kind, .. } => kind.health(),
            _ => 0,
        }
    }
}
