use socha_client_2023::{client::GameClientDelegate, game::{Move, Team, State}};



fn get_move_num(gamestate:State) -> i32 {
    gamestate.possible_moves().length;
}

fn get_fish_dif(gamestate:State, my_turn:i8) -> i32 {
    my_turn
}