// This file contains copyrighted assets owned by Greater Than Games.

use std::{
    rc::{Rc},
};

use crate::base::{BoardDescription, LandDescription, LandKind, Piece, InvaderKind, TokenKind};

pub fn make_board_a() -> BoardDescription {
    let mut lands = vec![
        Rc::new(LandDescription {
            adjacent: Vec::new(),
            kind: LandKind::Ocean,
            is_coastal: true,
            edge_range: Some((2, 8)),
            board_index: 0,
            starting_pieces: Vec::new(),
            map_index: 0,
            parent_board_index: 0,
        }),
        Rc::new(LandDescription {
            adjacent: Vec::new(),
            kind: LandKind::Mountain,
            is_coastal: true,
            edge_range: Some((8, 10)),
            board_index: 1,
            starting_pieces: vec![],
            map_index: 1,
            parent_board_index: 0,
        }),
        Rc::new(LandDescription {
            adjacent: Vec::new(),
            kind: LandKind::Wetlands,
            is_coastal: true,
            edge_range: None,
            board_index: 2,
            starting_pieces: vec![
                Piece::Invader { kind: InvaderKind::City, health: 2 },
                Piece::Dahan { health: 2 },
            ],
            map_index: 2,
            parent_board_index: 0,
        }),
        Rc::new(LandDescription {
            adjacent: Vec::new(),
            kind: LandKind::Jungle,
            is_coastal: true,
            edge_range: Some((25, 1)),
            board_index: 3,
            starting_pieces: vec![
                Piece::Dahan { health: 2 },
                Piece::Dahan { health: 2 },
            ],
            map_index: 3,
            parent_board_index: 0,
        }),
        Rc::new(LandDescription {
            adjacent: Vec::new(),
            kind: LandKind::Sands,
            is_coastal: false,
            edge_range: Some((23, 25)),
            board_index: 4,
            starting_pieces: vec![
                Piece::Token { kind: TokenKind::Blight, count: 1 }
            ],
            map_index: 4,
            parent_board_index: 0,
        }),
        Rc::new(LandDescription {
            adjacent: Vec::new(),
            kind: LandKind::Wetlands,
            is_coastal: false,
            edge_range: Some((22, 23)),
            board_index: 5,
            starting_pieces: vec![],
            map_index: 5,
            parent_board_index: 0,
        }),
        Rc::new(LandDescription {
            adjacent: Vec::new(),
            kind: LandKind::Mountain,
            is_coastal: false,
            edge_range: Some((10, 12)),
            board_index: 6,
            starting_pieces: vec![
                Piece::Dahan { health: 2 },
            ],
            map_index: 6,
            parent_board_index: 0,
        }),
        Rc::new(LandDescription {
            adjacent: Vec::new(),
            kind: LandKind::Sands,
            is_coastal: false,
            edge_range: Some((16, 22)),
            board_index: 7,
            starting_pieces: vec![
                Piece::Dahan { health: 2 },
                Piece::Dahan { health: 2 },
            ],
            map_index: 7,
            parent_board_index: 0,
        }),
        Rc::new(LandDescription {
            adjacent: Vec::new(),
            kind: LandKind::Jungle,
            is_coastal: false,
            edge_range: Some((12, 16)),
            board_index: 8,
            starting_pieces: vec![
                Piece::Invader { kind: InvaderKind::Town, health: 2 },
            ],
            map_index: 8,
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
