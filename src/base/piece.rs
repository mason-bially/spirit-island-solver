
pub enum TokenKind {
    Blight,
    Beast,
    Wilds,
    Disease,
    Strife,
    Badlands,
}

pub enum InvaderKind {
    Explorer,
    Town,
    City,
}

pub enum Piece {
    Token {kind: TokenKind, count: u8},
    Scenario {index: u8},
    Presence {index: u8, count: u8},
    Dahan {health: u8},
    Invader {kind: InvaderKind, health: u8},
}