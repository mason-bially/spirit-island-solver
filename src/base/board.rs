// This file contains copyrighted assets owned by Greater Than Games.

use std::{
    rc::{Rc},
    iter::*,
};

use super::{
    piece::*,
    concept::{LandKind, ContentPack, search_for_board},
};

pub struct LandDescription {
    // -- Primary description elements --

    pub index_on_board: u8,
    pub adjacent: Vec<u8>,
    pub kind: LandKind,
    pub is_coastal: bool,
    pub edge_range: Option<(u8, u8)>, // ranges from 0 to 27... need to figure out the tiling, is it 7 gaps a side?

    pub starting_tokens: TokenMap<u8>,
    pub starting_invaders: InvaderMap<u8>,
    pub starting_dahan: u8,

    // -- Generated description elements --

    pub index_in_map: u8,
    pub parent_board_index: u8,
}

pub struct BoardDescription {
    pub name: &'static str,
    pub lands: Vec<Rc<LandDescription>>,
}

pub struct TableDescription {
    pub boards: Vec<BoardDescription>,
    pub lands: Vec<Rc<LandDescription>>,
    pub land_count: u8,
}

#[derive(Clone)]
pub struct LandState {
    pub desc: Rc<LandDescription>,

    pub is_in_play: bool,

    pub tokens: TokenMap<u8>,
    pub presence: SpiritMap<u8>,

    pub invaders: Vec<Invader>,
    pub dahan: Vec<Dahan>,

    pub defense: u16,
}

#[derive(Clone)]
pub struct TableState {
    pub desc: Rc<TableDescription>,

    pub lands: Vec<LandState>,
}

pub fn make_map(content: &Vec<Box<dyn ContentPack>>, board_names: Vec<&str>) -> TableDescription {
    let mut boards = Vec::new();
    let mut lands = Vec::new();
    let mut land_count = 0;
    let mut board_count = 0;

    for board_name in board_names.into_iter() {
        let mut board = search_for_board(content, board_name).unwrap();
        for land in board.lands.iter_mut() {
            let land_mut = Rc::get_mut(land).unwrap();
            land_mut.index_in_map += land_count;
            land_mut.parent_board_index = board_count;
            
            lands.push(land.clone());
        }

        land_count += board.lands.len() as u8;
        boards.push(board);

        board_count += 1;
    }

    TableDescription {
        boards,
        lands,
        land_count
    }
}


impl TableDescription {
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
    pub fn time_passes(&mut self) {
        for invader in self.invaders.iter_mut() {
            invader.time_passes();
        }
        for dahan in self.dahan.iter_mut() {
            dahan.time_passes();
        }

        self.defense = 0;
    }
}

impl TableState {
    pub fn new(desc: Rc<TableDescription>) -> TableState {
        let mut lands = Vec::new();

        for board in desc.boards.iter() {
            for land in board.lands.iter() {
                lands.push(LandState {
                    desc: land.clone(),

                    is_in_play: land.index_on_board != 0,

                    tokens: land.starting_tokens.clone(),
                    presence: SpiritMap::new(|| 0),

                    invaders: repeat(Invader::new(InvaderKind::Explorer)).take(land.starting_invaders[InvaderKind::Explorer] as usize)
                        .chain(repeat(Invader::new(InvaderKind::Town)).take(land.starting_invaders[InvaderKind::Town] as usize))
                        .chain(repeat(Invader::new(InvaderKind::City)).take(land.starting_invaders[InvaderKind::City] as usize))
                        .collect(),
                    dahan: repeat(Dahan::new()).take(land.starting_dahan as usize).collect(),

                    defense: 0,
                });
            }
        }

        TableState {
            desc,
            lands,
        }
    }
}