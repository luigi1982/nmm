use crate::nmm::Game;
//use crate::alpha_beta_nmm::abs;
use crate::principle_variation_search::Pvs;
use crate::player::player;
use crate::iterative_deepening::iterative_deepening;
use std::io::{stdout, Write};
use std::time::Instant;

const DOME: bool = false;

pub fn play(weights_a: [i16; 17], weights_b: [i16; 17], human_a: bool, human_b: bool, time_a: u128, time_b: u128, print_info: bool) -> i8 {

    //initialize the pvs instances
    let mut pvs_a = Pvs::new(1, weights_a);
    let mut pvs_b = Pvs::new(-1, weights_b);

    //initialize the game
    let mut game = Game::new();

    //white plays first
    let mut color: i8 = 1;
    let mut value: i16 = 0;
    let mut next_move: [usize; 3];

    //initialize the pvlines for black and white
    let mut pvline_a = vec![[8, 8, 24], [12, 12, 24], [4, 4, 24], [0, 0, 24], [15, 15, 24], [9, 9, 24], [13, 13, 24], [14, 14, 24], [0,0,24], [0,0,25]];
    let mut pvline_b = vec![[9, 9, 24], [1, 1, 24], [0, 0, 24], [10, 10, 24], [2, 2, 24], [5, 5, 24], [14, 14, 24], [3, 3, 24], [0,0,24]];
    
    //measure the time
    let mut times: Vec<u128> = Vec::new();

    const BACKSPACE: char = 8u8 as char;
    let mut stdout = stdout();

    if human_a || human_b {
        game.explain();
    }

    //play as long as no player has lost
    while game.score_a > 2 && game.score_b > 2 && game.plies/2 < 100 {

        //get the next move
        if color == 1 {

            if human_a {
                next_move = player(&mut game, color);
            } else {

                //start the clock
                let start_time = Instant::now();

                //println!("pvline len {}", pvline_a.len());

                (value, pvline_a) = iterative_deepening(&mut pvs_a, time_a, &mut game, 5, pvline_a);
                //(value, pvline_a) = pvs_a.search(&mut game, color, 9, i16::MIN+1, i16::MAX, &pvline_a[..]);

                //stop the clock!
                let ms = start_time.elapsed().as_millis();
                times.push(ms);

                //pvline_a.extend(vec![[0,0,25];2]);
                //pvline_a = pvline_a[2..].to_vec();

                //next move is the first move from the pvline
                next_move = pvline_a[0];
            }

        } else if human_b {
            next_move = player(&mut game, color);
        } else {
            (value, pvline_b) = iterative_deepening(&mut pvs_b, time_b, &mut game, 5, pvline_b);
            //next move is the first move from the pvline
            next_move = pvline_b[0];
        }

        //check if the player ran out of moves
        if next_move[2] == 25 {
            println!("Gameover player {color} ran out of moves");
            println!("times: {:?}", times);
            return -color*9;
        }

        //execute the next move
        game.make_move(color, &next_move);

        // if print_info is true print the board state and other information to the console
        if print_info {

            if color == 1 {
                println!("-------- round {}, score {}-{}, evaluation {value} -----------", game.plies/2 + 1, &game.score_a, &game.score_b);
            }

            if DOME {
                next_move = [dome(next_move[0]), dome(next_move[1]), dome(next_move[2])];
            }

            if next_move[0] == next_move[1] {
                println!("player {} placed stone on field {}", color, next_move[0]);
            } else {
                println!("player {} moved stone from field {} to {}", color, next_move[0], next_move[1]);
            }

            if next_move[2] < 24 {
                println!("player {} removes: {}", color, next_move[2]);
            }

            if color == -1 {
                game.print();
            }

        } else if color == 1 {
            print!("{}\rround {}, score {}-{}", BACKSPACE, game.plies/2 + 1, &game.score_a, &game.score_b);
            stdout.flush().unwrap();
        }

        //change player for next move
        color = -color;
    }

    //print the oucome of the match
    println!("\rmatch ended after {} plies with score {}-{}", game.plies/2, &game.score_a, &game.score_b);

    println!("times: {:?}", times);

    //return the result for tournament.rs
    if game.score_a < 3 {
        return -9;
    } else if game.score_b < 3 {
        return 9;
    }
    
    game.score_a - game.score_b
}

//function transforms the board positions to the positions of my project partner
fn dome(field: usize) -> usize {
    if field == 7 {0} else if field == 15 {8} else if field == 23 {16} else if field == 24 {24} else {field + 1}
} 