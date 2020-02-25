
use super::*;

#[derive(Clone)]
pub struct CascadeBlightDecision {
    pub src_land_index: u8
}

impl Effect for CascadeBlightDecision {
    fn apply_effect(&self, game: &mut GameState) -> Result<(), ()> {
        let land_index = match game.consume_choice()
        {
            Ok(DecisionChoice::TargetLand(land)) => Ok(land),
            _ => Err(()),
        }?;

        game.log(format!("cascading blight to: {}", land_index));

        game.do_effect(AddBlightEffect { land_index, count: 1 });
        
        Ok(())
    }

    fn box_clone(&self) -> Box<dyn Effect> {
        Box::new(self.clone())
    }
}

impl Decision for CascadeBlightDecision {
    fn as_any(&self) -> Box<dyn Any> {
        Box::new(self.clone())
    }
}