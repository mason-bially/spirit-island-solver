use std::rc::Rc;
use std::fmt;

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
}

pub struct BoardDescription {
    name: String,
    lands: Vec<Rc<LandDescription>>,
}

pub struct MapDescription {
    boards: Vec<BoardDescription>,
}
