use crate::game::State;




pub fn get_move_num(gamestate:&State) -> i64 {
    gamestate.possible_moves().len() as i64
}

pub fn get_fish_dif(gamestate:State, my_turn:i8) -> i32 {
    my_turn.into()
}