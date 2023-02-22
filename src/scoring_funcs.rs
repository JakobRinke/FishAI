use crate::game::{State, Team, self};

pub fn get_move_num(gamestate:&State, my_turn:i32) -> i32 {
    my_turn * (gamestate.possible_moves().len() as i32 - gamestate.opponent_moves().len() as i32)
}

pub fn get_fish_dif(gamestate:&State, my_turn:i32) -> i32 {
    my_turn * ( gamestate.fish(gamestate.current_team()) as i32 - 
                gamestate.fish(gamestate.current_team().opponent()) as i32)
}

pub fn get_turn(gamestate:&State) -> i32 {
    gamestate.turn() as i32
}

pub fn get_fish_left(gamestate:&State) -> i32 {
    return gamestate.get_fish_left() as i32;
}

// Optimize
pub fn get_moveable_peguins(gamestate:&State, team:Team) -> i32 {
    let pingus = gamestate.pieces_of(team);
    let board = gamestate.board();
    let mut d = 0;
    for position in pingus {
        if board.possible_moves_from(position.0).count() > 0 {
            d += 1;
        }
    }
    return d;
}

pub fn get_pengu(gamestate:&State, my_turn:i32) -> i32 {
    return my_turn * (get_moveable_peguins(gamestate, gamestate.current_team())
             - get_moveable_peguins(gamestate, gamestate.current_team().opponent()));
}

pub fn evaluate(gamestate:&State, my_turn:i32, args:&Vec<f32>) -> f32 {
    let lateness = 40.0 / gamestate.turn() as f32;
    return  args[0] * lateness.powf(args[1]) * get_fish_dif(gamestate, my_turn) as f32
        +   args[2] * lateness.powf(args[3]) * get_move_num(gamestate, my_turn) as f32
        +   args[4] * lateness.powf(args[5]) * get_pengu(gamestate, my_turn) as f32;
}
