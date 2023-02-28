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

pub fn get_controlled_fish_of(gamestate:&State, team1:Team, team2:Team) -> i32 {
    let pingus1 = gamestate.pieces_of(team1);
    let pingus2 = gamestate.pieces_of(team2);
    let mut hm = HashMap::new();
    let mut count = 0;
    for position in pingus1 {
        let d=get_field_control(gamestate, position.0, vec![]) ;
        for field in d {
           if (hm.contains_key(&field)) {
            hm.insert(field, hm.get(&field).unwrap()+1);
           } else {
            hm.insert(field, 1);
           }
        }
    }
    for position in pingus2 {
        let d=get_field_control(gamestate, position.0, vec![]) ;
        for field in d {
            if (hm.contains_key(&field)) {
             hm.insert(field, hm.get(&field).unwrap()-1);
            } else {
             hm.insert(field, -1);
            }
         }
    }
    for key in hm.keys() {
        count += max(-1, min(1, *hm.get(key).unwrap()));
    }  
    return count as i32;
}

pub fn get_controlled_fish(gamestate:&State, my_turn:i32) -> f32 {
    return (my_turn * get_controlled_fish_of(gamestate, gamestate.current_team(), gamestate.current_team().opponent()))as f32;
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


pub fn evaluate(gamestate:&State, my_turn:i32, args:&Vec<f32>) -> f32 {
    let lateness = 40.0 / gamestate.turn() as f32;
    return  args[0] * lateness.powf(args[1]) * get_fish_dif(gamestate, my_turn) as f32
        +   args[2] * lateness.powf(args[3]) * get_move_num(gamestate, my_turn) as f32
        +   args[4] * lateness.powf(args[5]) * get_pengu(gamestate, my_turn) as f32
        //+   args[6] * lateness.powf(args[7]) * get_controlled_fish(gamestate, my_turn) as f32
        //+   args[8] * lateness.powf(args[9]) * get_controlled_fields(gamestate, my_turn) as f32
        
        ;
}
