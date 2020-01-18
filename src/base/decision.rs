use std::rc::Rc;

use super::{ GameState };

#[derive(Copy, Clone)]
pub enum DecisionKind {
    DamageToInvaders(u16),
    CascadeBlight(u8),
}

#[derive(Clone)]
pub struct Decision {
    pub kind: DecisionKind,
    pub resolver: Rc<dyn Fn(&mut GameState)>,
}
