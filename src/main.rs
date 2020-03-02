// This file contains copyrighted assets owned by Greater Than Games.

#[macro_use]
extern crate simple_error;
extern crate crypto;
extern crate clap;

use std::error::Error;
use std::sync::{Arc};
use rand::prelude::*;
use rand_chacha::{ChaChaRng};
use self::crypto::digest::Digest;
use self::crypto::sha1::Sha1;
use clap::{Arg, App};

mod base;
mod core;
mod solve;

use crate::core::{CoreContent};

fn main() -> Result<(), Box<dyn Error>> {
    let matches = App::new("Spirit Island Solver")
        .version("0.1.0")
        .author("Mason Bially <mason.bially@gmail.com>")
        .about("A spirit island solver and simulator.")
        .arg(Arg::with_name("spirit")
            .short("s")
            .long("spirit")
            .help("Selects a spirit to solve the game with.")
            .takes_value(true)
            .multiple(true))
        .arg(Arg::with_name("seed")
            .long("seed")
            .help("Sets the seet of the random system for reproducible results..")
            .takes_value(true))
        .get_matches();

    let mut seed: [u8; 32] = [0; 32];
    let mut hasher = Sha1::new();
    hasher.input_str(matches.value_of("seed").unwrap_or("default"));
    hasher.result(&mut seed);
    let rng = Box::new(base::DeterministicChaCha::new(ChaChaRng::from_seed(seed)));

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

    let adversary: Box<dyn base::AdversaryDescription> = Box::new(base::DefaultAdversaryDescription::new());

    let map = Box::new(base::make_map(&content, vec!["A"]));

    let description = Arc::new(base::GameDescription::new(content, adversary, spirits, map));
    let state = base::GameState::new(description, rng);

    let mut solver = solve::SolveEngine::new(&state,
        solve::SimpleDecisionMaker::new());

    solver.main()?;

    Ok(())
}
