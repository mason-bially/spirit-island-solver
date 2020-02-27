
use std::{
    any::Any
};

use super::*;


#[derive(Clone)]
pub struct NotImplementedEffect {
    pub what: &'static str,
}

impl Effect for NotImplementedEffect {
    fn apply_effect(&self, game: &mut GameState) -> Result<(), StepFailure> {
        game.log(format!("NOT IMPLEMENTED {}.", self.what));

        Ok(())
    }

    fn box_clone(&self) -> Box<dyn Effect> { Box::new(self.clone()) }
    fn as_any(&self) -> Box<dyn Any> { Box::new(self.clone()) }
}
