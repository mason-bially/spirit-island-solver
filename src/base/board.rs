// This file contains copyrighted assets owned by Greater Than Games.

use std::{
    rc::{Rc},
    fmt,
    iter::*,
};

use super::{
    piece::{TokenKind, InvaderKind, Piece},
    concept::{ContentPack, search_for_board},
};

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

pub struct LandDescription {
    pub adjacent: Vec<u8>,
    pub kind: LandKind,
    pub is_coastal: bool,
    pub edge_range: Option<(u8, u8)>, // ranges from 0 to 20... need to figure out the tiling, is it 7 gaps a side?
    pub board_index: u8,
    pub starting_pieces: Vec<Piece>,
    pub map_index: u8,
    pub parent_board_index: u8,
}

pub struct BoardDescription {
    pub name: &'static str,
    pub lands: Vec<Rc<LandDescription>>,
}

pub struct MapDescription {
    pub boards: Vec<BoardDescription>,
    pub lands: Vec<Rc<LandDescription>>,
    pub land_count: u8,
}

#[derive(Clone)]
pub struct LandState {
    pub desc: Rc<LandDescription>,
    pub pieces: Vec<Piece>,
}

#[derive(Clone)]
pub struct MapState {
    pub desc: Rc<MapDescription>,
    pub lands: Vec<LandState>,
}

pub fn make_map(content: &Vec<Box<dyn ContentPack>>, board_names: Vec<&str>) -> MapDescription {
    let mut boards = Vec::new();
    let mut lands = Vec::new();
    let mut land_count = 0;
    let mut board_count = 0;

    for board_name in board_names.into_iter() {
        let mut board = search_for_board(content, board_name).unwrap();
        for land in board.lands.iter_mut() {
            let land_mut = Rc::get_mut(land).unwrap();
            land_mut.map_index += land_count;
            land_mut.parent_board_index = board_count;
            
            lands.push(land.clone());
        }

        land_count += board.lands.len() as u8;
        boards.push(board);

        board_count += 1;
    }

    MapDescription {
        boards,
        lands,
        land_count
    }
}


impl MapDescription {
    pub fn get_land(&self, index: u8) -> Rc<LandDescription> {
        self.lands.get(index as usize).unwrap().clone()
    }

    pub fn lands_adjacent(&self, adjacent_to_index: u8) -> Vec<Rc<LandDescription>> {
        self.lands.clone().into_iter()
            .filter(|l| l.adjacent.contains(&adjacent_to_index))
            .collect()
    }
}

impl LandState {
    pub fn add_tokens(&mut self, kind: TokenKind, amount: u8) {
        let entry = self.pieces.iter_mut().find(|piece| match piece {
            Piece::Token{kind: pkind, ..} => *pkind == kind,
            _ => false
        });

        if let Some(Piece::Token{count, ..}) = entry {
            *count += amount;
        } else {
            self.pieces.push(Piece::Token{kind: kind, count: amount});
        }
    }

    pub fn get_token_count(&mut self, kind: TokenKind) -> u8 {
        let entry = self.pieces.iter().find(|piece| match piece {
            Piece::Token{kind: pkind, ..} => *pkind == kind,
            _ => false
        });

        match entry {
            Some(Piece::Token {count, ..}) => *count,
            _ => 0
        }
    }

    pub fn add_invader(&mut self, kind: InvaderKind) {
        self.pieces.push(Piece::Invader{kind: kind, health: kind.health()});
    }
}

impl MapState {
    pub fn new(desc: Rc<MapDescription>) -> MapState {
        let mut lands = Vec::new();

        for board in desc.boards.iter() {
            for land in board.lands.iter() {
                lands.push(LandState {
                    desc: land.clone(),
                    pieces: land.starting_pieces.clone(),
                });
            }
        }

        MapState {
            desc,
            lands,
        }
    }
}