//here we implemented the nine mens morris game
//the structure stores the board, the number of pieces left for both players and the current ply

//the struct is able to compute the legal moves in all three stages and identify if a morris is being closed

//for an explanation on how the board is conceptualized we refer to our written part

//the moves are sored as arrays of length 3
//the first entry holds the piece to move and the second where it should go
//the third entry tells which piece should be removed if a mill was closed
//if the third entry holds a 24 no mill was closed
// if it holds 25 most of the time it is used to signify that the player ran out of moves

pub struct Game {

    //the board is represented by 3 circles which are consecutively appended to the list
    pub board: [i8; 24],
    //number of stones left on the field
    pub score_a: i8,
    pub score_b: i8,
    //plies
    pub plies: u64
}

impl Game {

    //initialize new game
    pub fn new() -> Self {
        Game{board: [0; 24], score_a: 9, score_b: 9, plies: 0}
    }

    // get fields with specified value
    pub fn get_fields_with(&self, number: i8) -> Vec<usize>{

        //initilaize the fields
        let mut fields: Vec<usize> = Vec::new();
        for i in 0..24 {

            //check for number on the given field
            if self.board[i] == number {
                fields.push(i);
            }
        }

        //return the found fields
        fields
    }

    pub fn get_moves_stage_two(&self, player: i8) -> Vec<[usize;2]> {
        
        //initialize possible moves
        let mut moves: Vec<[usize;2]> = Vec::new();

        //check wether the stones can move along the circles
        for circle in [0,8,16] {
            for i in 0..8 {
                if self.board[circle + i] == player {

                    //check wether the clockwise viewed predecessor is empty
                    let prev = circle + (i+7)%8;
                    if self.board[prev] == 0 {
                        moves.push([circle + i, prev]);
                    }  

                    //check wether the succesor is empty
                    let post = circle + (i+1)%8;
                    if self.board[post] == 0 {
                        moves.push([circle + i, post]);
                    }
                }
            }
        }

        //check wether the stones can be moved along the lines connecting the circles 
        //checking for the outer and middle circle if stones can be moved further to the center
        for node in [0,2,4,6,8,10,12,14] {
            if self.board[node] == player && self.board[node + 8] ==0 {
                moves.push([node, node +8]);
            }
        }
        //checking for the middle and inner circle if stones can be moved outwards
        for node in [8,10,12,14,16,18,20,22] {
            if self.board[node] == player && self.board[node - 8] == 0{
                moves.push([node, node-8]);
            }
        }
        moves
    }

    //returns the stones from the oposing player which can be removed
    //i.e. the stones currently not in a mill
    pub fn get_removables(&self, player: i8) -> Vec<usize> {
        
        let stones = self.get_fields_with(player);
        let mut removable: Vec<usize> = Vec::new();

        //check for each stone in stones if it is in a mill
        //if not add it to the removable stones
        for stone in stones {
            if !self.in_mill(player, stone) {
                removable.push(stone);
            }
        }
        removable
    }

    //returns all the possible moves for the specified player
    pub fn get_moves(&mut self, player: i8) -> Vec<[usize; 3]> {

        let mut moves: Vec<[usize; 3]> = Vec::new();

        //get the stones of the oposing playwer which are currently removable
        let removable = self.get_removables(-player);
            
        // jumping if player has 3 stones left
        if (self.score_a == 3 && player == 1) || (self.score_b == 3 && player == -1) {

            //finding positions of the stones
            let stones = self.get_fields_with(player);

            //finding free fields
            let free = self.get_fields_with(0);

            for stone in &stones {
                for field in &free {

                    //make the move to check if a mill is formed
                    self.make_move(player, &[*stone, *field, 24]);

                    //if a mill is formed add all the pieces which can be captured
                    if self.in_mill(player, *field) {
                        for op_stone in &removable {
                            moves.push([*stone, *field, *op_stone]);
                        }
                    } else {
                        moves.push([*stone, *field, 24]);
                    }

                    //undo the move again
                    self.undo_move(player, &[*stone, *field, 24]);
                }
            }

        // move stones if all 9 have been placed
        } else if self.plies >= 18 {
            let moves1 = self.get_moves_stage_two(player); 

            for m in &moves1 {

                //make the move to check if a mill is formed
                self.make_move(player, &[m[0], m[1], 24]); 

                //if a mill is formed add all the pieces which can be captured
                if self.in_mill(player, m[1]) {
                    for op_stone in &removable {
                        moves.push([m[0], m[1], *op_stone]);
                    }
                } else {
                    moves.push([m[0], m[1], 24]);
                } 
                //undo the move again
                self.undo_move(player, &[m[0], m[1], 24]);
            }

        //placing stones if less than 9 have been placed so far
        } else {
            let free = self.get_fields_with(0);
            for pos in &free {
                
                //make the move
                self.board[*pos] = player; 

                //check if it results in a mill
                if self.in_mill(player, *pos) {
                    for op_stone in &removable {
                        moves.push([*pos, *pos, *op_stone]);
                    }
                } else {
                    moves.push([*pos, *pos, 24]);
                } 
                //undo the move again
                self.board[*pos] = 0;
            } 
        }

        moves
    }

