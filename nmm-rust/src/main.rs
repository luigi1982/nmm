//Paul Hasenbusch

//implementation of a nine men´s mmorris engine.
//it pairs principle variation search with heuristic from S. Petcu and S. Holban
//the principle variation search is further improved using various move ordering heuristics.

//mod minicolor_nmm;
mod nmm;
//mod alpha_beta_nmm;
mod principle_variation_search;
mod player;
//mod alpha_beta_parallel;
mod move_ordering;
mod heuristics;
mod iterative_deepening;
mod play;
mod tournament;
//mod pvs_no_struct;


use crate::tournament::{tournament, initialize};
use crate::play::play;
use std::env;

const TOURNAMENT: bool = false;

fn main() {

    // Retrieve command-line arguments.
    let args: Vec<String> = env::args().collect();

    // Check if the correct number of command-line arguments is provided.
    if args.len() != 5 {
        eprintln!("Usage: {} <filename>", args[0]);
        std::process::exit(1);
    }

    if TOURNAMENT {
        let contenders = initialize();
        let winner = tournament(contenders);

        println!("{:?}", winner);
    }

    //parse the command line inputs
    let human_a: bool = args[1].parse::<usize>().unwrap() != 0;
    let human_b: bool = args[2].parse::<usize>().unwrap() != 0;
    let time_a: u128 = args[3].parse::<u128>().unwrap();
    let time_b: u128 = args[4].parse::<u128>().unwrap();
    
    let petcu = [18, 26, 1, 6, 12, 7, 14, 43, 10, 8, 7, 42, 3086, 10, 1, 16, 1190];
    let winner = [18, 26, 13, 6, 12, 7, 14, 43, 10, 8, 7, 42, 3086, 10, 1, 16, 1190];
    play(petcu, winner, human_a, human_b, time_a, time_b, true);
}
