use crate::board::Board;
use std::i32::MAX;
use std::time::SystemTime;
const MIN:i32 = -MAX;

#[derive(Clone,Copy)]
enum ABResult{
    TimeLimitExpired,
    DepthReached(Option<usize>),
    Finished(Option<usize>),
    Inital,
}

impl ABResult {

    fn set(mut self, val: usize) -> Self{
        match self {
            Self::DepthReached(ref mut wv) => {
                *wv = Some(val);
            },
            Self::Finished(ref mut wv) => {
                *wv = Some(val);
            },
            _=> {}
        }
        self
    }
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
    let time_limit = ((time_limit as u128) * 1000) - 100;
    loop {
        let (_, v) = max_value(b.clone(), d, MIN, MAX, time_limit, &now);
        match v {
            ABResult::Finished(value) => {
                println!("Found Bottom Depth: {:?}", d);
                return value.expect("Err: Finished without value");
            },
            ABResult::TimeLimitExpired => {
                println!("Time limit expired in depth {:?}, current time is {:?}", d, now.elapsed().expect("Err: Invalid Sys time").as_millis());
                return mv;
            },
            ABResult::DepthReached(value) => {
                println!("Finished depth {:?}, current time is {:?}", d, now.elapsed().expect("Err: Invalid Sys time").as_millis());
                mv = value.expect("Err: No DepthReached without value");
            },
            ABResult::Inital => {
                if check_time_limit(time_limit, &now) {
                    println!("Time limit expired in depth {:?}, current time is {:?}", d, now.elapsed().expect("Err: Invalid Sys time").as_millis());
                    return mv;
                }
                println!("Finished depth {:?}, current time is {:?}", d, now.elapsed().expect("Err: Invalid Sys time").as_millis());
                mv = 0;
            }
        };
        d+=1;
    }
}

fn check_time_limit(time_limit: u128, now: &SystemTime) -> bool {
    now.elapsed().expect("Err: Invalid Sys time").as_millis() >= time_limit
}


fn is_terminal(state: &Board, depth: u32, time_limit: u128, now: &SystemTime, is_max: bool) -> Result<(i32,ABResult), ()> {
    if check_time_limit(time_limit, now){
        return Result::Ok((0, ABResult::TimeLimitExpired));
    }
    let (is_game_over, is_tie_op, winner) = state.is_game_over();
    if is_game_over {
        let (is_tie, winner) = (is_tie_op.unwrap(), winner.unwrap());
        if is_tie {
            return Result::Ok((0, ABResult::Finished(None)));
        }
        if (winner == state.get_current_player() && is_max) || (winner != state.get_current_player() && !is_max) {
            return Result::Ok((MAX, ABResult::Finished(None)));
        }
        return Result::Ok((MIN, ABResult::Finished(None)));
    }
    if depth == 0 {
        return Result::Ok((h(&state, is_max), ABResult::DepthReached(None)));
    }
    Result::Err(())

}

fn max_value(state: Board, depth: u32, mut alpha: i32, beta: i32, time_limit: u128, now: &SystemTime) -> (i32, ABResult){
    match is_terminal(&state, depth, time_limit, now, true) {
        Result::Ok(r) => return r,
        Result::Err(_) => {}
    };

    let mut v = MIN;
    let mut mv = ABResult::Inital;
    for p_mv in 0..state.get_player_info().borrow().get_moves().len() {
        let mut new_state = state.clone();
        new_state.do_move(p_mv);
        let (v2, t_move) = min_value(new_state,depth-1,alpha,beta,time_limit,now);
        // should I update stuff
        if v2 > v {
            v = v2;
            mv = t_move.set(p_mv);
            if v > alpha {
                alpha = v;
            }
        }
        // time limit expired get out
        if let ABResult::TimeLimitExpired = t_move {
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
        Result::Ok(r) => return r,
        Result::Err(_) => {}
    };

    let mut v = MAX;
    let mut mv = ABResult::Inital;
    for p_mv in 0..state.get_player_info().borrow().get_moves().len() {
        let mut new_state = state.clone();
        new_state.do_move(p_mv);
        let (v2,t_move) = max_value(new_state,depth-1, alpha,beta,time_limit, now);
        if v2 < v {
            v = v2;
            mv = t_move.set(p_mv);
            if v < beta {
                beta = v
            }
        }
        // time limit expired get out
        if let ABResult::TimeLimitExpired = t_move {
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
