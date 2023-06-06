use crate::game::{State, Team, Vec2, Doubled};


fn own_is_valid(cord:Vec2<Doubled>) -> bool{
    return 0 <= cord.x && cord.x < 8 && 0 <= cord.y && cord.y < 8
}

fn get_spot_name(gamestate:&State, root:Vec2<Doubled>) -> char {
    let mut count = 0;
    let mut redspot = false;
    let mut blackspot = true;
    let mut empty_mirror = false;

    let mut mirror = true;

    let mut neighbors:Vec<bool> = vec![];

    for n in root.hex_neighbors() {
        if own_is_valid(n) {
            neighbors.push(gamestate.board()[n].fish() > 0)
        } else {
            neighbors.push(false)
        }
    }

    for i in 0..6 {
        if neighbors[i] {
            if neighbors[(i+1) % 6] {
                count += 1;
                blackspot = false;
            } else if !neighbors[(i-1)%6] {
                count += 1;
                redspot = true;
            }
        } else if !neighbors[(i-3)%6] {
            empty_mirror = true;
        }
        else if neighbors[(i-3)%6] {
            mirror = false
        }    
    }
    if empty_mirror && count == 2 {
        redspot = true;
    }
    let mut spot = 'w';
    if count == 1 && redspot {
        spot = 'y'
    } else if blackspot && !mirror {
        spot = 'b'
    }
    else if redspot {
        spot = 'r'
    }
    return spot;
}

const scores:&[f32] = &[0.85, -0.6, 0.3, -1.1];
// const scores:&[f32] = &[1.3, 0.9, -0.7, -2.];
pub fn get_spot_score(gamestate:&State, spot:Vec2<Doubled>) -> f32 {
    let name = get_spot_name(gamestate, spot);
    if name== 'y'  {
        return scores[0]
    } else if name == 'w' {
        return scores[1]
    } else if name == 'r' {
        return scores[2]
    } 
    return scores[3]; 
}

pub fn get_pingu_spot_scores(gamestate:&State, team:Team) -> f32 {
    let pingus2 = gamestate.pieces_of(team);
    let mut d = 0.0;
    for p1 in pingus2 {
        d += get_spot_score(gamestate, p1.0)
    }
    return d;
}

pub fn get_spot_scores(gamestate:&State, my_turn:i32) -> f32 {
    my_turn as f32 * ( 
        get_pingu_spot_scores(&gamestate, gamestate.current_team()) -
        get_pingu_spot_scores(&gamestate, gamestate.current_team().opponent())
    )
}