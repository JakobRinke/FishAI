use std::{vec, thread, time::Duration, sync::mpsc::{self}, cmp::min};
use log::info;

use crate::{game::{State, Team, Move}, scoring_funcs::{ evaluate, fast_evaluate, get_fish_dif}};
use std::time::Instant;

const ZER_VEC:Vec<usize> = vec![];
const BREAK_TIME:u128=1730;




pub fn dyn_max(gamestate:State, my_team:Team) -> Option<Move>
{
    let start = Instant::now();

    let mut curmove:Option<Move>= None;
    let mut controlfirst:Vec<usize> = (0..gamestate.possible_moves().len()).collect();
    let mut curdepth = 0;

    while start.elapsed().as_millis() < BREAK_TIME && curdepth < 30 { 
        let (mtx, mrx) = mpsc::channel();
        let (stx, srx) = mpsc::channel();
        let mut cf = controlfirst.clone();
        thread::spawn(move || {
            let (m, _, cf) = minimax2(&mut (gamestate.clone()), my_team, f32::MIN, f32::MAX, curdepth, 2, cf);
            mtx.send(m);
            stx.send(cf)
        });
        while start.elapsed().as_millis() < BREAK_TIME {
            let m = mrx.recv_timeout(Duration::new(0, 50));
            if !m.is_err() {
                curdepth+=1;
                curmove = m.unwrap();
                if start.elapsed().as_millis() > BREAK_TIME {
                    break;
                }
                controlfirst = srx.recv().unwrap();
                break;
            } 
        }
    }
    info!("depth: {}", curdepth);
    return curmove;
}






