use log::{info, debug};
use rand::seq::SliceRandom;
use socha_client_2023::{client::GameClientDelegate, game::{Move, Team, State}};
use socha_client_2023::scoring_funcs::*;

/// An empty game logic structure that implements the client delegate trait
/// and thus is responsible e.g. for picking a move when requested.
pub struct OwnLogic;

impl GameClientDelegate for OwnLogic {
    fn request_move(&mut self, state: &State, _my_team: Team) -> Move {
        info!("Requested move");
        info!("{}", get_move_num(state, 1));
        let chosen_move = *state.possible_moves()
            .choose(&mut rand::thread_rng())
            .expect("No move found!");
        info!("Chose move {}", chosen_move);
        chosen_move
    }

    fn on_update_state(&mut self, state: &State) {
        debug!("Board:\n{}", state.board());
    }
}
