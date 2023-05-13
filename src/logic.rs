use log::{info, debug};
use rand::seq::SliceRandom;
use socha_client_2023::{client::GameClientDelegate, game::{Move, Team, State}, minimax::{minimax, dyn_max, find_best_start_move}};
use std::{f32::INFINITY};
use socha_client_2023::scoring_funcs::*;


/// An empty game logic structure that implements the client delegate trait
/// and thus is responsible e.g. for picking a move when requested.
pub struct OwnLogic;

impl GameClientDelegate for OwnLogic {

    fn request_move(&mut self, state: &State, _my_team: Team) -> Move {
        
        // 0.46; 500: 0.48
        //let a: &[f32] = &[3.0, -0.5, 0.3, 1.9, 7.3, 0.5, 0., 0., 0.0, 0.0];


        // 30
        //let a: &[f32] = &[ 3.7,  -1.2, 0.55, 0.7, 4.3, 0.5, 2.1, -0.5, 1.3, 0.2 ];

        // 38
        //let a: &[f32] = &[ 3.3,  -1.6, 0.78, 0.94, 7.3, 0.3, 2.6, -0.44, 1.6, 0.16 ];

        // 
        //let a: &[f32] = &[ 2.4,  -1.9, 1.3, 0.7, 7.3, 0.2, 2.8, -0.3, 1.8, 0.13 ];

        
        // 
        //let a: &[f32] = &[ 9., -0.8, 2.15, 0.7, 6.3, 0.6, 2.8, -0.3, 0.5, 0.7 ];

        
        // let a: &[f32] = &[ 7., -0.6, 2.6, 0.9, 4.3, 0.8, 3.5, -0.25, 0.7, 0.5 ];

        // let a: &[f32] = &[ 6., -0.7, 2.2, 0.5, 3.1, 0.4, 4.5, -0.19, 0.8, 0.4 ];


        // Ice Dancer               Art of the Pengu        f(x)
        // 0.78                         0.5                 1/x
        //                                                  4*(1-sigm(x))
        let a: &[f32] = &[  5., -0.7, 3.6, 0.5, 5.7, 0.4, 4.5, -0.19, 0.9, 0.4  ];

        let b = a.to_vec();
        info!("round: {}", state.turn());
        info!("score : {}", evaluate(state, 1, &b));



        print_eval(state, 1, &b);
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
}
