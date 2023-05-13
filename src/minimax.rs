use std::{f32::INFINITY, vec, thread, time::Duration, sync::mpsc::{self}};
use log::info;

use crate::{game::{State, Team, Move, self}, scoring_funcs::{ evaluate}};
use std::time::Instant;

const ZER_VEC:Vec<usize> = vec![];
const BREAK_TIME:u128=1000;




pub fn dyn_max(gamestate:State, my_team:Team, args:Vec<f32>) -> Option<Move>
{
    let start = Instant::now();

    let mut curmove:Option<Move>= None;
    let mut curdepth = 0;

    while start.elapsed().as_millis() < BREAK_TIME && curdepth < 30 { 
        let (mtx, mrx) = mpsc::channel();
        let args_c = args.clone();
        thread::spawn(move || {
            let (m, _, cf) = new_minmax(&mut (gamestate.clone()), my_team, -INFINITY, INFINITY, &args_c, curdepth);
            mtx.send(m);
        });
        while start.elapsed().as_millis() < BREAK_TIME {
            let m = mrx.recv_timeout(Duration::new(0, 50));
            if !m.is_err() {
                curdepth+=1;
                curmove = m.unwrap();
                if start.elapsed().as_millis() > BREAK_TIME {
                    break;
                }
                break;
            } 
        }
    }
    info!("depth: {}", curdepth);
    return curmove;
}






pub fn minimax(gamestate:&mut State, my_team:Team, mut alpha:f32, mut beta:f32, args:&Vec<f32>, depth:i32) -> (Option<Move>, f32, Vec<usize>) {
    let mut my_turn = -1;
    if gamestate.current_team().index()== my_team.index() {
        my_turn = 1;
    }
    if gamestate.is_over() 
    {
        if gamestate.winner().is_none() {
            return (None, 0.0, ZER_VEC)
        }
        if gamestate.winner().unwrap().index() == my_team.index() {
            return (None, INFINITY, ZER_VEC) ;
        }
        else {
            return (None, -INFINITY, ZER_VEC);
        };
    } 
    if depth <= 0 {
        return (None, evaluate(gamestate, my_turn, args), ZER_VEC);
    }

   
    let possible_moves = gamestate.possible_moves();
    let mut best_move =  possible_moves[0];
    let mut value;
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
    return (Some(best_move), value, ZER_VEC);
} 



pub fn new_minmax(gamestate:&mut State, my_team:Team, mut alpha:f32, mut beta:f32, args:&Vec<f32>, depth:i32) -> (Option<Move>, f32, Vec<usize>) {
    let mut my_turn = -1;
    if gamestate.current_team().index()== my_team.index() {
        my_turn = 1;
    }
    if gamestate.is_over() 
    {
        if gamestate.winner().is_none() {
            return (None, 0.0, ZER_VEC)
        }
        if gamestate.winner().unwrap().index() == my_team.index() {
            return (None, INFINITY, ZER_VEC) ;
        }
        else {
            return (None, -INFINITY, ZER_VEC);
        };
    } 
    if depth <= 0 {
        return (None, evaluate(gamestate, my_turn, args), ZER_VEC);
    }
    let mut possible_moves = gamestate.possible_moves();
    let mut best_move =  possible_moves[0];
    let mut value;
    if my_turn == 1 {
        possible_moves.sort_by(|&a, &_b| evaluate_move(gamestate, a, my_turn, args).partial_cmp(&evaluate_move(gamestate, a, my_turn, args)).unwrap());
        possible_moves.reverse();
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
        possible_moves.sort_by(|&a, &_b| evaluate_move(gamestate, a, my_turn, args).partial_cmp(&evaluate_move(gamestate, a, my_turn, args)).unwrap());
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
    return (Some(best_move), value, ZER_VEC);
} 

pub fn evaluate_move(gamestate: &mut State, m:Move, my_turn:i32, args:&Vec<f32>) {
    let t = gamestate.current_team();
    let f = gamestate.perform(m);
    let d = evaluate(gamestate, my_turn, args);
    gamestate.undo_move(m, f, t)
}


pub fn test_speed_minmax(args_c:&Vec<f32>, gamestate:&State) {

    

    let now = Instant::now();
    {
        for i in 1..10 {
            minimax(&mut (gamestate.clone()), gamestate.current_team(), -INFINITY, INFINITY, &args_c, 4);
        }
    }
    let elapsed = now.elapsed();
    info!("Min1 Speed: {:.2?}", elapsed);

    let now = Instant::now();
    {
        for i in 1..10 {
            new_minmax(&mut (gamestate.clone()), gamestate.current_team(), -INFINITY, INFINITY, &args_c, 4);
        }
    }
    let elapsed = now.elapsed();
    info!("Min1 Speed: {:.2?}", elapsed);
}
