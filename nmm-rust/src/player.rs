use std::io;
use crate::nmm::Game;

//this script allows to play against the engine via console input

//first stage           : type the field as a single number and return
//second and third stage: type the field of the stone you wish to move and where it should go seperated by a space and return
//captures              : if you closed a mill you are asked to remove a piece, 
//                        type the number of the field on which the piece is located you wish to remove and return

pub fn player(game: &mut Game, color: i8) -> [usize; 3] {


    //check if the player ran out of legal moves
    if game.get_moves(color).is_empty() {
        return [0,0,25];
    };

    loop{


        if game.plies >= 18 {
            println!("Move a piece!");
        } else {
            println!("Place a piece!");
        }


        //read the player input from the console

        let mut next_move = String::new();

        io::stdin()
            .read_line(&mut next_move)
            .expect("Failed to read line");

        let next_move: Vec<&str> = next_move.split_whitespace().collect();

        //check for stage 1
        if next_move.len() == 1 && game.plies <= 17 {

            //get the possible moves
            let moves = game.get_fields_with(0);
            let next_move: usize = next_move[0].parse().unwrap();

            //check if player move is legal
            if moves.contains(&next_move) {

                //check if a mill is being closed
                let to_be_removed = check_mill(game, color, &[next_move, next_move]);
                return [next_move, next_move, to_be_removed];

            } else {

                //if not a valid move ask for a new one
                println!("Not a valid move!");
                continue;
            }

        //check for stage 3
        } else if next_move.len() == 2 && game.plies >= 18 && ((color == 1 && game.score_a == 3) || (color == -1 && game.score_b == 3)) {

            //get the possible moves
            let pieces = game.get_fields_with(color);
            let free = game.get_fields_with(0);
            let next_move: [usize; 2] = [next_move[0].parse().unwrap(), next_move[1].parse().unwrap()];

            println!("{:?}", free);
            println!("{:?}", pieces);

            //check if player move is legal
            if pieces.contains(&next_move[0]) && free.contains(&next_move[1]) {

                //check if a mill is being closed
                let to_be_removed = check_mill(game, color, &next_move);
                return [next_move[0], next_move[1], to_be_removed];
            } else {

                //if not a valid move ask for a new one
                println!("Not a valid move!");
                continue;
            }

        //check for stage 2
        } else if next_move.len() == 2 && game.plies >= 18 {

            //get the possible moves
            let moves = game.get_moves_stage_two(color);
            let next_move: [usize; 2] = [next_move[0].parse().unwrap(), next_move[1].parse().unwrap()];

            //check if player move is legal
            if moves.contains(&next_move) {

                //check if a mill is being closed
                let to_be_removed = check_mill(game, color, &next_move);
                return [next_move[0], next_move[1], to_be_removed];
            } else {

                //if not a valid move ask for a new one
                println!("Not a valid move!");
                continue;
            }
        } else {

            //if not a valid move ask for a new one
            println!("Not a valid move!");
            continue;
        }
    }
}

//function checks if the player closed a mill in his last move
//if yes the player is asked which piece to remove
fn check_mill(game: &mut Game, color: i8, next_move: &[usize; 2]) -> usize {

    //check whether mill was closed by the player
    game.make_move(color, &[next_move[0], next_move[1], 24]);
    let is_mill = game.in_mill(color, next_move[1]);
    game.undo_move(color, &[next_move[0], next_move[1], 24]);

    //if yes ->
    if is_mill {

        //get the positions of the removable pieces of the oppponent
        let removable = game.get_removables(-color);
        println!("Remove one of your opponents pieces!");

        loop {
            
            //read the player input from the console
            let mut to_be_removed = String::new();

            io::stdin()
                .read_line(&mut to_be_removed)
                .expect("Failed to read line");

            let to_be_removed: Vec<&str> = to_be_removed.split_whitespace().collect();

            //check if correct input has been provided
            if to_be_removed.len() == 1 {

                let to_be_removed: usize = to_be_removed[0].parse().unwrap();

                //check if the move is legal
                if removable.contains(&to_be_removed) {
                    return to_be_removed;
                } else {
                    //if not a valid move ask for a new one
                    println!("Not a valid move!");
                    continue;
                }
            } else {
                //if not a valid move ask for a new one
                println!("Not a valid move!");
                continue;
            }
        
        }

    }

    //return 24 if no mill has been closed
    24
}