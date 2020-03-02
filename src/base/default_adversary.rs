use super::concept::{AdversaryDescription, InvaderActionKind};
use super::step::{GameStep};
use super::game::{GameState};
use super::deck::{InvaderCard};

pub struct DefaultAdversaryDescription {

}

impl DefaultAdversaryDescription {
    pub fn new() -> DefaultAdversaryDescription {
        DefaultAdversaryDescription {
            
        }
    }
}

pub fn invader_deck_setup_standard(game: &mut GameState)
{
    let phase3 = game.invader.draw.iter().position(|&x| if let InvaderCard::Phase3(_, _) = x { true } else { false }).unwrap();
    game.log_effect(format_args!("Removing {{{}}}", game.invader.draw.get(phase3).unwrap()));
    game.invader.draw.remove(phase3);

    let phase2 = game.invader.draw.iter().position(|&x| if let InvaderCard::Phase2(_) = x { true } else { false }).unwrap();
    game.log_effect(format_args!("Removing {{{}}}", game.invader.draw.get(phase2).unwrap()));
    game.invader.draw.remove(phase2);

    let phase1 = game.invader.draw.iter().position(|&x| if let InvaderCard::Phase1(_) = x { true } else { false }).unwrap();
    game.log_effect(format_args!("Removing {{{}}}", game.invader.draw.get(phase1).unwrap()));
    game.invader.draw.remove(phase1);
}


impl AdversaryDescription for DefaultAdversaryDescription {
    fn fear_cards(&self) -> (u8, u8, u8) { 
        (3, 3, 3)
    }
    fn invader_steps(&self) -> Vec<InvaderActionKind> {
        vec![InvaderActionKind::Ravage, InvaderActionKind::Build, InvaderActionKind::Explore]
    }

    fn setup(&self, game: &mut GameState) {
        let step = game.step;

        match step {
            GameStep::Init => {
                invader_deck_setup_standard(game);
            },
            _ => {},
        }
    }
}