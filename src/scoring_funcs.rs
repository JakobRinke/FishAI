use crate::game::{State, Team, Move};




pub fn get_move_num(gamestate:&State, my_turn:i32) -> i32 {
    my_turn * ((gamestate.possible_moves().len() - gamestate.opponent_moves().len()) as i32)
}

pub fn get_fish_dif(gamestate:&State, my_turn:i32) -> i32 {
    my_turn * ( (gamestate.fish(gamestate.current_team()) - 
                gamestate.fish(gamestate.current_team().opponent())) as i32)
}

pub fn get_turn(gamestate:&State) -> i32 {
    gamestate.turn() as i32
}

pub fn get_moveable_peguins(gamestate:&State, team:Team) -> i32 {
    let pingus = gamestate.pieces_of(team);
    let mut d = 0;
    for position in pingus {
        if gamestate.board().possible_moves_from(position.0).count() > 0 {
            d += 1;
        }
    }
    return d;
}

pub fn get_pengu(gamestate:&State, my_turn:i32) -> i32 {
    return my_turn * (get_moveable_peguins(gamestate, gamestate.current_team())
             - get_moveable_peguins(gamestate, gamestate.current_team().opponent()));
}

pub fn evaluate(gamestate:&State, my_turn:i32, args:Vec<f32>) -> f32 {
    let lateness = 1.0 / get_turn(gamestate) as f32;
    return    args[0] * lateness.powf(args[1]) * get_fish_dif(gamestate, my_turn) as f32
            + args[2] * lateness.powf(args[3]) * get_move_num(gamestate, my_turn) as f32
            + args[4] * lateness.powf(args[5]) * get_pengu(gamestate, my_turn) as f32;
}
