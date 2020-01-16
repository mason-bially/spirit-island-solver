
#[derive(Copy, Clone)]
pub enum TokenKind {
    Blight,
    Beast,
    Wilds,
    Disease,
    Strife,
    Badlands,
}

#[derive(Copy, Clone)]
pub enum InvaderKind {
    Explorer,
    Town,
    City,
}

#[derive(Copy, Clone)]
pub enum Piece {
    Token {kind: TokenKind, count: u8},
    Scenario {index: u8},
    Presence {index: u8, count: u8},
    Dahan {health: u8},
    Invader {kind: InvaderKind, health: u8},
}

impl Piece {
    pub fn invader_damage(&self) -> u16 {
        match *self {
            Piece::Invader { kind: InvaderKind::Explorer, .. } => 1,
            Piece::Invader { kind: InvaderKind::Town, .. } => 2,
            Piece::Invader { kind: InvaderKind::City, .. } => 3,
            _ => 0,
        }
    }
}
