use super::game::{GameState};

#[derive(Copy, Clone)]
pub enum InvaderActionKind {
    Ravage,
    Build,
    Explore
}

pub trait Spirit {
    fn name(&self) -> &'static str;
    fn all_names(&self) -> &'static [&'static str];

    fn do_reset(&mut self, game: &mut GameState);
    fn do_setup(&mut self, game: &mut GameState);
}

pub trait Power {

}

pub trait Fear {

}

pub trait Event {

}

pub trait Adversary {
    fn fear_cards(&self) -> (u8, u8, u8);
    fn invader_steps(&self) -> Vec<InvaderActionKind>;

    fn setup(&self, game: &mut GameState);
}

pub trait ContentPack {
    fn get_spirits(&self) -> Vec<Box<dyn Spirit>>;
}

pub struct DefaultAdversary {

}

impl DefaultAdversary {
    pub fn new() -> DefaultAdversary {
        DefaultAdversary {
            
        }
    }
}

impl Adversary for DefaultAdversary {
    fn fear_cards(&self) -> (u8, u8, u8) { 
        (3, 3, 3)
    }
    fn invader_steps(&self) -> Vec<InvaderActionKind> {
        vec![InvaderActionKind::Ravage, InvaderActionKind::Build, InvaderActionKind::Explore]
    }

    fn setup(&self, game: &mut GameState) {

    }
}

pub fn search_for_spirit(content: &Vec<Box<dyn ContentPack>>, name: &str) -> Option<Box<dyn Spirit>>
{
    for c in content.iter() {
        for s in c.get_spirits().into_iter() {
            for n in s.all_names() {
                if n == &name {
                    return Some(s);
                }
            }
        }
    }

    None
}

