use std::{f32::INFINITY, fmt::Debug};

use crate::{game::{State, Team, Move}, scoring_funcs::{ evaluate}};





pub fn minimax(gamestate:&mut State, my_team:Team, mut alpha:f32, mut beta:f32, args:&Vec<f32>, depth:i32) -> (Option<Move>, f32) {
    let mut my_turn = -1;
    if gamestate.current_team().index()== my_team.index() {
        my_turn = 1;
    }
    
    if gamestate.is_over() 
    {
        if gamestate.winner().unwrap().index() == my_team.index() {
            return (None, INFINITY);
        }
        else {
            return (None, -INFINITY);
        };
    } 
    
    if depth <= 0 {
        return (None, evaluate(gamestate, my_turn, args));
    }
    
    let possible_moves = gamestate.possible_moves();
    let mut best_move =  possible_moves[0];
    let mut value = 0.0;
    if my_turn == 1 {
        value = -INFINITY;
        for m in possible_moves {
            let f = gamestate.perform(m);
            let l = minimax(gamestate, my_team, alpha, beta, args, depth-1).1;
            gamestate.undo_move(m, f, my_team);
            if  l > value {
                best_move = m;
                value = l;
            }
            alpha = f32::max(alpha, value);
            if alpha >= beta {
                break;
            }
        }
    }
    else {
        value = INFINITY;
        for m in possible_moves {
                let f = gamestate.perform(m);
                let l = minimax(gamestate, my_team, alpha, beta, args, depth-1).1;
                gamestate.undo_move(m, f, my_team.opponent());
                if  l < value {
                    best_move = m;
                    value = l;
                }
                beta = f32::min(beta, value);
                if alpha >= beta {
                    break;
                }  
        }
    }

    return (Some(best_move), value);


} 