// This file contains copyrighted assets owned by Greater Than Games.

use std::{
    rc::Rc,
    fmt,
};

use super::{
    deck::{PowerCardDescription, SpiritPowerDeck},
    step::{StepFailure},
    game::{GameState},
};


#[derive(Copy, Clone)]
pub struct SpiritMap<T>( pub [T; 6] );

impl<T> SpiritMap<T> {
    pub fn new<F>(v: F) -> SpiritMap<T>
        where F: Fn() -> T
    {
        SpiritMap( [v(),v(),v(),v(),v(),v()] )
    }
}

impl<T> std::ops::Index<u8> for SpiritMap<T>  {
    type Output = T;
    fn index(&self, s: u8) -> &T {
        &self.0[s as usize]
    }
}
impl<T> std::ops::IndexMut<u8> for SpiritMap<T>  {
    fn index_mut(&mut self, s: u8) -> &mut T {
        &mut self.0[s as usize]
    }
}


#[derive(PartialEq, Eq, Hash, Copy, Clone)]
pub enum Element {
    Sun,
    Moon,
    Fire,
    Air,
    Water,
    Earth,
    Plant,
    Animal,
}


impl fmt::Display for Element {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            // removed emojis due to not being fixed width
            Element::Sun => write!(f, "S"),      // ☀️
            Element::Moon => write!(f, "M"),     // 🌙
            Element::Fire => write!(f, "F"),      // 🔥
            Element::Air => write!(f, "A"),       // 🪶
            Element::Water => write!(f, "W"),     // 💧
            Element::Earth => write!(f, "E"),     // ⛰️
            Element::Plant => write!(f, "L"),     // ☘️
            Element::Animal => write!(f, "N"),    // 🐾
       }
    }
}


#[derive(Copy, Clone)]
pub struct ElementMap<T>( pub [T; 8] );

impl<T> ElementMap<T> {
    pub fn new<F>(v: F) -> ElementMap<T>
        where F: Fn() -> T
    {
        ElementMap( [v(),v(),v(),v(),v(),v(),v(),v()] )
    }

    pub fn map(mut self, kind: Element, value: T) -> Self {
        self[kind] = value;
        self
    }
}

impl ElementMap<bool> {
    pub fn from_slice(slice: &[Element]) -> ElementMap<bool>
    {
        let mut res = ElementMap::new(|| false);
        for e in slice {
            res[*e] = true;
        };
        res
    }
}

impl fmt::Display for ElementMap<bool> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.0[Element::Sun as usize] { write!(f, "{}", Element::Sun)?; } else { write!(f, " ")?; }
        if self.0[Element::Moon as usize] { write!(f, "{}", Element::Moon)?; } else { write!(f, " ")?; }
        if self.0[Element::Fire as usize] { write!(f, "{}", Element::Fire)?; } else { write!(f, " ")?; }
        if self.0[Element::Air as usize] { write!(f, "{}", Element::Air)?; } else { write!(f, " ")?; }
        if self.0[Element::Water as usize] { write!(f, "{}", Element::Water)?; } else { write!(f, " ")?; }
        if self.0[Element::Earth as usize] { write!(f, "{}", Element::Earth)?; } else { write!(f, " ")?; }
        if self.0[Element::Plant as usize] { write!(f, "{}", Element::Plant)?; } else { write!(f, " ")?; }
        if self.0[Element::Animal as usize] { write!(f, "{}", Element::Animal)?; } else { write!(f, " ")?; }

        Ok(())
    }
}

impl<T> std::ops::Index<Element> for ElementMap<T>  {
    type Output = T;
    fn index(&self, s: Element) -> &T {
        &self.0[s as usize]
    }
}
impl<T> std::ops::IndexMut<Element> for ElementMap<T>  {
    fn index_mut(&mut self, s: Element) -> &mut T {
        &mut self.0[s as usize]
    }
}



pub trait SpiritDescription {
    fn name(&self) -> &'static str;
    fn all_names(&self) -> &'static [&'static str];

    fn get_power_cards(&self, spirit_index: u8) -> Vec<PowerCardDescription>;

    fn do_setup(&self, game: &mut GameState, spirit_index: usize) -> Result<(), StepFailure>;

    fn do_growth(&self, game: &mut GameState, spirit_index: usize) -> Result<(), StepFailure>;
    fn do_income(&self, game: &mut GameState, spirit_index: usize) -> Result<(), StepFailure>;
}

#[derive(Copy, Clone)]
pub enum PresenceState {
    OnBoard(u8),
    OnTrack(u8),
    Destroyed,
    RemovedFromGame,
}


#[derive(Clone)]
pub struct SpiritState {
    pub desc: Rc<Box<dyn SpiritDescription>>,

    pub presence: [PresenceState; 13],
    pub deck: SpiritPowerDeck,

    pub energy: u8,
    pub plays: u8,
    pub elements: ElementMap<u8>,
}

impl SpiritState {
    pub fn new(desc: &Rc<Box<dyn SpiritDescription>>) -> SpiritState {
        SpiritState {
            desc: Rc::clone(desc),

            presence: [PresenceState::RemovedFromGame; 13],
            deck: SpiritPowerDeck::new(),

            energy: 0,
            plays: 0,
            elements: ElementMap::new(|| 0),
        }
    }

    pub fn init() {
        
    }
}
