use crate::nmm::Game;
use crate::heuristics::big_heuristic;

//here alpha-beta search is implemented

pub fn abs(mut game: &mut Game, color: i8, depth: u16, alpha: i16, beta: i16) -> (i16, [usize; 3], u8) {

    let mut current_d: u8 = 255;
    let mut d: u8 = 0;
    
    if color == 1 {

        //initialize the value
        let mut value = alpha;
        let mut pot_val = 0; 

        let mut max_move: [usize; 3] = [0,0,25];

        // compute current legal moves
        let moves = &game.get_moves(1);
        
        // check if the current player has legal moves
        if moves.is_empty() {
            return evaluate(game, -color, &[0,0,25], true);
        }

        //increase the number of plies
        game.plies += 1;

        for next_move in moves {

            // make the move
            game.make_move(1, next_move);

            //evaluate the game after the move
            if depth > 0 {
                (pot_val, _,d) = abs(game, -color, depth-1, value, beta);
            } else {
                (pot_val, _,d) = evaluate(game, color, next_move, false);
            }

            //undo the move
            game.undo_move(1, next_move);

            // searches for the best potential move
            // checks if currently evaluated move is better
            if value < pot_val || (value == pot_val && current_d < d) {
                current_d = d;
                value = pot_val;
                max_move = *next_move;
                if value >= beta {break;}
            }
        }
        game.plies -= 1;
        (value, max_move, current_d)

    } else {

        //logic for the min player

        //initialize the value
        let mut value = beta;
        let mut pot_val = 0;

        //place stone if not 9 stones have been placed yet

        let mut min_move: [usize; 3] = [0,0,25];

        // compute if the current player has legal moves
        let moves = &game.get_moves(-1);

        // check if the current player has legal moves
        if moves.is_empty() {
            return evaluate(game, -color, &[0,0,25], true);
        }

        for next_move in moves {

            // make the move
            game.make_move(-1, next_move);

            //evaluate the move
            if depth > 0 {
                (pot_val, _, d) = abs(game, -color, depth-1, alpha, value);
            } else {
                (pot_val, _, d) = evaluate(game, color, next_move, false);
            }

            //undo the move
            game.undo_move(-1, next_move);

            // searches for the minimizing move
            if value > pot_val || (value == pot_val && current_d < d) {
                current_d = d;
                value = pot_val;
                min_move = *next_move;
                if value <= alpha {break;}
            }
        }
        (value, min_move, current_d)
    }
}

//evaluation for the 
fn evaluate(game: &mut Game, color: i8, mov: &[usize; 3], winning_pos: bool) -> (i16, [usize; 3], u8) {
    let max_est = big_heuristic(game, color, mov, winning_pos);
    let min_est = big_heuristic(game, -color, &[0,0,25], winning_pos);
    let estimate = i16::from(color)*(max_est - min_est);
    return (estimate.into(), [0,0,25], 0);
}