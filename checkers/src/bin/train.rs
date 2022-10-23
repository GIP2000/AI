use checkers::ai::{heuristic::Heuristic, predict_move};
use checkers::board::{Board, Player};
use std::thread;

type GameResult = (u32, Heuristic);

const TIME_LIMIT: u32 = 5;

fn game_loop(red_h: Heuristic, black_h: Heuristic, child_num: u32) -> GameResult {
    let mut b = Board::new(&Option::None);
    let mut is_game_over = b.is_game_over();
    let mut red_counter = 0;
    let mut black_counter = 0;
    let mut time_since_last_jump = 0;
    while let None = is_game_over {
        if red_counter + black_counter % 20 == 0 {
            println!("Each player in game {} has done 20 moves", child_num);
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

        if time_since_last_jump > 50 {
            break;
        }
        b.do_move(m);
        is_game_over = b.is_game_over();
    }
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
    let mut children = vec![];
    for i in 0..siblings {
        let base = prev.clone();
        children.push(thread::spawn(move || {
            let red_h = if i == 0 { base.clone() } else { base.mutate(5) };
            let black_h = base.mutate(5);
            println!("Starting game {}", i);
            return game_loop(red_h, black_h, i);
        }));
    }

    let mut best_result: GameResult = (std::u32::MAX, prev);

    for child in children {
        let g_result = child.join().expect("Thread Crashed");
        if g_result.0 < best_result.0 {
            best_result = g_result
        }
    }

    best_result
}

fn main() {
    let mut h = Heuristic::default_new();
    for i in 0..10 {
        let (c, nh) = run_generation(h, 5, i);
        println!(
            "Generation {} ended selected new h: {:?} with c {}",
            i, nh, c
        );
        h = nh;
    }
}
