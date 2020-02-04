
use super::*;


#[derive(Clone)]
pub struct AddBlightEffect {
    pub land_index: u8,
    pub count: u8,
}

impl Effect for AddBlightEffect {
    fn apply_effect(&self, game: &mut GameState) -> Result<(), ()> {
        game.log(format!("blighting land {}.", self.land_index));

        // 1. Remove blight from card
        if game.blight_remaining == 0 {
            game.do_defeat("No blight is left.")?;
        }

        game.blight_remaining -= 1;

        // 2. Add blight to the land
        let land = game.map.lands.get_mut(self.land_index as usize).unwrap();
        land.add_tokens(TokenKind::Blight, 1);

        // 3. Kill presence
        //land.destroy_presence();

        // 4. Check for cascade
        // This is a decision point... ugh
        
        Ok(())
    }
    fn box_clone(&self) -> Box<dyn Effect> {
        Box::new(self.clone())
    }
}


#[derive(Clone)]
pub struct AddInvaderEffect {
    pub land_index: u8,
    pub invader_kind: InvaderKind,
    pub count: u8,
}

impl Effect for AddInvaderEffect {
    fn apply_effect(&self, game: &mut GameState) -> Result<(), ()> {
        game.log(format!("adding invader {} {} to {}.", self.count, self.invader_kind, self.land_index));

        let land = game.map.lands.get_mut(self.land_index as usize).unwrap();

        // 1. Add the invaders
        for _ in 0..self.count {
            land.add_invader(self.invader_kind);
        }
        
        Ok(())
    }
    fn box_clone(&self) -> Box<dyn Effect> {
        Box::new(self.clone())
    }
}