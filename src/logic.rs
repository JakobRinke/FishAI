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
        
        //let a: &[f32] = &[2.3465499896565225, 0.9509269204004758, 40.0, -0.6819216789064733, -0.5364894515536918, 40.0, 2.2913894724952417, 0.2864972300261446, 40.0];
        //let a: &[f32] = &[3.9557836482982456, 0.4177128264965727, 40.0, -0.7039701243668675, -1.6635912470787138, 40.0, -1.578680770902127, 0.22007119163660083, 40.0];
        //let a: &[f32] = &[4.135102895853603, -2.8017075937684055, 40.0, 0.5253584493618249, 1.435609120279965, 40.0, -2.899542746951548, -0.09057006397037526, 40.0];

        //let a: &[f32] = &[1.0, -0.8017075937684055, 40.0, 0.1, 0.4, 40.0, 20.0, 1.2, 40.0];

      //  let a: &[f32] = &[1.0, -2.8017075937684055, 40.0, 0.15, 1.435609120279965, 40.0, 8.0, 0.2, 40.0];

        let a = &[5.358816234399903, -2.436932536885531, 40.0, 0.027487167696233238, 1.6942336778007334, 40.0, -2.2714511358914273, 0.5504245063144049, 40.0];

        //let a: &[f32] = &[1.8509686217366212, -4.510831156467119, 40.0, -1.9237961917478072, -4.134668419156508, 40.0, 2.4400472142636165, 1.0910846883723142, 40.0];

        
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
