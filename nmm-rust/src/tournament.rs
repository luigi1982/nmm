use crate::play::play;

//number of participants in the tournament
//needs to be a power of 2
const NUM_CONTENDERS: usize = 16;

//initializes the weights participating in the tournament
pub fn initialize() -> [[i16; 17]; NUM_CONTENDERS] {

    //initialize Petcu
    let petcu: [i16; 17] = [18, 26, 1, 6, 12, 7, 14, 43, 10, 8, 7, 42, 1086, 10, 1, 16, 1190];

    let mut contenders = [petcu ;NUM_CONTENDERS];

    //modify the petcu weights
    for i in 0..contenders.len()-1 {
        contenders[i][2] += i as i16
    }

    //return the contenders
    contenders
}

//play-off style tournament to determine best weight
//weights that are matched against each other face twice
//each of the weights is playing once as black once as white
//in case of a draw the weight with a higher seat moves on
pub fn tournament(contenders: [[i16; 17];NUM_CONTENDERS]) -> [i16; 17] {

    //initialize the winners
    let mut winners: Vec<[i16; 17]>;
    let mut contenders = contenders.to_vec();

    while contenders.len() > 1 {

        println!("{}", contenders.len());
        println!("{:?}", contenders);

        //clear the winners, new round!
        winners = Vec::new();

        // go through the match-ups
        for i in 0..contenders.len()/2 {

            println!("now playing {} vs. {}", 2*i, 2*i+1);
            println!("{} is now playing as white", 2*i);

            //first weight playing white
            let score_1 = play(contenders[2*i], contenders[2*i + 1], false, false, 1000, 1000, false);
            println!("{} is now plying as white", 2*i+1);

            //second weight playing white
            let score_2 = -play(contenders[2*i+1], contenders[2*i], false, false, 1000, 1000, false);

            //determine the winner and push him in the winner list
            // a win is 9 points
            // a draw will return the difference of pieces as score
            if score_1 + score_2 > 0 {
                println!("{} won with {} points", 2*i, score_1 + score_2);
                winners.push(contenders[2*i])
            } else {
                println!("{} won with {} points", 2*i+1, -(score_1 + score_2));
                winners.push(contenders[2*i+1])
            }
        }

        //remove the losers
        contenders = winners;
    }

    //return the winner
    contenders[0]
}