    //checking if stone is in mill
    pub fn in_mill(&self, player: i8, node: usize) -> bool {

        if node%2 == 0 {

            //get the predecessor and succesor viewed clock-wise on the circle
            let pre = if node%8 == 0 {node+7} else {node-1};
            let post = node+1;

            //return bool value if stone is in mill
            self.is_mill(pre, node, post, player) || self.is_mill(node, (node+8)%24, (node+16)%24, player)

        } else {
            //stone is located at the "corner" of a circle

            //determine the positions before the stone, viewed clockwise
            let pre1 = node-1;
            let pre2 = if pre1%8 == 0 {pre1+7} else {pre1-1};

            //determine the positions coming after the stone
            let post1 = if (node+1)%8 == 0 {node-7} else {node+1};
            let post2 = post1+1;

            //check if one of them is a mill
            //return bool value if stone is in mill
            self.is_mill(pre1, pre2, node, player) || self.is_mill(node, post1, post2, player)
        }
    }

    //checking for a mill
    pub fn is_mill(&self, node_1: usize, node_2: usize, node_3: usize, player:i8) -> bool {

        //return trtue if all fields hold a piece of the same color
        self.board[node_1] + self.board[node_2] + self.board[node_3] == 3*player
    }

    pub fn make_move(&mut self, player: i8, mov: &[usize; 3]) {

        //increase number of plies
        self.plies += 1;
        
        //make the move
        self.board[mov[0]] = 0;
        self.board[mov[1]] = player;

        //check if a mill was closed
        if mov[2] < 24 {

            //decrease the opponents score
            if player == 1 {
                self.score_b -= 1;
            } else {
                self.score_a -= 1;
            }

            //remove the piece
            self.board[mov[2]] = 0;
        }
    }

    pub fn undo_move(&mut self, player: i8, mov: &[usize; 3]) {

        //decrease number of plies
        self.plies -= 1;
        
        //undo the move
        self.board[mov[0]] = player;
        self.board[mov[1]] = 0;

        //check if a mill was closed
        if mov[2] < 24 {
            if player == 1 {
                self.score_b += 1;
            } else {
                self.score_a += 1;
            }
            self.board[mov[2]] = -player;
        }
    }

    //prints the board
    pub fn print(&self) {

        //transform the -1s to 2s for better readability
        let mut board = self.board;

        for field in &mut board {
            if *field == -1 {
                *field = 2;
            }
        }

        println!("({})----------------({})--------------({})", board[7], board[0], board[1]);
        println!("|                    |               |");
        println!("|      ({})---------({})-------({})      |", board[15],board[8], board[9]);
        println!("|        |           |         |      |");
        println!("|        |     ({})-({})-({})     |      |", board[23],board[16], board[17]);
        println!("|        |      |        |     |      |");
        println!("({})----({})----({})      ({})----({})----({})", board[6], board[14], board[22], board[18],board[10], board[2] );
        println!("|        |      |        |     |      |");
        println!("|        |     ({})-({})-({})     |      |", board[21], board[20], board[19]);
        println!("|        |           |         |      |");
        println!("|       ({})--------({})--------({})     |", board[13], board[12], board[11]);
        println!("|                    |                |");
        println!("({})----------------({})---------------({})", board[5], board[4], board[3]);       
    }

    //print explenation
    pub fn explain(&self) {

        println!("name of the positions");

        let mut board = self.board;

        for (i, field) in &mut board.iter_mut().enumerate() {
            *field = i as i8;
        }

        println!("({})----------------({})--------------({})", board[7], board[0], board[1]);
        println!("|                    |                       |");
        println!("|      ({})---------({})-------({})      |", board[15],board[8], board[9]);
        println!("|        |           |         |      |");
        println!("|        |     ({})-({})-({})     |      |", board[23],board[16], board[17]);
        println!("|        |      |        |     |      |");
        println!("({})----({})----({})      ({})----({})----({})", board[6], board[14], board[22], board[18],board[10], board[2] );
        println!("|        |      |        |     |      |");
        println!("|        |     ({})-({})-({})     |      |", board[21], board[20], board[19]);
        println!("|        |           |         |      |");
        println!("|       ({})--------({})--------({})     |", board[13], board[12], board[11]);
        println!("|                    |                |");
        println!("({})----------------({})---------------({})", board[5], board[4], board[3]);  
        
        println!("first stage: type the field as a single number and enter return");
        println!("second and third stage: type the field of the stone you wish to move and where it should go seperated by a space and enter return");
        println!("captures: if you closed a mill you are asked to remove a piece, type the number of the field on which the piece is located you wish to remove and enter return");
    }
}

pub fn pre(node: usize) -> usize {
    //determine the positions before the stone, viewed clockwise
    if node%8 == 0 {node+7} else {node-1}
}

pub fn post(node:usize) -> usize {
    //determine the positions coming after the stone
    if (node+1)%8 == 0 {node-7} else {node+1}
}