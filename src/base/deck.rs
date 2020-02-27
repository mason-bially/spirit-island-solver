
use rand::prelude::*;

/*
    In general decks of cards organized as Vecs will follow physical card rules:

    * Push a card means put it on top of the stack.
    * Pop means take off the top of the stack.

    This does however mean that the first card when iterating is the _bottom_ card which might be the last one poped.
*/

pub trait Deck<T> {
    fn shuffle_draw(&mut self, rng: &mut dyn RngCore);
    fn draw(&mut self, count: usize) -> Vec<T>;
}

#[derive(Clone)]
pub struct SimpleDeck<T> {
    draw: Vec<T>,
    discard: Vec<T>,
}

impl<T> SimpleDeck<T> {
    pub fn new() -> Self {
        SimpleDeck::<T> {
            draw: Vec::new(),
            discard: Vec::new(),
        }
    }

    pub fn set_state(&mut self, draw: Vec<T>, discard: Vec<T>) {
        self.draw = draw;
        self.discard = discard;
    }
}

impl<T> Deck<T> for SimpleDeck<T> {
    fn shuffle_draw(&mut self, mut rng: &mut dyn RngCore) {
        self.draw.shuffle(&mut rng);
    }

    fn draw(&mut self, count: usize) -> Vec<T> {
        let mut res = Vec::new();
        for _ in 0..count {
            res.insert(0, self.draw.pop().unwrap());
        }

        res
    }
}


mod invader;
mod fear;

pub use self::invader::*;
pub use self::fear::*;

