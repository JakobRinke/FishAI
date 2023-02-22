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
        

        // 0.34
        // let a: &[f32] = &[5.0, -0.2, 0.15, 1.435609120279965, 2.5, 2.3];

        // 0.36
        // let a: &[f32] = &[5.0, -0.3, 0.15, 1.435609120279965, 4.5, 0.8];

        // 0.44
        // let a: &[f32] = &[4.0, -0.3, 0.2, 1.5, 7., 0.8];
        
        // 0.46; 500: 0.48
        // let a: &[f32] = &[3.0, -0.5, 0.3, 1.9, 7.3, 0.5];

        // 0.42 (32, 10)
        // let a: &[f32] = &[3.5, -0.6, 0.25, 0.3, 7.3, 2.5];

        // 0.46    /   0.40    ; 500:      
        //let a: &[f32] = &[4.0, -0.45, 0.3, 0.8, 7.3, 0.35];

        // 0.36
        // let a: &[f32] = &[3.0, -0.35, 0.3, 0.2, 7.3, 0.45];

        // 0.45 (29 - 16)
         let a: &[f32] = &[3.8, -0.55, 0.3, 1.4, 8.3, 0.25];

        // 0.44 
        // let a: &[f32] = &[3.9, -0.5, 0.3, 1.6, 7.7, 0.3];

        // 0.56 + 0.46 = 0.51; 0.44; 42; 500: 0.43
        // let a: &[f32] = &[2.9, -0.5, 0.3, 1.5, 7.3, 0.45];

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
            //info!("Chose move {}", k.unwrap());
            return k.unwrap();
        }

        
    }

    fn on_update_state(&mut self, state: &State) {
        debug!("Board:\n{}", state.board());
    }
}
