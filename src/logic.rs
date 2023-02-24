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
        let a: &[f32] = &[3.0, -0.5, 0.3, 1.9, 7.3, 0.5, 1.8, 0.4];


        // ???
        // let a: &[f32] = &[2.4, -1.5, 0.35, 1.1, 1.1, 0.5, 1.8, 0.4];


        let b = a.to_vec();
        info!("round: {}", state.turn());
        info!("score : {}", evaluate(state, 1, &b));

        if state.turn() <= 8 {
           // let chosen_move = find_best_start_move( *state);
            let mut s = state.clone();
            let chosen_move = dyn_max( s, _my_team, b);
            return chosen_move.unwrap();
        }
        else {
            let mut s = state.clone();
            let k = dyn_max(s, _my_team, b);
            info!("ControlDiff: {}", get_controlled_fields(&s, 1));
            //info!("Chose move {}", k.unwrap());
            return k.unwrap();
        }

        
    }

    fn on_update_state(&mut self, state: &State) {
        debug!("Board:\n{}", state.board());
    }
}
