use crate::heuristics::big_heuristic;
use crate::nmm::Game;
use crate::principle_variation_search::Pvs;

//as explained in our written summary PVS profits from a good prior move ordering
//here we tried some approaches to rank the moves

//ranking the moves based on various heuristics
pub fn score_moves(pvs: &Pvs, game: &mut Game, moves: &Vec<[usize; 3]>, pv_move: &[usize; 3], color: i8) -> Vec<i16> {
    
    let mut scores: Vec<i16> = Vec::new();
    let mut score: i16;

    for cur_move in moves {

        //initialize score as 0
        score = 0;

        //pv
        if cur_move == pv_move {     
            score += 100;
        }

        //killer move
        for i in 0..2 {
            if cur_move == &pvs.killer_moves[game.plies as usize][i] {
                score += (i+1) as i16 *25;
            }
        }

        //capture
        if cur_move[2] < 24 && game.plies >= 18 {
            score += 1;
        }

        //heuristic one side
        if game.plies <= 5 {
            game.make_move(color, cur_move);
            score += big_heuristic(game, color, cur_move, false);
            game.undo_move(color, cur_move);
        }

        scores.push(score);
    }

    scores
}

//sorting the moves based on their ranking
pub fn sort_moves(mut moves: Vec<[usize; 3]>, mut score: Vec<i16>) -> Vec<[usize; 3]> {

    //insertion sort
    for i in 0..moves.len() {
        let mut j = i;
        while j < moves.len()-1 && score[j + 1] > score[j] {
            moves.swap(j + 1, j);
            score.swap(j + 1, j);
            j += 1;
        }
    }

    //bubble sort
    /*for i in 0..moves.len() {
        for j in i+1..moves.len() {
            if score[i] < score[j] {
                moves.swap(i, j);
                score.swap(i, j);
            }
        }
    }*/

    moves
}

