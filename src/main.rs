// This file contains copyrighted assets owned by Greater Than Games.

#[macro_use]
extern crate simple_error;
extern crate crypto;
extern crate crossbeam;
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
    let args = App::new("Spirit Island Solver")
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
        .arg(Arg::with_name("threads")
            .short("j")
            .long("threads")
            .help("The number of concurrent threads to use.")
            .takes_value(true))
        .arg(Arg::with_name("solver")
            .long("solver")
            .help("The type of solver to use.")
            .possible_value("rng")
            .possible_value("simple")
            .takes_value(true))
        .arg(Arg::with_name("solver-take")
            .long("solver-take")
            .help("For solvers that only order decisions, how many to take. Use 0 to take all.")
            .takes_value(true))
        .arg(Arg::with_name("print-best")
            .long("print-best")
            .help("Attempted to print the best game sequence."))
        .get_matches();


    let mut seed: [u8; 32] = [0; 32];
    let mut hasher = Sha1::new();
    hasher.input_str(args.value_of("seed").unwrap_or("default"));
    hasher.result(&mut seed);
    let rng = Box::new(base::DeterministicChaCha::new(ChaChaRng::from_seed(seed)));

    let mut content: Vec<Box<dyn base::ContentPack>> = Vec::new();
    content.push(Box::new(CoreContent::new()));

    let mut spirits = Vec::new();
    if let Some(arg_spirits) = args.values_of("spirit") {
        for spirit in arg_spirits {
            match base::search_for_spirit(&content, spirit) {
                Some(spirit) => spirits.push(spirit),
                None => bail!("Spirit `{}` not found.", spirit),
            }
            
        }
    }

    let threads = args.value_of("threads").unwrap_or("4").parse::<usize>().unwrap();



    let adversary: Box<dyn base::AdversaryDescription> = Box::new(base::DefaultAdversaryDescription::new());

    let map = Box::new(base::make_map(&content, vec!["A"]));

    let description = Arc::new(base::GameDescription::new(content, adversary, spirits, map));
    let state = base::GameState::new(description, rng);

    let solver_name = args.value_of("solver").unwrap_or("simple");
    let solver_strategy =
        if solver_name == "simple" {
            let solver_take = args.value_of("solver-take").unwrap_or("2").parse::<u8>().unwrap();
            Ok(solve::SimpleDecisionMaker::new(solver_take) as Box<dyn solve::SolveStrategy>)
        } else if solver_name == "rng" {
            let solver_take = args.value_of("solver-take").unwrap_or("2").parse::<u8>().unwrap();
            let solver_rng = Box::new(base::DeterministicChaCha::new(ChaChaRng::from_seed(seed)));
            Ok(solve::StochasticDecisionMaker::new(solver_rng, solver_take) as Box<dyn solve::SolveStrategy>)
        } else {
            Err("Unknown solver.")
        }.unwrap();

    let mut solver = solve::SolveEngine::new(&state, solver_strategy);
    solver.print_first_best_game = args.is_present("print-best");
    
    solver.main(threads)?;

    Ok(())
}
