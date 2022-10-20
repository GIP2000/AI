pub mod heuristic;
mod visualize_tree_ai;
use crate::board::{Board, Moves};
use heuristic::Heuristic;
use std::fs::OpenOptions;
use std::i32::MAX;
use std::io::Write;
use std::time::SystemTime;
use visualize_tree_ai::{RTTree, Tree};
const MIN: i32 = -MAX;

#[derive(Clone, Copy)]
enum ABResult {
    TimeLimitExpired,
    DepthReached(Option<usize>),
    Finished(Option<usize>),
    Inital,
}

impl ABResult {
    fn set(mut self, val: usize) -> Self {
        match self {
            Self::DepthReached(ref mut wv) => {
                *wv = Some(val);
            }
            Self::Finished(ref mut wv) => {
                *wv = Some(val);
            }
            _ => {}
        }
        self
    }
}

pub fn predict_move(b: Board, time_limit: u32, h_s_param: Option<Heuristic>) -> usize {
    let player_info_rc = b.get_player_info();
    let player_info = player_info_rc.borrow();

    if player_info.get_moves().len() == 1 {
        // if there is only one move do it
        return 0;
    }
    drop(player_info);
    let mut d = 1;
    let mut mv = 0;

    // this creates a tree in debug mode
    // This match statment should always be compiled out
    let mut tree: Option<Tree<RTTree>> = match cfg!(debug_assertions) {
        true => Option::Some(Tree::new(RTTree {
            h_val: 0,
            mv: Moves::new_empty(),
            is_max: true,
            alpha: MIN,
            beta: MAX,
            pruned: false,
        })),
        false => Option::None,
    };
    println!("Starting AB/P");
    let h_s = h_s_param.unwrap_or_else(|| Heuristic::default_new());
    let now = SystemTime::now();
    let time_limit = ((time_limit as u128) * 1000) - 100;
    loop {
        let mut inner_tree: Option<Tree<RTTree>> = match cfg!(debug_assertions) {
            true => Option::Some(Tree::new(RTTree {
                h_val: 0,
                mv: Moves::new_empty(),
                is_max: true,
                alpha: MIN,
                beta: MAX,
                pruned: false,
            })),
            false => Option::None,
        };
        let (_, v) = max_value(
            b.clone(),
            d,
            MIN,
            MAX,
            time_limit,
            &now,
            &h_s,
            &mut inner_tree,
        );
        match v {
            ABResult::Finished(value) => {
                println!("Found Bottom Depth: {:?}", d);
                #[cfg(debug_assertions)]
                {
                    let mut f = OpenOptions::new()
                        .create(true)
                        .truncate(true)
                        .write(true)
                        .open("tree.json")
                        .expect("FS Error");

                    write!(
                        f,
                        "{}",
                        serde_json::to_string(&(inner_tree.expect("No Tree"))).unwrap()
                    )
                    .expect("Error Writting");
                }
                return value.expect("Err: Finished without value");
            }
            ABResult::TimeLimitExpired => {
                println!(
                    "Time limit expired in depth {:?}, current time is {:?}",
                    d,
                    now.elapsed().expect("Err: Invalid Sys time").as_millis()
                );
                #[cfg(debug_assertions)]
                {
                    let mut f = OpenOptions::new()
                        .create(true)
                        .truncate(true)
                        .write(true)
                        .open("tree.json")
                        .expect("FS Error");

                    write!(
                        f,
                        "{}",
                        serde_json::to_string(&(tree.expect("No Tree"))).unwrap()
                    )
                    .expect("Error Writting");
                }
                return mv;
            }
            ABResult::DepthReached(value) => {
                println!(
                    "Finished depth {:?}, current time is {:?}",
                    d,
                    now.elapsed().expect("Err: Invalid Sys time").as_millis()
                );
                mv = value.expect("Err: No DepthReached without value");
                #[cfg(debug_assertions)]
                {
                    tree = inner_tree;
                }
            }
            ABResult::Inital => {
                if check_time_limit(time_limit, &now) {
                    println!(
                        "Time limit expired in depth {:?}, current time is {:?}",
                        d,
                        now.elapsed().expect("Err: Invalid Sys time").as_millis()
                    );
                    #[cfg(debug_assertions)]
                    {
                        let mut f = OpenOptions::new()
                            .create(true)
                            .truncate(true)
                            .write(true)
                            .open("tree.json")
                            .expect("FS Error");

                        write!(f, "{}", serde_json::to_string(&inner_tree).unwrap())
                            .expect("Error Writting");
                    }
                    return mv;
                }
                println!(
                    "Finished depth {:?}, current time is {:?}",
                    d,
                    now.elapsed().expect("Err: Invalid Sys time").as_millis()
                );
                #[cfg(debug_assertions)]
                {
                    tree = inner_tree;
                }
                mv = 0;
            }
        };
        d += 1;
    }
}

fn check_time_limit(time_limit: u128, now: &SystemTime) -> bool {
    now.elapsed().expect("Err: Invalid Sys time").as_millis() >= time_limit
}

