use super::{
    effect::{Effect}
};

#[derive(Copy, Clone)]
pub enum DecisionKind {
    DamageToInvaders(u16),
    PlaceBlight(u8),
}

#[derive(Clone)]
pub struct Decision {
    pub kind: DecisionKind,
}
