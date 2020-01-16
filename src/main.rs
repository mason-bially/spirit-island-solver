
#[macro_use]
extern crate simple_error;
extern crate crypto;
extern crate clap;

use std::error::Error;
use rand::prelude::*;
use rand_chacha::{ChaChaRng};
use self::crypto::digest::Digest;
use self::crypto::sha1::Sha1;
use clap::{Arg, App, SubCommand};

mod base;
mod core;

use crate::core::{CoreContent};

fn main() -> Result<(), Box<dyn Error>> {
    let matches = App::new("Spirit Island Solver")
        .version("0.1.0")
        .author("Mason Bially <mason.bially@gmail.com>")
        .about("A spirit island solver and simulator.")
        .arg(Arg::with_name("spirit")
            .short("s")
            .short("spirit")
            .help("Selects a spirit to solve the game with.")
            .takes_value(true)
            .multiple(true))
        .get_matches();



    let input: &[_] = &[1, 2, 3, 4];
    let mut seed: [u8; 32] = [0; 32];
    let mut hasher = Sha1::new();
    hasher.input(input);
    hasher.result(&mut seed);
    let rng = Box::new(ChaChaRng::from_seed(seed));

    let mut content: Vec<Box<dyn base::ContentPack>> = Vec::new();
    content.push(Box::new(CoreContent::new()));

    let mut spirits = Vec::new();
    if let Some(arg_spirits) = matches.values_of("spirit") {
        for spirit in arg_spirits {
            match base::search_for_spirit(&content, spirit) {
                Some(spirit) => spirits.push(spirit),
                None => bail!("Spirit `{}` not found.", spirit),
            }
            
        }
    }

    let mut adversary: Box<dyn base::Adversary> = Box::new(base::DefaultAdversary::new());

    let mut state = base::GameState::new(rng, content, adversary, spirits);

    while !state.is_over()
    {
        state.step();
    }

    match state.step {
        base::GameStep::Victory => { println!("Vectory!    {}", state.game_over_reason.unwrap()); }
        base::GameStep::Defeat => {  println!("Defeat :(   {}", state.game_over_reason.unwrap()); }
        _ => panic!()
    }

    Ok(())
}
