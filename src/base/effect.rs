use std::{
    iter::*,
};

use super::*;

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
}


pub struct AddInvaderEffect {
    pub land_index: u8,
    pub invader_kind: InvaderKind,
    pub count: u8,
}

impl Effect for AddInvaderEffect {
    fn apply_effect(&self, game: &mut GameState) -> Result<(), ()> {
        println!("   |    `===> adding invader {} {} to {}.", self.count, self.invader_kind, self.land_index);

        let land = game.map.lands.get_mut(self.land_index as usize).unwrap();

        // 1. Add the invaders
        for _ in 0..self.count {
            land.add_invader(self.invader_kind);
        }
        
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


pub struct ExploreEffect {
    pub land_index: u8,
}

impl Effect for ExploreEffect {
    fn apply_effect(&self, game: &mut GameState) -> Result<(), ()> {
        println!("   | `  ===> Exploring in land {}.", self.land_index);

        let adj_lands = game.map.desc.lands_adjacent(self.land_index);
        let will_explore = adj_lands.iter().any(|l|
            game.map.lands.get(l.map_index as usize).unwrap().pieces.iter().any(|p| p.is_building())
        );

        if will_explore {
            game.do_effect(AddInvaderEffect {
                land_index: self.land_index,
                invader_kind: InvaderKind::Explorer,
                count: 1
            })?;
        }

        Ok(())
    }
}


pub struct BuildEffect {
    pub land_index: u8,
}

impl Effect for BuildEffect {
    fn apply_effect(&self, game: &mut GameState) -> Result<(), ()> {
        println!("   | `  ===> Building in land {}.", self.land_index);

        let land = game.map.lands.get(self.land_index as usize).unwrap();
        let will_build = land.pieces.iter().any(|p| p.is_invader());

        if will_build {
            let building_type_distance : i8 = land.pieces.iter().map(|p|
                match p {
                    Piece::Invader { kind: InvaderKind::Town, .. } => -1,
                    Piece::Invader { kind: InvaderKind::City, .. } => 1,
                    _ => 0,
                }).sum();

            game.do_effect(AddInvaderEffect {
                land_index: self.land_index,
                invader_kind: if building_type_distance > 0 { InvaderKind::Town } else { InvaderKind::City },
                count: 1
            })?;
        }

        Ok(())
    }
}


pub struct RavageEffect {
    pub land_index: u8,
}

impl Effect for RavageEffect {
    fn apply_effect(&self, game: &mut GameState) -> Result<(), ()> {
        println!("   | `  ===> Ravaging effect in land {}.", self.land_index);

        let land = game.map.lands.get_mut(self.land_index as usize).unwrap();
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
