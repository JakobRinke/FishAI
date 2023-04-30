use std::{collections::HashMap, cmp::{max, min}};

use log::info;

use crate::game::{State, Team, self, Vec2, Doubled};

pub fn get_move_num(gamestate:&State, my_turn:i32) -> i32 {
    my_turn * (gamestate.possible_moves().len() as i32 - gamestate.opponent_moves().len() as i32)
}

pub fn get_fish_dif(gamestate:&State, my_turn:i32) -> i32 {
    my_turn * ( gamestate.fish(gamestate.current_team()) as i32 - 
                gamestate.fish(gamestate.current_team().opponent()) as i32)
}

pub fn get_turn(gamestate:&State) -> i32 {
    gamestate.turn() as i32
}

pub fn get_fish_left(gamestate:&State) -> i32 {
    return gamestate.get_fish_left() as i32;
}

// Optimize
pub fn get_moveable_peguins(gamestate:&State, team:Team) -> i32 {
    let pingus = gamestate.pieces_of(team);
    let board = gamestate.board();
    let mut d = 0;
    for position in pingus {
        if board.possible_moves_from(position.0).count() > 0 {
            d += 1;
        }
    }
    return d;
}

pub fn get_pengu(gamestate:&State, my_turn:i32) -> i32 {
    return my_turn * (get_moveable_peguins(gamestate, gamestate.current_team())
             - get_moveable_peguins(gamestate, gamestate.current_team().opponent()));
}


pub fn get_field_control(gamestate:&State, field:Vec2<Doubled>, done:Vec<Vec2<Doubled>> ) -> Vec<Vec2<Doubled>> {
    let mut dn = done.clone();
    for new_field in gamestate.board().possible_moves_from(field) {
        if !dn.contains(&new_field.to()) {
            dn.push(new_field.to());
            dn = get_field_control(gamestate, new_field.to(), dn);
        }
    }
    return dn.to_vec();
}

pub fn get_controlled_fields_of(gamestate:&State, team:Team) -> i32 {
    let pingus1 = gamestate.pieces_of(team);
    let mut count = 0;
    let mut done = vec![];
    for position in pingus1 {
        done = get_field_control(gamestate, position.0, done);
    }

    for f in done {
        count+=gamestate.board()[f].fish();
    }
    return count as i32;
}

pub fn get_controlled_fields(gamestate:&State, my_turn:i32) -> f32 {
    return (my_turn * ( get_controlled_fields_of(gamestate, gamestate.current_team()) -
                        get_controlled_fields_of(gamestate, gamestate.current_team().opponent())
                    )
            ) as f32;
}


pub fn dj_activator(d:i32, f:usize) -> f32 {
    f as f32/ (1. + f32::exp(d as f32)) 
}

pub fn add_field_levels(gamestate:&State, field:&Vec2<Doubled>,hm:&mut HashMap<Vec2<Doubled>, f32>, depth:i32) {
    for new_field in gamestate.board().possible_moves_from(*field) {
        let mut val = dj_activator(depth, gamestate.board()[new_field.to()].fish()); 
        if !hm.contains_key(&new_field.to()) {
            hm.insert(new_field.to(), val);
            add_field_levels(gamestate, &new_field.to(), hm, depth+1)
        } else {
            if let Some(curval) = hm.get_mut(&new_field.to()) {
                if *curval < val {
                    *curval = val;
                    add_field_levels(gamestate, &new_field.to(), hm, depth+1)
                }
            }
        }
    }
}

pub fn get_field_levels_of(gamestate:&State, team1:Team) -> f32{
    let pingus1 = gamestate.pieces_of(team1);
    let mut hm = HashMap::new();
    for position in pingus1 {
        add_field_levels(gamestate, &position.0, &mut hm, 1);
    }
    let mut count:f32 = 0.;
    for key in hm.values() {
        count+=*key;
    }  
    return count;
}

pub fn get_field_levels(gamestate:&State, my_turn:i32) -> f32 {
    return (
        my_turn as f32 * ( 
                get_field_levels_of(gamestate, gamestate.current_team()) -
                get_field_levels_of(gamestate, gamestate.current_team().opponent())
    ));
}



pub fn evaluate(gamestate:&State, my_turn:i32, args:&Vec<f32>) -> f32 {
    let lateness = 40.0 / gamestate.turn() as f32;
    return  args[0] * lateness.powf(args[1]) * get_fish_dif(gamestate, my_turn) as f32
        +   args[2] * lateness.powf(args[3]) * get_field_levels(gamestate, my_turn) as f32
        +   args[4] * lateness.powf(args[5]) * get_pengu(gamestate, my_turn) as f32
      //  +   args[6] * lateness.powf(args[7]) * get_controlled_fish(gamestate, my_turn) as f32
        +   args[8] * lateness.powf(args[9]) * get_controlled_fields(gamestate, my_turn) as f32
        
        ;
}
