use super::board::{BoardDescription};
use super::step::{GameStep};
use super::game::{GameState, StepFailure};

#[derive(Copy, Clone)]
pub enum InvaderActionKind {
    Ravage,
    Build,
    Explore
}

pub trait SpiritDescription {
    fn name(&self) -> &'static str;
    fn all_names(&self) -> &'static [&'static str];

    fn do_setup(&self, game: &mut GameState, spirit_index: usize) -> Result<(), StepFailure>;
}

pub trait Power {

}

pub trait Fear {

}

pub trait Event {

}

pub trait AdversaryDescription {
    fn fear_cards(&self) -> (u8, u8, u8);
    fn invader_steps(&self) -> Vec<InvaderActionKind>;

    fn setup(&self, game: &mut GameState);
}

pub trait ContentPack {
    fn get_spirits(&self) -> Vec<Box<dyn SpiritDescription>>;
    fn get_boards(&self) -> Vec<BoardDescription>;
}


pub fn search_for_spirit(content: &Vec<Box<dyn ContentPack>>, name: &str) -> Option<Box<dyn SpiritDescription>>
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

pub fn search_for_board(content: &Vec<Box<dyn ContentPack>>, name: &str) -> Option<BoardDescription>
{
    for c in content.iter() {
        for b in c.get_boards().into_iter() {
            if b.name == name {
                return Some(b);
            }
        }
    }

    None
}

