use std::{
    clone::Clone,
};

use rand::prelude::*;
use rand_chacha::{ChaChaRng};


/* 
    Our RNG needs to be deterministic and copyable.
*/

pub trait DeterministicRng {
    fn get_rng<'a>(&'a mut self) -> &'a mut dyn RngCore;
    fn box_clone(&self) -> Box<dyn DeterministicRng>;
}

pub struct DeterministicChaCha {
    rng: ChaChaRng
}

impl DeterministicChaCha {
    pub fn new(rng: ChaChaRng) -> Self {
        DeterministicChaCha {
            rng
        }
    }
}

impl DeterministicRng for DeterministicChaCha {
    fn get_rng<'a>(&'a mut self) -> &'a mut dyn RngCore {
        &mut self.rng
    }
    fn box_clone(&self) -> Box<dyn DeterministicRng> {
        Box::new(DeterministicChaCha::new(self.rng.clone()))
    }
}

impl Clone for Box<dyn DeterministicRng> {
    fn clone(&self) -> Self {
        self.box_clone()
    }
}
