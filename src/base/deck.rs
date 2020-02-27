
/*
    In general decks of cards organized as Vecs will follow physical card rules:

    * Push a card means put it on top of the stack.
    * Pop means take off the top of the stack.

    This does however mean that the first card when iterating is the _bottom_ card which might be the last one poped.
*/

mod invader;
mod fear;

pub use self::invader::*;
pub use self::fear::*;

