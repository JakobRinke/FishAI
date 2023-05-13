use log::{info, debug};
use rand::seq::SliceRandom;
use socha_client_2023::{client::GameClientDelegate, game::{Move, Team, State}, minimax::{minimax, dyn_max, test_speed_minmax}};
use std::{f32::INFINITY};
use socha_client_2023::scoring_funcs::*;


/// An empty game logic structure that implements the client delegate trait
/// and thus is responsible e.g. for picking a move when requested.
pub struct OwnLogic;

impl GameClientDelegate for OwnLogic {

    fn request_move(&mut self, state: &State, _my_team: Team) -> Move {
        
        // Ice Dancer               Art of the Pengu        f(x)                ARG 5        rndarg
        // 0.78                         0.5                 1/x                    Yes
        //                              0.846              4*(1-sigm(x))           Yes
        //                              1.022              4*(1-sigm(x))            No
        let a: &[f32] = &[  5., -0.7, 3.6, 0.5, 5.7, 0.4, 4.5, -0.19, 0.9, 0.4  ];


        //                                               4*(1-sigm(x))             No           Yes
        //let a: &[f32] = &[  5., -0.7, 3.6, 0.5, 10.7, 0.4, 0.7, 0.0, 0.9, 0.4  ];


        let b = a.to_vec();
        info!("round: {}", state.turn());
        info!("score : {}", evaluate(&mut(state.clone()), 1, &b));
        test_speed_minmax(&b, &mut(state.clone()));


        print_eval(&mut(state.clone()), 1, &b);
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
