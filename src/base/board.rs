// This file contains copyrighted assets owned by Greater Than Games.

use std::{
    sync::{Arc},
    iter::*,
};

use super::{
    piece::*,
    spirit::{SpiritMap},
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

    pub index_on_table: u8,
    pub parent_board_index: u8,
}

pub struct BoardDescription {
    pub name: &'static str,
    pub lands: Vec<Arc<LandDescription>>,
}

pub struct TableDescription {
    pub boards: Vec<BoardDescription>,
    pub lands: Vec<Arc<LandDescription>>,
    pub land_count: u8,
}

#[derive(Clone)]
pub struct LandState {
    pub desc: Arc<LandDescription>,

    pub is_in_play: bool,

    pub tokens: TokenMap<u8>,
    pub presence: SpiritMap<u8>,

    pub invaders: Vec<Invader>,
    pub dahan: Vec<Dahan>,

    // more specific effects
    pub defense: u16,
    pub fear_generated_here_this_round: u8,
}

#[derive(Clone)]
pub struct TableState {
    pub desc: Arc<TableDescription>,

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
            let land_mut = Arc::get_mut(land).unwrap();
            land_mut.index_on_table += land_count;
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
    pub fn get_land(&self, index: u8) -> Arc<LandDescription> {
        self.lands.get(index as usize).unwrap().clone()
    }

    pub fn get_adjacent_lands(&self, adjacent_to_index: u8) -> Vec<Arc<LandDescription>> {
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
        self.fear_generated_here_this_round = 0;
    }

    pub fn get_count(&self, pk: &PieceKind) -> usize {
        match pk {
            PieceKind::Token(tok_kind) => self.tokens[*tok_kind] as usize,
            PieceKind::Invader(inv_kind) => self.invaders.iter().filter(|i| i.kind == *inv_kind).count(),
            PieceKind::Dahan => self.dahan.len(),
        }
    }
}

impl TableState {
    pub fn new(desc: Arc<TableDescription>) -> TableState {
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
                    fear_generated_here_this_round: 0,
                });
            }
        }

        TableState {
            desc,
            lands,
        }
    }
}