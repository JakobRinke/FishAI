use std::{collections::HashMap, cmp::{max, min}};

use log::info;
use rand::seq::SliceRandom;

use std::time::Instant;


use crate::game::{State, Team, Vec2, Doubled, self};

pub fn get_move_num(gamestate:&State, my_turn:i32) -> f32 {
    my_turn as f32  * (gamestate.possible_moves().len() as f32 - gamestate.opponent_moves().len() as f32)
}

pub fn get_fish_dif(gamestate:&State, my_turn:i32) -> f32 {
    my_turn as f32 * ( gamestate.fish(gamestate.current_team()) as f32 - 
                gamestate.fish(gamestate.current_team().opponent()) as f32)
}

pub fn get_turn(gamestate:&State) -> f32 {
    gamestate.turn() as f32
}

pub fn get_fish_left(gamestate:&State) -> f32 {
    return gamestate.get_fish_left() as f32;
}

// Optimize
pub fn get_moveable_peguins(gamestate:&State, team:Team) -> f32 {
    let pingus = gamestate.pieces_of(team);
    let board = gamestate.board();
    let mut d = 0;
    for position in pingus {
        if board.possible_moves_from(position.0).count() > 0 {
            d += 1;
        }
    }
    return d as f32;
}

pub fn get_pengu(gamestate:&State, my_turn:i32) -> f32 {
    return my_turn as f32 * (get_moveable_peguins(gamestate, gamestate.current_team())
             - get_moveable_peguins(gamestate, gamestate.current_team().opponent()));
}

// pub fn dj_activator(d:i32, f:usize) -> f32 {
//     f as f32 / d as f32
// }

pub fn dj_activator(d:i32, f:usize) -> f32 {
    4. * f as f32 * (1. - 1. / (1. + 2.71828_f32.powf(-d as f32)) )
}

pub fn get_field_levels_of(gamestate:&State, team1:Team) -> (f32, f32) {
    let pingus1 = gamestate.pieces_of(team1);
    let mut done_fields:Vec<Vec2<Doubled>> = vec![];
    let mut this_fields:Vec<Vec2<Doubled>> = vec![];
    for position in pingus1 {
        for m in gamestate.board().possible_moves_from(position.0) {
            if (!this_fields.contains(&m.to())) {
                this_fields.push(m.to());
            }
        }
    }
    let mut next_fields:Vec<Vec2<Doubled>>;
    let mut i = 1;
    let mut count:f32 = 0.;
    let mut fcount:usize = 0;
    while this_fields.len() > 0 {
        next_fields = vec![];
        for field in &this_fields {
            fcount+=gamestate.board()[*field].fish();
            count+=dj_activator(i, gamestate.board()[*field].fish()) as f32;
            done_fields.push(*field);
        }
        for field in this_fields {
            for new_field in gamestate.board().possible_moves_from(field) {
                if !(next_fields.contains(&new_field.to()) || done_fields.contains(&new_field.to())) {
                    next_fields.push(new_field.to());
                }
            }
        }
        this_fields = next_fields;
        i+=1;
    }

    return (count, fcount as f32);
}


pub fn get_field_levels(gamestate:&State, my_turn:i32) -> (f32, f32) {
    let (l1, f1) = get_field_levels_of(gamestate, gamestate.current_team());
    let (l2, f2) = get_field_levels_of(gamestate, gamestate.current_team().opponent());
    return (
        my_turn as f32 * (l1 - l2),
        my_turn as f32 * (f1 - f2)
    );
}



const TESTS: i32 = 100;
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

pub fn evaluate(gamestate:&mut State, my_turn:i32, args:&Vec<f32>) -> f32 {
    let lateness = 40.0 / gamestate.turn() as f32;
    let (levels, count) =  get_field_levels(gamestate, my_turn);
    return  args[0] * lateness.powf(args[1]) * get_fish_dif(gamestate, my_turn) as f32
        +   args[2] * lateness.powf(args[3]) * levels
        +   args[4] * lateness.powf(args[5]) * get_pengu(gamestate, my_turn) as f32
        //+   args[6] * lateness.powf(args[7]) * get_game_sim(gamestate, my_turn) as f32;
        //+   args[8] * lateness.powf(args[9]) * count
        ;
}

pub fn print_eval(gamestate:&mut State, my_turn:i32, args:&Vec<f32>) {

    let (levels, count) =  get_field_levels(gamestate, my_turn);

    let lateness = 40.0 / gamestate.turn() as f32;
    info!("Arg 1: {}", args[0] * lateness.powf(args[1]) * get_fish_dif(gamestate, my_turn) as f32);
    info!("Arg 2: {}", args[2] * lateness.powf(args[3]) * levels);
    info!("Arg 3: {}", args[4] * lateness.powf(args[5]) * get_pengu(gamestate, my_turn) as f32);
    //info!("Arg 4: {}", args[6] * lateness.powf(args[7]) * get_game_sim(gamestate, my_turn) as f32);
    //info!("Arg 5: {}", args[8] * lateness.powf(args[9]) * count);      
}

pub fn test_speed(gamestate:&State) {
    test_speed_single(get_fish_dif, gamestate);
    test_speed_double(get_field_levels, gamestate);
    test_speed_single(get_pengu, gamestate);
}

fn test_speed_single(f:fn(gamestate:&State, my_turn:i32)->f32, gamestate:&State) {
    let now = Instant::now();
    {
        for i in 1..2000 {
            f(gamestate, 1);
        }
    }
    let elapsed = now.elapsed();
    info!("Arg Speed: {:.2?}", elapsed);
}

fn test_speed_double(f:fn(gamestate:&State, my_turn:i32)->(f32, f32), gamestate:&State) {
    let now = Instant::now();
    {
        for i in 1..2000 {
            f(gamestate, 1);
        }
    }
    let elapsed = now.elapsed();
    info!("Arg Speed: {:.2?}", elapsed);
}