use crate::principle_variation_search::Pvs; 
use std::time::Instant;
use crate::nmm::Game;

//iterative deepening, used to control the time spend searching
//searches for roughly the provided time t_limit

pub fn iterative_deepening(pvs: &mut Pvs, t_limit: u128, game: &mut Game, start_depth: usize, mut pvline: Vec<[usize; 3]>) -> (i16, Vec<[usize; 3]>) {
    
    let mut t_last_search: u128 = 0;
    let mut value: i16 = 0;
    let mut depth = start_depth as u16;

    //cut off the first two moves as they are already in the past
    if game.plies > 1 {
        pvline = pvline[2..].to_vec();
    }

    //extend the pvline to fit the search depth
    if pvline.len() <= start_depth+2 {
        pvline.extend(vec![[0,0,25];start_depth+2 - pvline.len()]);
    }

    //start the clock
    let start_time = Instant::now();
    
    //check if the time limit has been exceeded else continue with the next search
    while  start_time.elapsed().as_millis() + t_last_search < t_limit && depth < 30 {
        
        //perform pvs search
        (value, pvline) = pvs.search(game, pvs.color, depth, i16::MIN+1, i16::MAX, &pvline[..]);

        //get the time needed for the last search
        t_last_search = start_time.elapsed().as_millis() - t_last_search;

        //increment the depth
        depth += 1;

        //extend the pvline to fit the search depth
        pvline.extend(vec![[0,0,25]; depth as usize + 1 - pvline.len()]);
    }

    //return value and pvline
    (value, pvline.clone())
}