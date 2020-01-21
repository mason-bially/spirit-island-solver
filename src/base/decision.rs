use std::rc::Rc;

use super::{ GameState };

#[derive(Copy, Clone)]
pub enum Decision {
    SequenceResolution(u8),
    DamageToInvaders(u16),
    CascadeBlight(u8),
}

#[derive(Copy, Clone)]
pub enum Choice {

}

#[derive(Clone)]
pub struct DecisionNode {
    pub decision: Decision,
    pub choice: Choice,
}
