use crate::board::Board;
use std::i32::MAX;
use std::time::SystemTime;
const MIN:i32 = -MAX;

pub fn predict_move(b: Board, time_limit: u32) -> usize {
    let player_info_rc = b.get_player_info();
    let player_info = player_info_rc.borrow();

    if player_info.get_moves().len() == 1 {
        // if there is only one move do it
        return 0;
    }
    drop(player_info);
    let mut d = 1;
    let mut mv = 0;
    println!("Starting AB/P");
    let now = SystemTime::now();
    loop {
        println!("Starting depth {:}, time is {:}ms", d, now.elapsed().expect("Err: Invalid Sys time").as_millis());
        match ab_prune_d(b.clone(), d, MIN, MAX,true, ((time_limit as u128) * 1000) - 100, &now).1 {
            Some(v) => {
                mv = v;
            },
            None => {
                println!("Done Pruning it took {:}ms",now.elapsed().expect("Err: Invalid Sys time").as_millis());
                return mv;
            }
        }
        d+=1;
    }


}

fn h(state:&Board, is_max: bool) -> i32 {
    let (my_pieces, other_pieces) = state.get_pieces();
    let mut score = 0;
    // Piece Worth
    my_pieces.into_iter().for_each(|(piece,_)| {
        if piece.is_king() {
            score += 5;
        } else {
            score += 1;
        }
    });
    other_pieces.into_iter().for_each(|(piece,_)| {
        if piece.is_king() {
            score -= 5;
        } else {
            score -= 1;
        }
    });
    score *= 100; // Piece Worth Multiplier
    // Inverter
    if is_max {
        score
    } else {
        -score
    }
}

fn ab_prune_d(state: Board, depth: u32, mut alpha: i32, mut beta: i32, is_max: bool, time_limit: u128, now: &SystemTime) -> (i32,Option<usize>) {
    if now.elapsed().expect("Err: Invalid Sys time").as_millis() >= time_limit {
        return (0, None);
    }

    let (is_game_over, is_tie_op, winner) = state.is_game_over();
    if is_game_over {
        let (is_tie, winner) = (is_tie_op.unwrap(), winner.unwrap());
        if is_tie {
            return (0, None);
        }
        if (winner == state.get_current_player() && is_max) || winner != state.get_current_player() && !is_max{
            return (MAX, None);
        }
        return (MIN, None);
    }
    if depth == 0 {
        return (h(&state, is_max), None);
    }

    let mut v = MIN;
    let mut mv = 0;
    for p_mv in 0..state.get_player_info().borrow().get_moves().len() {
        let mut new_state = state.clone();
        new_state.do_move(p_mv);
        let v2 = ab_prune_d(new_state,depth-1, alpha, beta, !is_max, time_limit, now).0;
        if (is_max && v2 > v) || (!is_max && v2 < v) {
            (v, mv) = (v2, p_mv);
            match is_max {
                true => {
                    if v > alpha {
                        alpha = v;
                    }
                },
                false => {
                    if v < beta {
                        beta = v;
                    }
                }
            };
        }
        match is_max {
            true=> {
                if v >= beta {
                    return (v,Some(mv));
                }
            },
            false => {
                if v <= alpha {
                    return (v,Some(mv));
                }
            }
        }
    }
    (v,Some(mv))
}
