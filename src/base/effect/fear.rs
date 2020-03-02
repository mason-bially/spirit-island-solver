
use std::{
    any::Any,
    cmp::*,
};

use super::*;


#[derive(Clone)]
pub struct GenerateFearEffect {
    pub fear: u8,
    pub land_index: Option<u8>, // ? some effects do proc off of this...
}

impl Effect for GenerateFearEffect {
    fn apply_effect(&self, game: &mut GameState) -> Result<(), StepFailure> {
        game.log_effect(format!("adding {} fear.", self.fear));

        // 1. Manipulate pool and draw cards!
        let mut remaining_fear = self.fear;
        while remaining_fear > 0 {
            let fear_to_move = min(self.fear, game.fear_pool);

            game.fear_generated += fear_to_move;
            game.fear_pool -= fear_to_move;
            remaining_fear -= fear_to_move;

            if game.fear_pool == 0 {
                game.log_subeffect("drawing a fear card!".to_string());

                game.fear.draw_into_pending();
                game.fear_pool = game.fear_generated;
                game.fear_generated = 0;

                // TODO also test map state!
                if game.fear.draw.len() == 0 {
                    game.do_victory("No fear cards remaining.")?;
                }
            }
        }

        // 2. Add fear to land?
        // TODO
        
        Ok(())
    }

    fn box_clone(&self) -> Box<dyn Effect> { Box::new(self.clone()) }
    fn as_any(&self) -> Box<dyn Any> { Box::new(self.clone()) }
}