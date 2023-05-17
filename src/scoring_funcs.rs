use std::{collections::HashMap, cmp::{max, min}};

use log::info;
use rand::seq::SliceRandom;

use std::time::Instant;
use array_tool::vec::Intersect;

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


fn get_field_levels_from(gamestate:&State, t_fields:&Vec<Vec2<Doubled>>) -> (f32, f32) {
    let mut this_fields = t_fields.clone();
    let mut done_fields:Vec<Vec2<Doubled>> = vec![];
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

pub fn get_pengu_mobility_of(gamestate:&State, team1:Team) -> f32 {
    let pingus1 = gamestate.pieces_of(team1);
    let mut c1 = 0.;
    for position in pingus1 {
        c1 += get_field_levels_from(gamestate, &vec![position.0]).0;
    }
    return c1;
}


pub fn get_field_levels_of(gamestate:&State, team1:Team) -> (f32, f32) {
    let pingus1 = gamestate.pieces_of(team1);
    let mut this_fields:Vec<Vec2<Doubled>> = vec![];
    for position in pingus1 {
        for m in gamestate.board().possible_moves_from(position.0) {
            if (!this_fields.contains(&m.to())) {
                this_fields.push(m.to());
            }
        }
    }
    return get_field_levels_from(gamestate, &this_fields, );
    
}



pub fn get_field_levels(gamestate:&State, my_turn:i32) -> (f32, f32) {
    let(l1,f1) = get_field_levels_of(gamestate, gamestate.current_team());
    let (l2, f2) = get_field_levels_of(gamestate, gamestate.current_team().opponent());
    return (
        my_turn as f32 * (l1 - l2),
        my_turn as f32 * (f1 - f2)
    );
}

pub fn get_pengu_mobility(gamestate:&State, my_turn:i32) -> (f32) {
    let l1 = get_pengu_mobility_of(gamestate, gamestate.current_team());
    let l2 = get_pengu_mobility_of(gamestate, gamestate.current_team().opponent());
    return my_turn as f32 * (l1 - l2);
}


pub fn evaluate(gamestate:&mut State, my_turn:i32, args:&Vec<f32>) -> f32 {
    let lateness = 40.0 / gamestate.turn() as f32;
    let (f,c) = get_field_levels(gamestate, my_turn);
    return  args[0] * lateness.powf(args[1]) * get_fish_dif(gamestate, my_turn) as f32
        +   args[2] * lateness.powf(args[3]) * f
        +   args[4] * lateness.powf(args[5]) * get_pengu(gamestate, my_turn) as f32
        //+   args[6] * lateness.powf(args[7]) * get_move_num(gamestate, my_turn)
        //+   args[8] * lateness.powf(args[9]) * c
        ;
}

pub fn fast_evaluate(gamestate:&State, my_turn:i32, args:&Vec<f32>) -> f32 {
    let lateness = 40.0 / gamestate.turn() as f32;
    return  args[0] * lateness.powf(args[1]) * get_fish_dif(gamestate, my_turn) as f32
    +   args[2] * lateness.powf(args[3]) * get_move_num(gamestate, my_turn)
    +   args[4] * lateness.powf(args[5]) * get_pengu(gamestate, my_turn) as f32
}




pub fn print_eval(gamestate:&mut State, my_turn:i32, args:&Vec<f32>) {
    let lateness = 40.0 / gamestate.turn() as f32;
    let (f,c) = get_field_levels(gamestate, my_turn);
    info!("Arg 1: {}", args[0] * lateness.powf(args[1]) * get_fish_dif(gamestate, my_turn) as f32);
    info!("Arg 2: {}", args[2] * lateness.powf(args[3]) * f);
    info!("Arg 3: {}", args[4] * lateness.powf(args[5]) * get_pengu(gamestate, my_turn) as f32);
    info!("Arg 4: {}", args[6] * lateness.powf(args[7]) * get_move_num(gamestate, my_turn));
    info!("Arg 5: {}", args[8] * lateness.powf(args[9]) * c);      
}

pub fn test_speed(gamestate:&State) {
    test_speed_single(get_fish_dif, gamestate);
    test_speed_double(get_field_levels, gamestate);
    test_speed_single(get_pengu, gamestate);
    test_speed_single(get_pengu_mobility, gamestate);
   // test_speed_single(get_game_sim, gamestate);
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


pub fn get_vals_as_str(gamestate:&State, my_turn:i32) -> String {
    let lateness =  gamestate.turn() as f32;
    let (f,c) = get_field_levels(gamestate, my_turn);
    return vec![
        lateness.to_string(),
        get_fish_dif(gamestate, my_turn).to_string(),
        f.to_string(),
        get_pengu(gamestate, my_turn).to_string(),
        get_move_num(gamestate, my_turn).to_string(),
        c.to_string()
    ].join(";");
}