// This file contains copyrighted assets owned by Greater Than Games.

use std::{
    fmt,
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
            Element::Sun => write!(f, "☀️"),
            Element::Moon => write!(f, "🌙"),
            Element::Fire => write!(f, "🔥"),
            Element::Air => write!(f, "🪶"),
            Element::Water => write!(f, "💧"),
            Element::Earth => write!(f, "⛰️"),
            Element::Plant => write!(f, "☘️"),
            Element::Animal => write!(f, "🐾"),
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
