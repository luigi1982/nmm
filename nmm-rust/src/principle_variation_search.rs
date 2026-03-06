use crate::nmm::Game;
use crate::move_ordering::{score_moves, sort_moves};
use crate::heuristics::{big_heuristic_weights};

//PVS struct
// in color is stored which side it is playing
// weights store the weights for the heuristic
// killer_moves store the moves for the killer move heuristic for the move ordering

pub struct Pvs {
    pub color: i8,
    weights: [i16; 17],
    pub killer_moves: Vec<[[usize;3];2]>
}

impl Pvs {

    //creates a new instance of PVS using specified color and weights
    pub fn new(color: i8, weights: [i16; 17]) -> Self {

        //initialize killer moves
        let killer_moves = vec![[[0,0,25]; 2]; 1000];

        Self{color, weights, killer_moves}
    }
    
    // evaluation, used when a leaf node is reached in the game tree 
    fn evaluate(&self, game: &mut Game, color: i8, mov: &[usize; 3], winning_pos: i8) -> i16 {
    
        let max_est = big_heuristic_weights(game, color, mov, winning_pos, self.weights);
        let min_est = big_heuristic_weights(game, -color, &[0,0,25], winning_pos, self.weights);
        max_est - min_est
    }

    // here is the actual search algorithm

    //arguments to provide
    //game - the game instance
    //color - player next to move
    //depth - remaining search depth
    //alpha, beta - search window size
    //old_pvline - pvline from previous iteration for move ordering
    pub fn search(&mut self, game: &mut Game, color: i8, depth: u16, mut alpha: i16, beta:i16, old_pvline: &[[usize;3]]) -> (i16, Vec<[usize; 3]>) {

        //create new line
        let mut new_line: Vec<[usize; 3]>  = Vec::new();
        let mut pvline: Vec<[usize; 3]> = Vec::new();
    
        // compute current legal moves
        let mut moves = game.get_moves(color);
    
        // check if the current color has legal moves or the game is lost
        if moves.is_empty() {
            return (self.evaluate(game, color, &[0,0,25], -color), vec![[0,0,25]]);
        }
    
        //order the moves
        let score = score_moves(self, game, &moves, &old_pvline[0], color);
        moves = sort_moves(moves, score);
    
        let mut value: i16;
        let mut first_move = true;
    
        for next_move in moves {
    
            // make the move
            game.make_move(color, &next_move);

            //check whether we evaluate here or move further down in the game tree
            if depth > 0 && old_pvline.len() >= 2 {

                //if this is the first child do a full window search
                if first_move {
                    (value, pvline) = self.search(game, -color, depth - 1, -beta, -alpha, &old_pvline[1..]);

                    //note first child has been searched
                    first_move = false;
                    value = -value;

                //else do a zero window search
                } else {
                    (value, pvline) = self.search(game, -color, depth - 1, -alpha - 1, -alpha, &old_pvline[1..]);
                    value = -value;

                    //if zero window search returned alpha can potentially be raised conduct again a full window search
                    if alpha < value && value < beta {
                        (value, pvline) = self.search(game, -color, depth - 1, -beta, -alpha, &old_pvline[1..]);
                        value = -value;
                    }
                }
            } 
            //if depth 0 is reached determine a heuristic value of the board state
            else {
                value = self.evaluate(game, color, &next_move, 0);
            }
            
            // undo the move
            game.undo_move(color, &next_move);
            
            //check wether a better move has been found
            if value > alpha {
                alpha = value;

                //update the pvline
                new_line = Vec::new();
                new_line.push(next_move);
                new_line.extend(pvline.clone());
            }

            //check for a beta-cutoff
            if alpha >= beta {

                //add the current move to killer moves
                //and remove the oldest
                self.killer_moves[game.plies as usize][0] = self.killer_moves[game.plies as usize][1];
                self.killer_moves[game.plies as usize][1] = next_move;
                break;
            }
        }
    
        // return the value and the pvline
        (alpha, new_line)
    }

}