fn is_terminal(
    state: &Board,
    depth: u32,
    time_limit: u128,
    now: &SystemTime,
    is_max: bool,
    h_s: &Heuristic,
) -> Result<(i32, ABResult), ()> {
    if check_time_limit(time_limit, now) {
        return Result::Ok((0, ABResult::TimeLimitExpired));
    }
    if let Some(winner) = state.is_game_over() {
        if (winner == state.get_current_player() && is_max)
            || (winner != state.get_current_player() && !is_max)
        {
            return Result::Ok((MAX - depth as i32, ABResult::Finished(None)));
        }
        return Result::Ok((MIN + depth as i32, ABResult::Finished(None)));
    }
    if depth == 0 {
        return Result::Ok((h_s.h(&state, is_max), ABResult::DepthReached(None)));
    }
    Result::Err(())
}

fn max_value(
    state: Board,
    depth: u32,
    mut alpha: i32,
    beta: i32,
    time_limit: u128,
    now: &SystemTime,
    h_s: &Heuristic,
    tree: &mut Option<Tree<RTTree>>,
) -> (i32, ABResult) {
    match is_terminal(&state, depth, time_limit, now, true, h_s) {
        Result::Ok(r) => return r,
        Result::Err(_) => {}
    };

    let mut v = MIN;
    let mut mv = ABResult::Inital;
    for p_mv in 0..state.get_player_info().borrow().get_moves().len() {
        let mut inner_tree: Option<Tree<RTTree>> = match cfg!(debug_assertions) {
            true => Option::Some(Tree::new(RTTree {
                h_val: 0,
                mv: state.get_player_info().borrow().get_moves()[p_mv].clone(),
                is_max: true,
                alpha,
                beta,
                pruned: false,
            })),
            false => Option::None,
        };
        let mut new_state = state.clone();
        new_state.do_move(p_mv);
        let (v2, t_move) = min_value(
            new_state,
            depth - 1,
            alpha,
            beta,
            time_limit,
            now,
            h_s,
            &mut inner_tree,
        );
        // should I update stuff
        if v2 > v {
            v = v2;
            mv = t_move.set(p_mv);
            if v > alpha {
                alpha = v;
            }
        }

        #[cfg(debug_assertions)]
        {
            inner_tree.as_mut().unwrap().val.h_val = v2;
            inner_tree.as_mut().unwrap().val.alpha = alpha;
            inner_tree.as_mut().unwrap().val.beta = beta;
            tree.as_mut().unwrap().push(inner_tree.unwrap());
        }
        // time limit expired get out
        if let ABResult::TimeLimitExpired = t_move {
            return (v, t_move);
        }
        // should I prune
        if v >= beta {
            #[cfg(debug_assertions)]
            {
                for pruned_mv in p_mv..state.get_player_info().borrow().get_moves().len() {
                    tree.as_mut().unwrap().push(Tree::new(RTTree {
                        alpha,
                        beta,
                        h_val: 0,
                        mv: state.get_player_info().borrow().get_moves()[pruned_mv].clone(),
                        is_max: true,
                        pruned: true,
                    }));
                }
            }
            return (v, mv);
        }
    }
    return (v, mv);
}

fn min_value(
    state: Board,
    depth: u32,
    alpha: i32,
    mut beta: i32,
    time_limit: u128,
    now: &SystemTime,
    h_s: &Heuristic,
    tree: &mut Option<Tree<RTTree>>,
) -> (i32, ABResult) {
    match is_terminal(&state, depth, time_limit, now, false, h_s) {
        Result::Ok(r) => return r,
        Result::Err(_) => {}
    };

    let mut v = MAX;
    let mut mv = ABResult::Inital;
    for p_mv in 0..state.get_player_info().borrow().get_moves().len() {
        let mut inner_tree: Option<Tree<RTTree>> = match cfg!(debug_assertions) {
            true => Option::Some(Tree::new(RTTree {
                h_val: 0,
                mv: state.get_player_info().borrow().get_moves()[p_mv].clone(),
                is_max: false,
                alpha,
                beta,
                pruned: false,
            })),
            false => Option::None,
        };
        let mut new_state = state.clone();
        new_state.do_move(p_mv);
        let (v2, t_move) = max_value(
            new_state,
            depth - 1,
            alpha,
            beta,
            time_limit,
            now,
            h_s,
            &mut inner_tree,
        );

        if v2 < v {
            v = v2;
            mv = t_move.set(p_mv);
            if v < beta {
                beta = v
            }
        }

        #[cfg(debug_assertions)]
        {
            inner_tree.as_mut().unwrap().val.h_val = v2;
            inner_tree.as_mut().unwrap().val.alpha = alpha;
            inner_tree.as_mut().unwrap().val.beta = beta;
            tree.as_mut().unwrap().push(inner_tree.unwrap());
        }

        // time limit expired get out
        if let ABResult::TimeLimitExpired = t_move {
            return (v, t_move);
        }

        if v <= alpha {
            #[cfg(debug_assertions)]
            {
                for pruned_mv in p_mv..state.get_player_info().borrow().get_moves().len() {
                    tree.as_mut().unwrap().push(Tree::new(RTTree {
                        alpha,
                        beta,
                        h_val: 0,
                        mv: state.get_player_info().borrow().get_moves()[pruned_mv].clone(),
                        is_max: false,
                        pruned: true,
                    }));
                }
            }
            return (v, mv);
        }
    }
    return (v, mv);
}
