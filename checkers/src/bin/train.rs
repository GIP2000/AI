use checkers::ai::{heuristic::Heuristic, predict_move};
use checkers::board::{Board, Player};
use std::sync::{Arc, RwLock};
use std::thread;

type GameResult = (u32, Heuristic);

const TIME_LIMIT: u32 = 5;

fn game_loop(
    red_h: Heuristic,
    black_h: Heuristic,
    child_num: u32,
    time_to_beat: &Arc<RwLock<u32>>,
) -> GameResult {
    let mut b = Board::new(&Option::None);
    if child_num % 2 == 0 {
        b.swap_current_player();
    }
    let mut is_game_over = b.is_game_over();
    let mut red_counter = 0;
    let mut black_counter = 0;
    let mut time_since_last_jump = 0;
    while let None = is_game_over {
        if (red_counter + black_counter) % 20 == 0 {
            println!(
                "The players in game {} have done ({},{}) moves, time since last just is {}",
                child_num, red_counter, black_counter, time_since_last_jump
            );
        }
        if (red_counter + black_counter) % 40 == 0 && red_counter != 0 {
            println!("board for child {}\n{}", child_num, b);
        }
        let m = match b.get_current_player() {
            Player::Red => {
                red_counter += 1;
                predict_move(b.clone(), TIME_LIMIT, Option::Some(red_h.clone()))
            }
            Player::Black => {
                black_counter += 1;
                predict_move(b.clone(), TIME_LIMIT, Option::Some(black_h.clone()))
            }
        };
        if b.get_player_info().borrow().get_moves()[m].is_jump() {
            time_since_last_jump = 0;
        } else {
            time_since_last_jump += 1;
        }
        {
            let t = time_to_beat.read().expect("Poisned Lock");
            if (red_counter + black_counter) % 60 == 0 {
                println!("t = {}", *t);
            }
            if time_since_last_jump > 50 || (std::cmp::max(black_counter, red_counter) > *t) {
                break;
            }
        }

        b.do_move(m);

        is_game_over = b.is_game_over();
    }

    println!(
        "Game {} finished in ~{} moves. winner? {:?}",
        child_num,
        std::cmp::max(red_counter, black_counter),
        is_game_over
    );

    return match is_game_over.unwrap_or_else(|| {
        red_counter = std::u32::MAX;
        black_counter = std::u32::MAX;
        let (my_pieces, other_pieces) = b.get_pieces();
        if my_pieces.len() > other_pieces.len() {
            return b.get_current_player();
        }
        return b.get_current_player().get_other();
    }) {
        Player::Red => (red_counter, red_h),
        Player::Black => (black_counter, black_h),
    };
}

fn run_generation(prev: Heuristic, siblings: u32, generation: u32) -> GameResult {
    println!("Initalizing generation: {}\n h(n) = {:?}", generation, prev);
    let time_to_beat_base = Arc::new(RwLock::new(std::u32::MAX));
    let mut children = vec![];
    for i in 0..siblings {
        let base = prev.clone();
        let time_to_beat = Arc::clone(&time_to_beat_base);
        children.push(thread::spawn(move || {
            let black_h = base.mutate();
            let red_h = base.clone();
            println!("Starting game {}", i);
            let res = game_loop(red_h, black_h, i, &time_to_beat);
            {
                let mut t = time_to_beat.write().expect("Posined Lock");
                if *t > res.0 {
                    *t = res.0;
                }
            }
            return res;
        }));
    }

    let mut best_result: GameResult = (std::u32::MAX, prev);

    for child in children {
        let g_result = child.join().expect("Thread Crashed");
        if g_result.0 < best_result.0 {
            best_result = g_result
        }
    }

    return best_result;
}

fn main() {
    let mut h = Heuristic::default_new();
    for i in 0..10 {
        let (c, nh) = run_generation(h, 50, i);
        println!(
            "Generation {} ended selected new h: {:?} with c {}",
            i, nh, c
        );
        h = nh;
    }
}
