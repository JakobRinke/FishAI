use std::{f32::INFINITY, vec};
use log::info;

use crate::{game::{State, Team, Move, self, Vec2}, scoring_funcs::{ evaluate}};
use std::time::Instant;

const ZER_VEC:Vec<usize> = vec![];
const break_time:u128= 100;




pub fn dyn_max(gamestate:&mut State, my_team:Team, args:&Vec<f32>) -> Option<Move>
{
    let mut curmove:Option<Move>= None;
    let mut controlfirst:Vec<usize> = (0..gamestate.possible_moves().len()).collect();
    let mut curdepth = 0;
    let start = Instant::now();
    while start.elapsed().as_millis() < break_time && curdepth < 50 && (curdepth < 6 || gamestate.turn() > 8)  {
        curdepth+=1;
        (curmove, _, controlfirst) = minimax(gamestate, my_team, -INFINITY, INFINITY, args, curdepth, controlfirst);
    }
    info!("Did Depth: {}", curdepth);
    return curmove;
}






pub fn minimax(gamestate:&mut State, my_team:Team, mut alpha:f32, mut beta:f32, args:&Vec<f32>, depth:i32,mut controlfirst:Vec<usize>) -> (Option<Move>, f32, Vec<usize>) {
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
    if controlfirst.len()==0 {
        if my_turn == 1 {
            value = -INFINITY;
            for m in possible_moves {
                let f = gamestate.perform(m);
                let l = minimax(gamestate, my_team, alpha, beta, args, depth-1, ZER_VEC).1;
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
                let l = minimax(gamestate, my_team, alpha, beta, args, depth-1, ZER_VEC).1;
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
        value = -INFINITY;
        let mut controlcop = controlfirst.clone();
        for iter in controlfirst {
            let f = gamestate.perform(possible_moves[iter]);
            let l = minimax(gamestate, my_team, alpha, beta, args, depth-1, ZER_VEC).1;
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


const CENTERVEC: i32 = 7;


pub fn find_best_start_move(gamestate: State) -> Move {
    let possible = gamestate.possible_moves();
    let mut current_best = possible[0];
    let mut current_max = -INFINITY;
    for m in possible {
        let cost = get_move_cost_diff(gamestate, m);
        if cost > current_max {
            current_max = cost;
            current_best = m;
        }
    }
    info!("{}", current_best.to());
    current_best
}

fn get_move_cost_diff(gamestate: State, m:Move) -> f32 {
    let mut cost = 0.0;

    /*
     for k in gamestate.current_pieces() {
        cost = f32::min(cost, f32::sqrt(((k.0.x - m.to().x).pow(2) + (k.0.y - m.to().y)).pow(2)  as f32))
        }
    for k in gamestate.opponent_pieces() {
        cost = f32::min(cost, f32::sqrt(((k.0.x - m.to().x).pow(2) + (k.0.y - m.to().y)).pow(2)  as f32))
    }
    */
   

         for k in gamestate.current_pieces() {
            cost += f32::sqrt(((k.0.x - m.to().x).pow(2) + (k.0.y - m.to().y)).pow(2)  as f32)
            }
        for k in gamestate.opponent_pieces() {
            cost += f32::sqrt(((k.0.x - m.to().x).pow(2) + (k.0.y - m.to().y)).pow(2)  as f32)
        }
        cost -= 1000.0 * f32::sqrt(((CENTERVEC - m.to().x).pow(2) + (CENTERVEC - m.to().y).pow(2)) as f32) ;

   
    return cost;
}