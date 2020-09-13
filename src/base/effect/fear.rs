
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
        game.log_effect(format_args!("adding {} fear.", self.fear));

        // 1. Manipulate pool and draw cards!
        let mut remaining_fear = self.fear;
        while remaining_fear > 0 {
            let fear_to_move = min(remaining_fear, game.fear_pool);

            game.fear_generated += fear_to_move;
            game.fear_generated_total += fear_to_move;
            game.fear_pool -= fear_to_move;
            remaining_fear -= fear_to_move;

            if game.fear_pool == 0 {
                game.log_subeffect(format_args!("drawing a fear card!"));

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
        match self.land_index {
            Some(land_index) => {
                game.get_land_mut(land_index)?.fear_generated_here_this_round += self.fear 
            },
            _ => { }
        }

        Ok(())
    }

    fn box_clone(&self) -> Box<dyn Effect> { Box::new(self.clone()) }
    fn as_any(&self) -> Box<dyn Any> { Box::new(self.clone()) }
}
