use std::f32::INFINITY;

use crate::{game::{State, Team, Move}, scoring_funcs::{self, evaluate}};





pub fn minimax(gamestate:&State, my_team:Team, alpha:f32, beta:f32, args:Vec<f32>, depth:i32) -> (Option<Move>, f32) {
    let mut my_turn = -1;
    if gamestate.current_team().letter()== my_team.letter() {
        my_turn = 1;
    }
    if gamestate.is_over() {
        if gamestate.winner().unwrap().letter() == my_team.letter() {
            return (None, INFINITY)
        }
        else {
            return (None, -INFINITY);
        }
    }
    if depth <= 0 {
        return (None, evaluate(gamestate, my_turn, args));
    }
    
    let possible_moves = gamestate.possible_moves();
    let use_move = possible_moves[0];
    if my_turn == 0 {
        let mut value = -INFINITY;
        for m in possible_moves {
            
        }
    }

}