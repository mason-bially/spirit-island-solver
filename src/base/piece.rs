// This file contains copyrighted assets owned by Greater Than Games.

use std::{
    fmt,
};

#[derive(PartialEq, Eq, Hash, Copy, Clone)]
pub enum TokenKind {
    Blight,
    Beast,
    Wilds,
    Disease,
    Strife,
    Badlands,
}

impl fmt::Display for TokenKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            TokenKind::Blight => write!(f, "Blight"),
            TokenKind::Beast => write!(f, "Beast"),
            TokenKind::Wilds => write!(f, "Wilds"),
            TokenKind::Disease => write!(f, "Disease"),
            TokenKind::Strife => write!(f, "Strife"),
            TokenKind::Badlands => write!(f, "Badlands"),
       }
    }
}

#[derive(Copy, Clone)]
pub struct TokenMap<T>( [T; 6] );

impl<T> TokenMap<T> {
    pub fn new<F>(v: F) -> TokenMap<T>
        where F: Fn() -> T
    {
        TokenMap( [v(),v(),v(),v(),v(),v()] )
    }

    pub fn map(mut self, kind: TokenKind, value: T) -> Self {
        self[kind] = value;
        self
    }
}

impl<T> std::ops::Index<TokenKind> for TokenMap<T>  {
    type Output = T;
    fn index(&self, s: TokenKind) -> &T {
        &self.0[s as usize]
    }
}
impl<T> std::ops::IndexMut<TokenKind> for TokenMap<T>  {
    fn index_mut(&mut self, s: TokenKind) -> &mut T {
        &mut self.0[s as usize]
    }
}


#[derive(PartialEq, Eq, Hash, Copy, Clone)]
pub enum InvaderKind {
    Explorer,
    Town,
    City,
}

impl InvaderKind {
    pub fn attack(&self) -> u16 {
        match *self {
            InvaderKind::Explorer => 1,
            InvaderKind::Town => 2,
            InvaderKind::City => 3,
        }
    }

    pub fn health(&self) -> u8 {
        match *self {
            InvaderKind::Explorer => 1,
            InvaderKind::Town => 2,
            InvaderKind::City => 3,
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
pub struct InvaderMap<T>( [T; 3] );

impl<T> InvaderMap<T> 
where T : Copy {
    pub fn new(v: T) -> InvaderMap<T>
    {
        InvaderMap( [v,v,v] )
    }

    pub fn map(mut self, kind: InvaderKind, value: T) -> Self {
        self[kind] = value;
        self
    }
}

impl<T> std::ops::Index<InvaderKind> for InvaderMap<T>  {
    type Output = T;
    fn index(&self, s: InvaderKind) -> &T {
        &self.0[s as usize]
    }
}
impl<T> std::ops::IndexMut<InvaderKind> for InvaderMap<T>  {
    fn index_mut(&mut self, s: InvaderKind) -> &mut T {
        &mut self.0[s as usize]
    }
}


#[derive(Copy, Clone)]
pub struct Invader {
    pub kind: InvaderKind,
    pub health_max: u8,
    pub health_cur: u8,
    pub attack: u16,
}

impl Invader {
    pub fn new(kind: InvaderKind) -> Invader {
        Invader {
            kind,

            health_max: kind.health(),
            health_cur: kind.health(),
            attack: kind.attack(),
        }
    }

    pub fn is_building(&self) -> bool {
        self.kind.is_building()
    }

    pub fn time_passes(&mut self) {
        self.health_max = self.kind.health();
        self.health_cur = self.health_max;
        self.attack = self.kind.attack();
    }
}

#[derive(Copy, Clone)]
pub struct Dahan {
    pub health_max: u8,
    pub health_cur: u8,
    pub attack: u16,
}

impl Dahan {
    pub fn new() -> Dahan {
        Dahan {
            health_max: 2,
            health_cur: 2,
            attack: 2,
        }
    }

    pub fn time_passes(&mut self) {
        self.health_max = 2;
        self.health_cur = self.health_max;
        self.attack = 2;
    }
}

#[derive(PartialEq, Eq, Hash, Copy, Clone)]
pub enum PieceKind {
    Token(TokenKind),
    Invader(InvaderKind),
    Dahan,
}

impl fmt::Display for PieceKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            PieceKind::Token(tok) => write!(f, "{}", tok),
            PieceKind::Invader(inv) => write!(f, "{}", inv),
            PieceKind::Dahan => write!(f, "Dahan"),
       }
    }
}
