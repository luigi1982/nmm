use crate::nmm::Game;
use crate::nmm::{post, pre};

//implementaion of the heuristic of petcu
//the heuristic is used to evaluate the board state when a leaf node is reached in the game tree when performing pvs or alpha beta search

//function counting the relations petcu suggested
pub fn count_all(game: &mut Game, player: i8, mov: &[usize; 3]) -> (i16, i16, i16, i16, i16, i16, i16, i16) {

    //number of morrises
    let mut morris_n: i16 = 0;
    //number of blocked pirces of opponent
    let mut blocked_n: i16 = 0;
    //number of pieces on the game.board
    let pieces_n: i16 = if player == 1 {i16::from(game.score_a)} else {i16::from(game.score_b)};
    //number 2 piece configuaeations
    let mut two_piece_n: i16 = 0;
    //number of 3 piece configurations
    let mut three_piece_n: i16 = 0;
    //number of double morrises
    let mut dbl_morris_n: i16 = 0;
    //closed
    let closed: i16 = if mov[2] < 24 {1} else {0};

    //check if a morris was opened
    let mut opened: i16 = 0;
    if mov[2] < 25 {
        game.undo_move(player, mov);
        opened = if game.in_mill(player, mov[0]) {1} else {0};
        game.make_move(player, mov);
    }

    //going through all fileds
    for node in 0..24 {

        if node%2 == 0 {

            //field is located at a crossroad

            //compute the predecessor and successor on the circle
            let prev = pre(node);
            let next = post(node);

            //check number of morrises
            // check for morris on the circles
            if game.is_mill(prev, node, next, player) {
                morris_n += 1;

                // check for double morris
                // a double morris allows the player to close a morris in every turn
                if node >= 8 {
                    //check for config on the outer and middle circle
                    //check if by moving current node one circle outwards a morris is closed
                    if game.board[node-8] == 0 && game.board[prev-8] + game.board[next - 8] == 2*player {
                        dbl_morris_n += 1;
                    }
                }
                if node <= 14 {
                    //check for config on the middle and inner circles
                    //check if by moving current node one circle inwards a morris is closed
                    if game.board[node+8] == 0 && game.board[prev+8] + game.board[next + 8] == 2*player {
                        dbl_morris_n += 1;
                    }
                }  

                //check if by moving next one field clock-wise a morris is closed
                if game.board[post(next)] == 0 && game.board[(post(next)+8)%24] + game.board[(post(next)+16)%24] == 2*player {
                    dbl_morris_n += 1;
                }

                //check if by moving prev one field counter-clock-wise a morris is closed
                if game.board[pre(prev)] == 0 && game.board[(pre(prev)+8)%24] + game.board[(pre(prev)+16)%24] == 2*player {
                    dbl_morris_n += 1;
                }
            } 
            //check for two piece config
            else if game.board[node] + game.board[prev] + game.board[next] == 2*player{
                two_piece_n += 1;
            }

            // check for vertical morris i.e. on the four lines connecting the circles
            if  node <= 6 && game.is_mill(node, (node+8)%24, (node+16)%24, player) {
                morris_n += 1;
                for circle in [0, 8, 16] {
                    //check if moving counter-clockwise gives morris
                    if game.board[prev + circle] == 0 && game.board[pre(prev + circle)] + game.board[pre(pre(prev + circle))] == 2* player {
                        dbl_morris_n += 1;
                    }
                    //check if moving clockwise gives morris 
                    if game.board[next + circle] == 0 && game.board[post(next + circle)] + game.board[post(post(next + circle))] == 2* player {
                        dbl_morris_n += 1;
                    }
                }
            } else {

                //check for three piece config, i.e. a config where the opposing player is not able to stop the formation of a mill
                //check for L formation of the players stones going inwards
                //and check if the opposing player does not occupy the fields needed to complete the morris
                if node <= 14 && game.board[node] + game.board[node + 8] + game.board[(node + 16)%24] == 2*player && game.board[prev] + game.board[next] == player && (node <=6 || game.board[(node + 16)%24] == 0) {
                    three_piece_n += 1;
                //check for L formation going outwards
                } 
                if node >= 8 && game.board[node] + game.board[node - 8] + game.board[(node + 8)%24] == 2*player && game.board[prev] + game.board[next] == player && (node >= 16 || game.board[(node + 8)%24] == 0){
                    three_piece_n += 1;
                }

                // check for two piece config
                if node <= 6 && game.board[node] + game.board[node + 8] + game.board[node + 16] == 2*player {
                    two_piece_n += 1;
                }
            }

            //checking if piece is blocked
            //check if the fields left and right are occupied
            if (game.board[node] == -player && game.board[prev].abs() + game.board[next].abs() == 2) &&

                //for node on outmost circle
                ((node <= 6 && game.board[node+8] != 0) ||
                //inner circle
                (node >= 16 && game.board[node-8] != 0) ||
                //middle circle
                ((8..=14).contains(&node) && game.board[node-8].abs() + game.board[node+8].abs() == 2))  {

                    blocked_n += 1;
            }
            
        } else {
            //stone is located at the "corner" of a circle

            //determine the positions before the stone, viewed clockwise
            let prev = pre(node);
            let prev_prev = pre(prev);

            //determine the positions coming after the stone
            let next = post(node);
            let next_next = post(next);

            //check for three piece config
            // second condition only satisfied when both 0
            if game.is_mill(prev, next, node, player) && game.board[prev_prev]*game.board[next_next] == game.board[prev_prev]+game.board[next_next] {
                three_piece_n += 1;
            }
            //checking if piece is blocked
            else if game.board[node] == -player && game.board[prev].abs() + game.board[next].abs() == 2 {
                blocked_n += 1;
            }
        }
    }
    (morris_n, blocked_n, pieces_n, two_piece_n, three_piece_n, dbl_morris_n ,opened, closed)
}


