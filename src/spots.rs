use crate::game::{State, Team, Vec2, Doubled};


fn own_is_valid(cord:Vec2<Doubled>) {
    return 0 <= cord.x < 8 && 0 <= cord.y < 8
}

fn get_spot_name(gamestate:&State, root:Vec2<Doubled>) -> str {
    let mut count = 0;
    let mut retspot = false;
    let mut blackspot = true;
    let mut empty_mirror = false;
    let mut mirror = true;

    let mut neighbours:Vec<bool> = vec![];

    for n in root.hex_neighbors() {
        if own_is_valid(n) {
            neighbors.append(gamestate.board()[n].fish() > 0)
        } else {
            neighbors.append(false)
        }
    }

    for u in 0..6 {
        if neighbors[i] {
            if neighbors[(i+1) % 6] {
                count += 1;
                blackspot = false;
            } else if !neighbors[i-1] {
                count += 1;
                redspot = true;
            }
        } else if !neighbors[i-3] {
            empty_mirror = true;
        }
        else if neighbors[i-3] {
            mirror = false
        }    
    }
    if empty_mirror && count == 2 {
        redspot = true;
    }
    let mut spot = "white";
    if count == 1 && redspot {
        spot = "yellow"
    } else if blackspot && !mirror {
        spot = "black"
    }
    else if redspot {
        spot = "red"
    }
    return spot;
}

const scores:[f32] = [1.5, 1, -0.8, -2];
pub fn get_spot_score(gamestate:State, spot:Vec2<Doubled>) -> f32 {
    let name = get_spot_name(gamestate, spot);
    if name == "yellow" {
        scores[0]
    } else if name == "white" {
        scores[1]
    } else if name == "red" {
        scores[2]
    } 
    return scores[3]; 
}

pub fn get_pingu_spot_scores(gamestate:&State, team:Team) -> f32 {
    let pingus2 = gamestate.pieces_of(team2);
    let mut d = 0.0;
    for p1 in pingus2 {
        d += get_spot_score(state, p1.0)
    }
    return d;
}

pub fn get_spot_scores(gamestate:&State, my_turn:f32) -> f32 {
    my_turn as f32 * ( 
        get_pingu_spot_scores(&gamestate, gamestate.current_team()) -
        get_pingu_spot_scores(&gamestate, gamestate.current_team().opponent())
    )
}