pub fn minimax(gamestate:&mut State, my_team:Team, mut alpha:f32, mut beta:f32, depth:i32,controlfirst:Vec<usize>) -> (Option<Move>, f32, Vec<usize>) {
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
            return (None, f32::MAX-100.+get_fish_dif(gamestate, my_turn), ZER_VEC);
        }
        else {
            return (None, f32::MIN+100.+get_fish_dif(gamestate, my_turn), ZER_VEC);
        };
    } 
    if depth <= 0 {
        return (None, evaluate(gamestate, my_turn), ZER_VEC);
    }

   
    let possible_moves = gamestate.possible_moves();
    let mut best_move =  possible_moves[0];
    let mut value;
    if controlfirst.len()==0 {
        if my_turn == 1 {
            value = f32::MIN;
            for m in possible_moves {
                let f = gamestate.perform(m);
                let l = minimax(gamestate, my_team, alpha, beta, depth-1, ZER_VEC).1;
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
            value = f32::MAX;
            for m in possible_moves {
                let f = gamestate.perform(m);
                let l = minimax(gamestate, my_team, alpha, beta, depth-1, ZER_VEC).1;
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
    } else {
        let mut vals:Vec<f32> = vec![0.0; controlfirst.len()];
        value = f32::MIN;
        let mut controlcop = controlfirst.clone();
        for iter in controlfirst {
            let f = gamestate.perform(possible_moves[iter]);
            let l = minimax(gamestate, my_team, alpha, beta, depth-1, ZER_VEC).1;
            vals[iter] = l;
            gamestate.undo_move(possible_moves[iter], f, my_team);
            if  l > value {
                best_move = possible_moves[iter];
                value = l;
            }
            alpha = f32::max(alpha, value);
            if alpha >= beta {
                break;
            }
        }
        controlcop.sort_by(|&a, &b| vals[a].partial_cmp(&vals[b]).unwrap());
        controlcop.reverse();
        return (Some(best_move), value, controlcop);
    }
}

pub fn minimax2(gamestate:&mut State, my_team:Team, mut alpha:f32, mut beta:f32, depth:i32, depth2:i32,controlfirst:Vec<usize>) -> (Option<Move>, f32, Vec<usize>) {
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
            return (None, f32::MAX-100.+get_fish_dif(gamestate, my_turn), ZER_VEC);
        }
        else {
            return (None, f32::MIN+100.+get_fish_dif(gamestate, my_turn), ZER_VEC);
        };
    } 
    if depth <= 0 {
        return (None, evaluate(gamestate, my_turn), ZER_VEC);
    }

   
    let possible_moves = gamestate.possible_moves();
    let mut best_move =  possible_moves[0];
    let mut value;
    if controlfirst.len()==0 {
        if my_turn == 1 {
            value = f32::MIN;
            for m in possible_moves {
                let f = gamestate.perform(m);
                let l = minimax(gamestate, my_team, alpha, beta, depth-1, ZER_VEC).1;
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
            value = f32::MAX;
            for m in possible_moves {
                let f = gamestate.perform(m);
                let l = minimax(gamestate, my_team, alpha, beta, depth-1, ZER_VEC).1;
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
    } else {
        let mut vals:Vec<f32> = vec![0.0; controlfirst.len()];
        value = f32::MIN;
        let mut controlcop = controlfirst.clone();
        for iter in controlfirst {
            let f = gamestate.perform(possible_moves[iter]);
            let l = new_minimax(gamestate, my_team, alpha, beta, depth-1, depth2-1).1;
            vals[iter] = l;
            gamestate.undo_move(possible_moves[iter], f, my_team);
            if  l > value {
                best_move = possible_moves[iter];
                value = l;
            }
            alpha = f32::max(alpha, value);
            if alpha >= beta {
                break;
            }
        }
        controlcop.sort_by(|&a, &b| vals[a].partial_cmp(&vals[b]).unwrap());
        controlcop.reverse();
        return (Some(best_move), value, controlcop);
    }
}




pub fn new_minimax(gamestate:&mut State, my_team:Team, mut alpha:f32, mut beta:f32, depth:i32, depth2:i32) -> (Option<Move>, f32) {
    let mut my_turn = -1;
    if gamestate.current_team().index()== my_team.index() {
        my_turn = 1;
    }
    if gamestate.is_over() 
    {
        if gamestate.winner().is_none() {
            return (None, 0.0)
        }
        if gamestate.winner().unwrap().index() == my_team.index() {
            return (None, f32::MAX-100.+get_fish_dif(gamestate, my_turn)) ;
        }
        else {
            return (None, f32::MIN+100.+get_fish_dif(gamestate, my_turn));
        };
    } 
    if depth <= 0 {
        return (None, evaluate(gamestate, my_turn));
    }

    
    let mut possible_moves = gamestate.possible_moves();
    if depth2 > 0 {
        possible_moves.sort_by(|&a, &_b| fast_evaluate_move(gamestate, a, my_turn).partial_cmp(&fast_evaluate_move(gamestate, a, my_turn)).unwrap());
        if my_turn==1 {
            possible_moves.reverse();
        }
    }
    let mut best_move =  possible_moves[0];
    let mut value;
        if my_turn == 1 {
            value = f32::MIN;
            for m in possible_moves {
                let f = gamestate.perform(m);
                let l = new_minimax(gamestate, my_team, alpha, beta, depth-1, depth2-1).1;
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
            value = f32::MAX;
            for m in possible_moves {
                let f = gamestate.perform(m);
                let l = new_minimax(gamestate, my_team, alpha, beta, depth-1, depth2-1).1;
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




pub fn test_speed_minmax( gamestate:&State) {
    for i in 0..5 {
        info!("DynMax Depth: {}", dyn_max_test(*gamestate, gamestate.current_team(), i))
    }
}



pub fn evaluate_move(gamestate: &mut State, m:Move, my_turn:i32) -> f32 {
    let t = gamestate.current_team();
    let f = gamestate.perform(m);
    let d = evaluate(gamestate, my_turn);
    gamestate.undo_move(m, f, t);
    return d;
}


const be:&[f32] = &[ 3.3,  -1.6, 0.78, 0.94, 7.3, 0.3, 2.6, -0.44, 1.6, 0.16 ];
pub fn fast_evaluate_move(gamestate: &mut State, m:Move, my_turn:i32) -> f32 {
    let t = gamestate.current_team();
    let f = gamestate.perform(m);
    let d = fast_evaluate(gamestate, my_turn, &be.to_vec());
    gamestate.undo_move(m, f, t);
    return d;
}


pub fn dyn_max_test(gamestate:State, my_team:Team, depth2:i32) -> i32
{
    let start = Instant::now();

    let mut curmove:Option<Move>= None;
    let mut controlfirst:Vec<usize> = (0..gamestate.possible_moves().len()).collect();
    let mut curdepth = 0;

    while start.elapsed().as_millis() < BREAK_TIME && curdepth < 30 { 
        let (mtx, mrx) = mpsc::channel();
        let (stx, srx) = mpsc::channel();
        let mut cf = controlfirst.clone();
        thread::spawn(move || {
            let (m, _, cf) = minimax2(&mut (gamestate.clone()), my_team, f32::MIN, f32::MAX, curdepth, depth2, cf);
            mtx.send(m);
            stx.send(cf)
        });
        while start.elapsed().as_millis() < BREAK_TIME {
            let m = mrx.recv_timeout(Duration::new(0, 50));
            if !m.is_err() {
                curdepth+=1;
                curmove = m.unwrap();
                if start.elapsed().as_millis() > BREAK_TIME {
                    break;
                }
                controlfirst = srx.recv().unwrap();
                break;
            } 
        }
    }
    return curdepth;
}