//weighting the relation returned from count all according to petcu
pub fn big_heuristic(game: &mut Game, player: i8, mov: &[usize; 3], winning_pos: bool) -> i16 {

    //get the relations
    let (morris_n, blocked_n, pieces_n, two_piece_n, three_piece_n, dbl_morris_n ,opened, closed) = count_all(game, player, mov);

    //initialize score
    let mut score: i16 = 0;

    //check if the game has been won
    if (game.score_a < 3 && player == -1) || (game.score_b < 3 && player == 1) || winning_pos {
        if (game.score_a == 3 && player == 1) || (game.score_b == 3 && player == -1)  {
            score += 1190;
        } else {
            score += 1086;
        }
    }

    //check if we are in the opening stage
    if game.plies <= 17 {
        score += 18*closed + 26*morris_n + blocked_n + 6*pieces_n + 12*two_piece_n + 7*three_piece_n;
    }

    //if we are in stage 3
    else if (game.score_a == 3 && player == 1) || (game.score_b == 3 && player == -1)  {
        score += 10*two_piece_n + three_piece_n + 16*closed;

    //or stage 2
    } else {
        score += 14*closed + 43*morris_n + 10*blocked_n + 8*pieces_n + 7*opened + 42*dbl_morris_n;
    }
    score
}

//weighting the relation returned from count all using custom weights
pub fn big_heuristic_weights(game: &mut Game, player: i8, mov: &[usize; 3], winning_pos: i8, weights: [i16; 17]) -> i16 {

    //get the relations
    let (morris_n, blocked_n, pieces_n, two_piece_n, three_piece_n, dbl_morris_n ,opened, closed) = count_all(game, player, mov);

    //initialize score
    let mut score: i16 = 0;

    //check if the game has been won
    if (game.score_a < 3 && player == -1) || (game.score_b < 3 && player == 1) || winning_pos == player {
        if (game.score_a == 3 && player == 1) || (game.score_b == 3 && player == -1)  {
            score += weights[3];
        } else {
            score += weights[16] - weights[9]*(game.plies - 15) as i16;
        }
    }

    //check if we are in the opening stage
    if game.plies <= 17 {
        score += weights[0]*closed + weights[1]*morris_n + weights[2]*blocked_n + weights[3]*pieces_n + weights[4]*two_piece_n + weights[5]*three_piece_n;
    }

    //if we are in stage 3
    else if (game.score_a == 3 && player == 1) || (game.score_b == 3 && player == -1)  {
        score += weights[13]*two_piece_n + weights[14]*three_piece_n + weights[15]*closed;

    //or stage 2
    } else {
        score += weights[6]*closed + weights[7]*morris_n + weights[8]*blocked_n + weights[9]*pieces_n + weights[10]*opened + weights[11]*dbl_morris_n;
    }

    //return the score
    score
}