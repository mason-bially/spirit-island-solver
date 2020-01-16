use std::rc::Rc;
use std::fmt;

use super::concept::{ContentPack};

#[derive(Copy, Clone)]
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
    adjacent: Vec<Rc<LandDescription>>,
    kind: LandKind,
    is_coastal: bool,
    edge_range: Option<(u8, u8)>, // ranges from 0 to 20... need to figure out the tiling, is it 7 gaps a side?
    map_index: u8,
    board_index: u8,
}

pub struct BoardDescription {
    name: String,
    lands: Vec<Rc<LandDescription>>,
}

pub struct MapDescription {
    boards: Vec<BoardDescription>,
}

pub struct LandState {
    desc: LandDescription,
}

pub struct MapState {
    desc: Rc<MapDescription>,
    lands: Vec<LandState>,
}

pub fn make_map(content: &Vec<Box<dyn ContentPack>>, board_names: Vec<&str>) -> MapDescription {
    MapDescription {
        boards: Vec::new()
    }
}