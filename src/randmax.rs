
const TESTS: i32 = 10;
pub fn get_game_sim(gamestate: &State, my_turn:i32) -> f32 {
    let mut k = 0;
    for _ in 0..TESTS {
        k += randmax(&mut(gamestate.clone()), gamestate.current_team());
    }
    return (my_turn * k) as f32;
}

fn randmax(gamestate:&mut State, my_team:Team) -> i32 {
    if gamestate.is_over() 
    {
        if gamestate.winner().is_none() {
            return 0
        }
        else if gamestate.winner().unwrap().index() == my_team.index() {
            return  1;
        }
        else {
            return -1;
        };
    } 
    let curteam = gamestate.current_team();
    let possible_moves = gamestate.possible_moves();
    let m =  *possible_moves.choose(&mut rand::thread_rng()).unwrap();
    let f = gamestate.perform(m);
    let d = randmax(gamestate, my_team);
    gamestate.undo_move(m, f, curteam);
    return d;
}