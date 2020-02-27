// This file contains copyrighted assets owned by Greater Than Games.

use std::{
    rc::{Rc},
};

use crate::base::{
    BoardDescription, LandDescription,
    LandKind, InvaderKind, TokenKind,
    TokenMap, InvaderMap
};

pub fn make_board_a() -> BoardDescription {
    let mut lands = vec![
        Rc::new(LandDescription {
            adjacent: Vec::new(),
            kind: LandKind::Ocean,
            is_coastal: true,
            edge_range: Some((2, 8)),
            index_on_board: 0,

            starting_tokens: TokenMap::new(|| 0),
            starting_invaders: InvaderMap::new(|| 0),
            starting_dahan: 0,

            index_on_table: 0,
            parent_board_index: 0,
        }),
        Rc::new(LandDescription {
            adjacent: Vec::new(),
            kind: LandKind::Mountain,
            is_coastal: true,
            edge_range: Some((8, 10)),
            index_on_board: 1,
            
            starting_tokens: TokenMap::new(|| 0),
            starting_invaders: InvaderMap::new(|| 0),
            starting_dahan: 0,

            index_on_table: 1,
            parent_board_index: 0,
        }),
        Rc::new(LandDescription {
            adjacent: Vec::new(),
            kind: LandKind::Wetlands,
            is_coastal: true,
            edge_range: None,
            index_on_board: 2,
            
            starting_tokens: TokenMap::new(|| 0),
            starting_invaders: InvaderMap::new(|| 0).map(InvaderKind::City, 1),
            starting_dahan: 1,

            index_on_table: 2,
            parent_board_index: 0,
        }),
        Rc::new(LandDescription {
            adjacent: Vec::new(),
            kind: LandKind::Jungle,
            is_coastal: true,
            edge_range: Some((25, 1)),
            index_on_board: 3,

            starting_tokens: TokenMap::new(|| 0),
            starting_invaders: InvaderMap::new(|| 0),
            starting_dahan: 2,

            index_on_table: 3,
            parent_board_index: 0,
        }),
        Rc::new(LandDescription {
            adjacent: Vec::new(),
            kind: LandKind::Sands,
            is_coastal: false,
            edge_range: Some((23, 25)),
            index_on_board: 4,

            starting_tokens: TokenMap::new(|| 0).map(TokenKind::Blight, 1),
            starting_invaders: InvaderMap::new(|| 0),
            starting_dahan: 0,

            index_on_table: 4,
            parent_board_index: 0,
        }),
        Rc::new(LandDescription {
            adjacent: Vec::new(),
            kind: LandKind::Wetlands,
            is_coastal: false,
            edge_range: Some((22, 23)),
            index_on_board: 5,

            starting_tokens: TokenMap::new(|| 0),
            starting_invaders: InvaderMap::new(|| 0),
            starting_dahan: 0,

            index_on_table: 5,
            parent_board_index: 0,
        }),
        Rc::new(LandDescription {
            adjacent: Vec::new(),
            kind: LandKind::Mountain,
            is_coastal: false,
            edge_range: Some((10, 12)),
            index_on_board: 6,

            starting_tokens: TokenMap::new(|| 0),
            starting_invaders: InvaderMap::new(|| 0),
            starting_dahan: 1,

            index_on_table: 6,
            parent_board_index: 0,
        }),
        Rc::new(LandDescription {
            adjacent: Vec::new(),
            kind: LandKind::Sands,
            is_coastal: false,
            edge_range: Some((16, 22)),
            index_on_board: 7,

            starting_tokens: TokenMap::new(|| 0),
            starting_invaders: InvaderMap::new(|| 0),
            starting_dahan: 2,

            index_on_table: 7,
            parent_board_index: 0,
        }),
        Rc::new(LandDescription {
            adjacent: Vec::new(),
            kind: LandKind::Jungle,
            is_coastal: false,
            edge_range: Some((12, 16)),
            index_on_board: 8,

            starting_tokens: TokenMap::new(|| 0),
            starting_invaders: InvaderMap::new(|| 0).map(InvaderKind::Town, 1),
            starting_dahan: 0,

            index_on_table: 8,
            parent_board_index: 0,
        }),
    ];

    (*Rc::get_mut(lands.get_mut(0).unwrap()).unwrap()).adjacent = vec![1, 2, 3];

    (*Rc::get_mut(lands.get_mut(1).unwrap()).unwrap()).adjacent = vec![0, 2, 4, 5, 6];
    (*Rc::get_mut(lands.get_mut(2).unwrap()).unwrap()).adjacent = vec![0, 1, 4, 3];
    (*Rc::get_mut(lands.get_mut(3).unwrap()).unwrap()).adjacent = vec![0, 2, 4];

    (*Rc::get_mut(lands.get_mut(4).unwrap()).unwrap()).adjacent = vec![1, 2, 3, 5];
    (*Rc::get_mut(lands.get_mut(5).unwrap()).unwrap()).adjacent = vec![1, 4, 6, 7, 8];
    (*Rc::get_mut(lands.get_mut(6).unwrap()).unwrap()).adjacent = vec![1, 5, 8];
    (*Rc::get_mut(lands.get_mut(7).unwrap()).unwrap()).adjacent = vec![5, 8];
    (*Rc::get_mut(lands.get_mut(8).unwrap()).unwrap()).adjacent = vec![5, 6, 7];

    BoardDescription {
        name: "A",
        lands: lands
    }
}
