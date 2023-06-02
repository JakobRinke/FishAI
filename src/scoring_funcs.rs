use std::{collections::HashMap, cmp::{max, min}};

use log::info;
// use neuroflow::{FeedForward, io};
use time::Instant;

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


pub fn get_field_control(gamestate:&State, field:Vec2<Doubled>, h:&mut HashMap<Vec2<Doubled>, i32>, val:i32) {
    for new_field in gamestate.board().possible_moves_from(field) {
        if let Some(k) = h.get_mut(&new_field.to()) {
            *k+=1;
        } else {
            h.insert(new_field.to(), val);
            get_field_control(gamestate, new_field.to(), h, val);
        }
    }
}


pub fn get_controlled_fields_of(gamestate:&State, team1:Team, team2:Team) -> i32 {
    let pingus1 = gamestate.pieces_of(team1);
    let pingus2 = gamestate.pieces_of(team2);
    let mut hm: HashMap<Vec2<Doubled>, i32> = HashMap::new();
    let mut count = 0;
    for position in pingus1 {
        get_field_control(gamestate, position.0, &mut hm, 1) ;
    }
    for position in pingus2 {
        get_field_control(gamestate, position.0, &mut hm, -1) ;
    }
    for key in hm.keys() {
        count += max(-1, min(1, *hm.get(key).unwrap())) * gamestate.board()[*key].fish() as i32;
    }  
    return count;
}

pub fn get_controlled_fields(gamestate:&State, my_turn:i32) -> f32 {
    return (my_turn * get_controlled_fields_of(gamestate, gamestate.current_team(), gamestate.current_team().opponent()))as f32;
}


