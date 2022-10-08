mod board;
mod ai;
use board::{Board,Player};
use ai::predict_move;
use std::io::stdin;
use std::fs::read_to_string;

type MoveFinderType = fn(Board, time_in_sec: u32) -> usize;

fn game_loop (b: &mut Board, red_mover: MoveFinderType, black_mover: MoveFinderType, time_limit: u32 ) {
    let (mut is_game_over, mut is_tie, mut winner) = b.is_game_over();
    while !is_game_over {
        println!("{:}",b);
        b.print_moves();
        let mv = match b.get_current_player() {
            Player::Red => red_mover(b.clone(), time_limit),
            Player::Black => black_mover(b.clone(), time_limit)
        };

        while !b.do_move(mv) {
            println!("Please Enter a Number in the range");
        }
        (is_game_over, is_tie, winner) = b.is_game_over();
    }
    println!("Game Over!");
    println!("{:}",b);
    if is_tie.unwrap() {
        println!("It was a Tie");
    }
    println!("Player {:?} wins",winner.unwrap())
}


fn read_user_input(b:Board, time_limit: u32) -> usize {

    let mut s = String::new();
    match stdin().read_line(&mut s) {
        Err(_) => {
            println!("Please Enter a Valid Number");
            read_user_input(b, time_limit)
        },
        Ok(_) => {
            if let Some('\n')=s.chars().next_back() {
                s.pop();
            }
            if let Some('\r')=s.chars().next_back() {
                s.pop();
            }
            match s.parse::<u32>() {
                Err(_) => {
                    println!("Please Enter a Valid Number");
                    read_user_input(b, time_limit)
                },
                Ok(x) => {
                   x as usize
                }
            }
        }
    }

}

fn confirm(input: &str) -> bool {
    println!("{:}", input);
    let mut s = String::new();
    match stdin().read_line(&mut s) {
        Err(_) => {
            println!("Invalid Input");
            confirm(input)
        },
        Ok(_) => {
            if let Some('\n')=s.chars().next_back() {
                s.pop();
            }
            if let Some('\r')=s.chars().next_back() {
                s.pop();
            }

            match s.as_str() {
                "y" => true,
                "n" => false,
                _=> {
                    println!("Invalid Input please enter y/n");
                    confirm(input)
                }
            }
        }
    }
}


fn get_single_game_mode(input: &str) -> fn(Board, time_limit: u32)->usize {
    match confirm(input) {
        true=> read_user_input,
        false=> predict_move,
    }
}


fn get_game_mode() -> (MoveFinderType, MoveFinderType) {
    (
        get_single_game_mode("Would you like to play for red (y/n)"),
        get_single_game_mode("Would you like to play for Blue (y/n)")
    )
}


fn get_init_board() -> Option<String> {
    match confirm("Would you like to Input a Board Path (y/n)") {
        true => {
            let mut path = String::new();
            fn read_path(inner_s: &mut String) {
                println!("Please Input a Valid Path");
                if stdin().read_line(inner_s).is_err() {
                    println!("Error Reading Input");
                    read_path(inner_s)
                }
            }
            read_path(&mut path);
            match read_to_string(&path) {
                Err(_) => {
                    println!("Error: Invalid File Path {:}, Creating a Default Board", path);
                    None
                },
                Ok(s) => {
                    Some(s)
                }
            }
        },
        false => {
            None
        }
    }
}

fn main() {
    let mut b = Board::new(get_init_board());
    let (red,black) = get_game_mode();
    game_loop(&mut b, red, black, 20);
}
