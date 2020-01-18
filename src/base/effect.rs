use super::game::{GameState};

pub trait Effect {
    fn apply_effect(&self, game: &mut GameState) -> Result<(), ()>;
}


pub struct AddBlightEffect {
    pub land_index: u8,
    pub count: u8,
}

impl Effect for AddBlightEffect {
    fn apply_effect(&self, game: &mut GameState) -> Result<(), ()> {
        println!("   |    `===> blighting land {}.", self.land_index);

        let mut land = game.map.lands.get_mut(self.land_index as usize).unwrap();

        // 1. Remove blight from card
        if game.blight_remaining == 0 {
            game.do_defeat("No blight is left.")?;
        }

        game.blight_remaining -= 1;

        // 2. Add blight to the land
        //land.add_blight();

        // 3. Kill presence
        // TODO

        // 4. Check for cascade
        // This is a decision point... ugh
        
        Ok(())
    }
}


pub struct DoSpiritDamageEffect {
    pub land_index: u8,
    pub count: u16,
}


pub struct DoInvaderDamageEffect {
    pub land_index: u8,
    pub count: u16,
}


pub struct RavageEffect {
    pub land_index: u8,
}

impl Effect for RavageEffect {
    fn apply_effect(&self, game: &mut GameState) -> Result<(), ()> {
        println!("   | `  ===> Ravaging effect in land {}.", self.land_index);

        let mut land = game.map.lands.get_mut(self.land_index as usize).unwrap();
        let invader_damage: u16 = land.pieces.iter().map(|p| p.invader_damage()).sum();

        let blight_threshold = 2;

        // TODO intercept and modify this damage:
        // * Adversary manipulations
        // * Spirit manipulations
        // * Powers and other effects
        //   * defense
        //   * modify invader damage
        //   * modify dahan health
        //   * modify blight threshold
        // * ...

        // Damage is done in two steps, one to the land and one to the dahan
        for dahan in land.pieces.iter_mut() {
            
        }

        if invader_damage >= blight_threshold {
            game.do_effect(AddBlightEffect { land_index: self.land_index, count: 1 })?;
        }

        Ok(())
    }
}
