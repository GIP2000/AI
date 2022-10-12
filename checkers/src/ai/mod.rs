use crate::board::Board;
use std::i32::MAX;
use std::time::SystemTime;
const MIN:i32 = -MAX;

#[derive(Clone,Copy)]
enum ABResultType{
    TimeLimitExpired,
    DepthReached,
    Finished,
    Inital,
}

enum EndEarly {
    Yes((i32, ABResult)),
    No
}

struct ABResult {
    r#type: ABResultType,
    value: Option<usize>
}


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
        // println!("Starting depth {:}, time is {:}ms", d, now.elapsed().expect("Err: Invalid Sys time").as_millis());
        let (_, v) = max_value(b.clone(), d, MIN, MAX, ((time_limit as u128) * 1000) - 100, &now);
        match v.r#type {
            ABResultType::Finished => {
                println!("Found Bottom Depth: {:?}", d);
                return v.value.expect("Err: Finished without value");
            },
            ABResultType::TimeLimitExpired => {
                println!("Time limit expired in depth {:?}, curreint time is {:?}", d, now.elapsed().expect("Err: Invalid Sys time").as_millis());
                return mv;
            },
            ABResultType::DepthReached => {
                println!("Finishe depth {:?}, curreint time is {:?}", d, now.elapsed().expect("Err: Invalid Sys time").as_millis());
                mv = v.value.expect("Err: No DepthReached without value");
            },
            ABResultType::Inital => {
                panic!("Unreachable Inital value");
            }
        };
        d+=1;
    }
}


fn is_terminal(state: &Board, depth: u32, time_limit: u128, now: &SystemTime, is_max: bool) -> EndEarly {
    if now.elapsed().expect("Err: Invalid Sys time").as_millis() >= time_limit {
        return EndEarly::Yes((0, ABResult{
            r#type: ABResultType::TimeLimitExpired,
            value: None
        }));
    }
    let (is_game_over, is_tie_op, winner) = state.is_game_over();
    if is_game_over {
        let (is_tie, winner) = (is_tie_op.unwrap(), winner.unwrap());
        if is_tie {
            return EndEarly::Yes((0, ABResult{
                r#type: ABResultType::Finished,
                value: None
            }));
        }
        if (winner == state.get_current_player() && is_max) || (winner != state.get_current_player() && !is_max) {
            return EndEarly::Yes((MAX, ABResult{
                r#type: ABResultType::Finished,
                value: None
            }));
        }
        return EndEarly::Yes((MIN, ABResult{
            r#type: ABResultType::Finished,
            value: None
        }));
    }
    if depth == 0 {
        return EndEarly::Yes((h(&state, is_max), ABResult {
            r#type: ABResultType::DepthReached,
            value: None
        }));
    }
    EndEarly::No

}

fn max_value(state: Board, depth: u32, mut alpha: i32, beta: i32, time_limit: u128, now: &SystemTime) -> (i32, ABResult){
    match is_terminal(&state, depth, time_limit, now, true) {
        EndEarly::Yes(r) => return r,
        EndEarly::No => {}
    };

    let mut v = MIN;
    let mut mv = ABResult {
        r#type: ABResultType::Inital,
        value: Some(0)
    };
    for p_mv in 0..state.get_player_info().borrow().get_moves().len() {
        let mut new_state = state.clone();
        new_state.do_move(p_mv);
        let (v2, t_move) = min_value(new_state,depth-1,alpha,beta,time_limit,now);
        // should I update stuff
        if v2 > v {
            v = v2;
            mv.r#type = t_move.r#type;
            mv.value = Some(p_mv);
            if v > alpha {
                alpha = v;
            }
        }
        // time limit expired get out
        if let ABResultType::TimeLimitExpired = t_move.r#type {
            return (v, t_move);
        }
        // should I prune
        if v >= beta {
            return (v,mv);
        }
    }
    return (v,mv);

}
fn min_value(state: Board, depth: u32, alpha: i32, mut beta: i32, time_limit: u128, now: &SystemTime) -> (i32, ABResult){
    match is_terminal(&state, depth, time_limit, now, false) {
        EndEarly::Yes(r) => return r,
        EndEarly::No => {}
    };

    let mut v = MAX;
    let mut mv = ABResult {
        r#type: ABResultType::Inital,
        value: Some(0)
    };
    for p_mv in 0..state.get_player_info().borrow().get_moves().len() {
        let mut new_state = state.clone();
        new_state.do_move(p_mv);
        let (v2,t_move) = max_value(new_state,depth-1, alpha,beta,time_limit, now);
        if v2 < v {
            v = v2;
            mv.r#type = t_move.r#type;
            mv.value = Some(p_mv);
            if v < beta {
                beta = v
            }
        }

        // time limit expired get out
        if let ABResultType::TimeLimitExpired = t_move.r#type {
            return (v, t_move);
        }

        if v <= alpha {
            return (v,mv);
        }
    }
    return (v,mv);
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
