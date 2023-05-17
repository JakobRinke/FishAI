use array_tool::vec;
use log::{info, debug};
use rand::seq::SliceRandom;
use socha_client_2023::{client::GameClientDelegate, game::{Move, Team, State}, minimax::{minimax, dyn_max, test_speed_minmax}};
use std::{f32::INFINITY, fs::{File, OpenOptions}, path::Path, io::Write};
use socha_client_2023::scoring_funcs::*;

use crate::filename;

/// An empty game logic structure that implements the client delegate trait
/// and thus is responsible e.g. for picking a move when requested.
pub struct OwnLogic;


static mut data_vec:Vec<String> = vec![];
static mut team_name:usize = 0;


impl GameClientDelegate for OwnLogic {

    fn request_move(&mut self, state: &State, _my_team: Team) -> Move {
        
        // Ice Dancer               Art of the Pengu        f(x)                ARG 5        rndarg
        // 0.78                         0.5                 1/x                    Yes
        //                              0.846              4*(1-sigm(x))           Yes
        // 1.27                             1.022              4*(1-sigm(x))            No
        let a: &[f32] = &[  5., -0.7, 3.6, 0.5, 5.7, 0.4, 1.5, 0.0, 0.9, 0.4  ];


        //                                               4*(1-sigm(x))                        
        //let a: &[f32] = &[  5.8, -0.45, 3.9, 0.6, 5.7, 0.4, 0.78, 0.94, 4.0, -1.3 ];
        
        // AI one -> Slightly worese
        //let a: &[f32] = &[0.35355, -0.44521, 0.1337, -0.11591, 0.02509, 0.21724, 0.06388, 0.1461, 0.22005, -2.76369];


        //let a: &[f32] = &[0.21314, -0.30633, 0.18359, -0.66156, 0.10861, 0.42394, -0.00553, 0., 0., 0., 0.];
        let a: &[f32] = &[0.24125, -0.38455, 0.3477, -0.96098, 0.05645, 0.9795, 0.0131];

        let b = a.to_vec();

        info!("round: {}", state.turn());
        info!("score : {}", evaluate(&mut(state.clone()), 1, &b));

        // print_eval(&mut(state.clone()), 1, &b);
        if state.turn() > 2 {
            unsafe { 
                data_vec.push(get_vals_as_str(state, 1));
                team_name = _my_team.index() 
            };
        }


        //test_speed_minmax(&b, &mut(state.clone()));
        //test_speed(state);

        // info!("val1: {}", get_field_levels(state, 1));
        // info!("val2: {}", get_field_levels_2(state, 1));


        if state.turn() <= 7 {
            //let chosen_move = find_best_start_move( *state);
            let mut s = state.clone();
            let chosen_move = dyn_max( s, _my_team, b).unwrap();
            return chosen_move
        }
        else {
            let mut s = state.clone();
            let k = dyn_max(s, _my_team, b);

            //info!("Chose move {}", k.unwrap());
            return k.unwrap();
        }

        
    }

    fn on_update_state(&mut self, state: &State) {
        debug!("Board:\n{}", state.board());
    }


    fn on_game_end(&mut self, _result: &socha_client_2023::protocol::GameResult) {
        // unsafe {
        //     let mut win = 0;
        //     if let Some(winner) = _result.winner().clone() {
        //         if winner.team().index() == team_name {
        //             win = 1;
        //         } else {
        //             win = -1;
        //         }
        //     } 
        //     for i in 0..data_vec.len() {
        //         data_vec[i] += &(";".to_owned() + &win.to_string());
        //     }
        //     let mut f = OpenOptions::new()
        //         .write(true)
        //         .append(true)
        //         .open(filename)
        //         .unwrap();
        //     for v in &data_vec {
        //         if let Err(e) = writeln!(f, "{}", v) {
        //             eprintln!("Couldn't write to file: {}", e);
        //         };
        //     }      
        // }       
    }
}
