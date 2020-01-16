use super::{
    effect::{Effect}
};

pub trait Decision {
    fn enumerate_choices() -> Vec<Box<dyn Effect>>;
}