fn get_field_levels_from(gamestate:&State, t_fields:&Vec<Vec2<Doubled>>) -> (f32) {
    let mut this_fields = t_fields.clone();
    let mut done_fields:Vec<Vec2<Doubled>> = vec![];
    let mut next_fields:Vec<Vec2<Doubled>>;
    let mut i = 1;
    let mut count:f32 = 0.;
    while this_fields.len() > 0 {
        next_fields = vec![];
        for field in &this_fields {
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
    return count;
}

pub fn get_pengu_mobility_of(gamestate:&State, team1:Team) -> f32 {
    let pingus1 = gamestate.pieces_of(team1);
    let mut c1 = 0.;
    for position in pingus1 {
        c1 += get_field_levels_from(gamestate, &vec![position.0]);
    }
    return c1;
}


pub fn get_field_levels_of(gamestate:&State, team1:Team) -> (f32) {
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



pub fn get_field_levels(gamestate:&State, my_turn:i32) -> (f32) {
    let(l1) = get_field_levels_of(gamestate, gamestate.current_team());
    let (l2) = get_field_levels_of(gamestate, gamestate.current_team().opponent());
    return my_turn as f32 * (l1 - l2);
    
}

pub fn get_pengu_mobility(gamestate:&State, my_turn:i32) -> (f32) {
    let l1 = get_pengu_mobility_of(gamestate, gamestate.current_team());
    let l2 = get_pengu_mobility_of(gamestate, gamestate.current_team().opponent());
    return my_turn as f32 * (l1 - l2);
}


fn get_distance(v1 : Vec2<Doubled>, v2:Vec2<Doubled>) -> f32 {
    return (v1-v2).length();
}

fn get_pingu_dist(gamestate:&State, team1:Team) -> f32{
    let pingus1 = gamestate.pieces_of(team1);
    let mut v = vec![];
    for p1 in pingus1 {
        v.push(p1.0);
    }
    let mut d = 0.;
    for i in 0..v.len() {
        for j in i+1..v.len() {
            d+=get_distance(v[i], v[j]);
        }
    }
    return d;
}

fn get_distances_pengu_to_enemies(gamestate:&State,  v2:Vec2<Doubled>, team2:Team) -> f32 {
    let pingus2 = gamestate.pieces_of(team2);
    let mut d = 0.0;
    for p1 in pingus2 {
        d += get_distance(p1.0, v2)
    }
    return d;
}

fn get_distances_to_enemy_of(gamestate:&State, team1:Team) -> f32{
    let pingus = gamestate.pieces_of(team1);
    let mut d = 0.0;
    for p1 in pingus {
        d += get_distances_pengu_to_enemies(gamestate,p1.0, team1.opponent());
    }
    return d;
}

fn get_pingu_dist_diff(gamestate:&State, my_turn:i32) -> f32 {
    my_turn as f32 * ( get_pingu_dist(&gamestate, gamestate.current_team()) -
                get_pingu_dist(&gamestate, gamestate.current_team().opponent()))
}

fn get_pingu_enemy_dist_diff(gamestate:&State, my_turn:i32) -> f32 {
    my_turn as f32 * ( get_distances_to_enemy_of(&gamestate, gamestate.current_team()) -
                get_distances_to_enemy_of(&gamestate, gamestate.current_team().opponent()))
}

fn get_mobbing_of_pengu(gamestate:&State, spot:Vec2<Doubled>,enemy_team:Team) -> f32 {
    let pingus = gamestate.pieces_of(enemy_team);
    let mut d = 0;
    for p1 in pingus {
        if get_distance(p1.0, spot) < 3. {
            d+=1;
        }
    }
    if (d>=2) {
        return 1.;
    }
   return 0.;
}

fn get_overall_mobbing(gamestate:&State, team:Team) -> f32 {
    let pingus = gamestate.pieces_of(team);
    let mut d = 0.0;
    for p1 in pingus {
        d += get_mobbing_of_pengu(gamestate, p1.0, team.opponent());
    }
    return d;
}

fn get_mobbing_val(gamestate:&State, my_turn:i32) -> f32 {
    my_turn as f32 * ( get_overall_mobbing(&gamestate, gamestate.current_team()) -
    get_overall_mobbing(&gamestate, gamestate.current_team().opponent()))
}


// pub static mut NN:Option<FeedForward> = None;

// pub fn set_net(f:FeedForward) {
//      unsafe { NN = Some(f) } ;
// }


// const args1: &[f32] = &[0.21314, -0.30633, 0.18359, -0.66156, 0.10861, 0.42394, -0.00553];
// // const args1: &[f32] = &[0.23317, -1.07372, 0.27044, -0.61939, 0.06002, 0.74372, 0.03093, -0.9385, -0.01514, 0.20597]
// pub fn evaluate(gamestate:&mut State, my_turn:i32) -> f32 {
//     let lateness = 40.0 / gamestate.turn() as f32;
//     return  args1[0] * lateness.powf(args1[1]) * get_fish_dif(gamestate, my_turn)
//         +   args1[2] * lateness.powf(args1[3]) *  get_field_levels(gamestate, my_turn)
//         +   args1[4] * lateness.powf(args1[5]) * get_pengu(gamestate, my_turn)
//         //  +   args1[6] * lateness.powf(args[7]) * get_move_num(gamestate, my_turn)
//         // +   args[12] * lateness.powf(args[13]) * get_controlled_fields(gamestate, my_turn)
//         // +   args[8] * lateness.powf(args[9]) * get_pingu_dist_diff(gamestate, my_turn)
//     //    +   args[10] * lateness.powf(args[11]) * get_pingu_enemy_dist_diff(gamestate, my_turn)
//         ;
// }

const args1: &[f32] = &[0.31774516, 0.36627942, 0.3016686];
// const args1: &[f32] = &[0.23317, -1.07372, 0.27044, -0.61939, 0.06002, 0.74372, 0.03093, -0.9385, -0.01514, 0.20597]
pub fn evaluate(gamestate:&mut State, my_turn:i32) -> f32 {
    //let lateness = 40.0 / gamestate.turn() as f32;
    return  args1[0] * get_fish_dif(gamestate, my_turn)
        +   args1[1] * get_field_levels(gamestate, my_turn)
        +   args1[2] * get_pengu(gamestate, my_turn) 
        // +   args1[3] * get_move_num(gamestate, my_turn)
        // +   args1[4] *  get_pingu_dist_diff(gamestate, my_turn)
        // +   args1[5] * get_pingu_enemy_dist_diff(gamestate, my_turn)
        //  +   args1[6] * lateness.powf(args[7]) * get_move_num(gamestate, my_turn)
        // +   args[12] * lateness.powf(args[13]) * get_controlled_fields(gamestate, my_turn)
        // +   args[8] * lateness.powf(args[9]) * get_pingu_dist_diff(gamestate, my_turn)
    //    +   args[10] * lateness.powf(args[11]) * get_pingu_enemy_dist_diff(gamestate, my_turn)
        ;
}

// pub fn evaluate(gamestate:&mut State, my_turn:i32, args:&Vec<f32>) -> f32 {
//     let lateness = 40.0 / gamestate.turn() as f64;
//     let (f,c, g) = get_field_levels(gamestate, my_turn);
//     return  unsafe { NN.as_mut().unwrap().calc(&[
//         lateness,
//         get_fish_dif(gamestate, my_turn) as f64,
//         f as f64,
//         get_pengu(gamestate, my_turn) as f64,
//         get_move_num(gamestate, my_turn) as f64,
//         c as f64, 
//         g as f64,
//         get_controlled_fields(gamestate, my_turn) as f64,
//         get_pingu_dist_diff(gamestate, my_turn) as f64, 
//         get_pingu_enemy_dist_diff(gamestate, my_turn) as f64
//     ]) } [0] as f32;
// }

pub fn fast_evaluate(gamestate:&State, my_turn:i32, args:&Vec<f32>) -> f32 {
    let lateness = 40.0 / gamestate.turn() as f32;
    return  args[0] * lateness.powf(args[1]) * get_fish_dif(gamestate, my_turn) as f32
    +   args[2] * lateness.powf(args[3]) * get_move_num(gamestate, my_turn)
    +   args[4] * lateness.powf(args[5]) * get_pengu(gamestate, my_turn) as f32
}




pub fn print_eval(gamestate:&mut State, my_turn:i32, args:&Vec<f32>) {
    let lateness = 40.0 / gamestate.turn() as f32;
    let (f) = get_field_levels(gamestate, my_turn);
    info!("Arg 1: {}", args[0] * lateness.powf(args[1]) * get_fish_dif(gamestate, my_turn) as f32);
    info!("Arg 2: {}", args[2] * lateness.powf(args[3]) * f);
    info!("Arg 3: {}", args[4] * lateness.powf(args[5]) * get_pengu(gamestate, my_turn) as f32);
    info!("Arg 4: {}", args[6] * lateness.powf(args[7]) * get_move_num(gamestate, my_turn));
    //info!("Arg 5: {}", args[8] * lateness.powf(args[9]) * c);      
}

pub fn test_speed(gamestate:&State) {
    test_speed_single(get_fish_dif, gamestate);
    test_speed_single(get_field_levels, gamestate);
    test_speed_single(get_pengu, gamestate);
    test_speed_single(get_move_num, gamestate);
    test_speed_single(get_pengu_mobility, gamestate);
    test_speed_single(get_pingu_dist_diff, gamestate);
    test_speed_single(get_pingu_enemy_dist_diff, gamestate);
    test_speed_single(get_controlled_fields, gamestate);
   // test_speed_single(get_game_sim, gamestate);
}


const spc:i32 = 5000;
fn test_speed_single(f:fn(gamestate:&State, my_turn:i32)->f32, gamestate:&State) {
    let now = Instant::now();
    {
        for i in 1..spc {
            f(gamestate, 1);
        }
    }
    let elapsed = now.elapsed().as_seconds_f32();
    info!("Arg Speed: {:.5?}", elapsed);
}

fn test_speed_double(f:fn(gamestate:&State, my_turn:i32)->(f32, f32, f32), gamestate:&State) {
    let now = Instant::now();
    {
        for i in 1..spc {
            f(gamestate, 1);
        }
    }
    let elapsed = now.elapsed().as_seconds_f32();
    info!("Arg Speed: {:.5?}", elapsed);
}


fn get_model_args(gamestate:&State, my_turn:i32) -> Vec<f32> {
    let lateness =  gamestate.turn() as f32;
    let (f) = get_field_levels(gamestate, my_turn);
    vec![
        lateness,
        get_fish_dif(gamestate, my_turn),
        f,
        get_pengu(gamestate, my_turn),
        get_mobbing_val(gamestate, my_turn),
    ]
}

pub fn toStringVec(inp:Vec<f32>) -> Vec<String> {
    let mut out = vec![];
    for v in inp {
        out.push(v.to_string())
    }
    return out;
}

pub fn get_vals_as_str(gamestate:&State, my_turn:i32) -> String {

    return toStringVec(get_model_args(gamestate, my_turn)).join(";");
}

