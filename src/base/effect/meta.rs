
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


#[derive(Clone)]
pub struct SequencedEffect {
    pub sequence: Vec<SubEffect>,
}

impl Effect for SequencedEffect {
    fn apply_effect(&self, game: &mut GameState) -> Result<(), StepFailure> {
        game.log("sequence...".to_string());

        for sub_effect in self.sequence.iter() {
            let effect;

            match sub_effect {
                SubEffect::Built(built_effect) => {
                    effect = built_effect.box_clone();
                },
                SubEffect::AlwaysBuild(builder) => {
                    effect = builder(game)?;
                },
                SubEffect::ConditionalBuild(builder) => {
                    if let Some(res) = builder(game)? {
                        effect = res;
                    } else {
                        continue;
                    }
                }
            };

            game.do_effect_box(effect)?;
        }

        Ok(())
    }

    fn box_clone(&self) -> Box<dyn Effect> { Box::new(self.clone()) }
    fn as_any(&self) -> Box<dyn Any> { Box::new(self.clone()) }
}


#[derive(Clone)]
pub struct ForAllLandsDoEffect {
    pub filter: fn(land: &LandState) -> bool,
    pub effect: fn(land: &LandState) -> Box<dyn Effect>,
}

impl Effect for ForAllLandsDoEffect {
    fn apply_effect(&self, game: &mut GameState) -> Result<(), StepFailure> {
        game.log("for all lands...".to_string());

        let effects: Vec<Box<dyn Effect>>
            = game.table.lands.iter()
                .filter(|l| (self.filter)(l))
                .map(|l| (self.effect)(l))
                .collect();

        for effect in effects {
            game.do_effect_box(effect)?;
        }

        Ok(())
    }

    fn box_clone(&self) -> Box<dyn Effect> { Box::new(self.clone()) }
    fn as_any(&self) -> Box<dyn Any> { Box::new(self.clone()) }
}
