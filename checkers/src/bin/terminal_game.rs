use checkers::ai::{heuristic::Heuristic, predict_move};
use checkers::board::{Board, Player};
use std::fs::read_to_string;
use std::io::stdin;

type AutomatedMoveFinder = fn(Board, time_in_sec: u32, Option<Heuristic>) -> usize;
type ManualMoveFinder = fn() -> usize;

enum MoveFinder {
    Automated(AutomatedMoveFinder),
    Manual(ManualMoveFinder),
}

fn game_loop(b: &mut Board, red_mover: MoveFinder, black_mover: MoveFinder, time_limit: u32) {
    let mut is_game_over = b.is_game_over();
    while let None = is_game_over {
        println!("{:}", b);
        b.print_moves();
        let mv = match b.get_current_player() {
            Player::Red => &red_mover,
            Player::Black => &black_mover,
        };

        loop {
            let m = match mv {
                MoveFinder::Manual(f) => f(),
                MoveFinder::Automated(f) => f(b.clone(), time_limit, None),
            };
            if b.do_move(m) {
                println!("Move {} was chosen", m);
                break;
            }
            println!("Please Enter a Number in the range");
            println!("You Tried to do {:?}", m);
        }
        is_game_over = b.is_game_over();
    }
    println!("Game Over!");
    println!("{:}", b);
    println!("Player {:?} wins", is_game_over.expect("Unrechable"));
}

fn read_number(input: &str) -> u32 {
    println!("{:}", input);
    let mut s = String::new();
    match stdin().read_line(&mut s) {
        Err(_) => {
            println!("Please Enter a Valid Number");
            read_number(input)
        }
        Ok(_) => {
            if let Some('\n') = s.chars().next_back() {
                s.pop();
            }
            if let Some('\r') = s.chars().next_back() {
                s.pop();
            }
            match s.parse::<u32>() {
                Err(_) => {
                    println!("Please Enter a Valid Number");
                    read_number(input)
                }
                Ok(x) => x,
            }
        }
    }
}

fn read_user_input() -> usize {
    read_number("Please Pick a Move") as usize
}

fn confirm(input: &str) -> bool {
    println!("{:}", input);
    let mut s = String::new();
    match stdin().read_line(&mut s) {
        Err(_) => {
            println!("Invalid Input");
            confirm(input)
        }
        Ok(_) => {
            if let Some('\n') = s.chars().next_back() {
                s.pop();
            }
            if let Some('\r') = s.chars().next_back() {
                s.pop();
            }

            match s.as_str() {
                "y" => true,
                "n" => false,
                _ => {
                    println!("Invalid Input please enter y/n");
                    confirm(input)
                }
            }
        }
    }
}

fn get_single_game_mode(input: &str) -> MoveFinder {
    match confirm(input) {
        true => MoveFinder::Manual(read_user_input),
        false => MoveFinder::Automated(predict_move),
    }
}

fn get_game_mode() -> (MoveFinder, MoveFinder) {
    (
        get_single_game_mode("Would you like to play for red (y/n)"),
        get_single_game_mode("Would you like to play for Blue (y/n)"),
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
            if let Some('\n') = path.chars().next_back() {
                path.pop();
            }
            if let Some('\r') = path.chars().next_back() {
                path.pop();
            }

            match read_to_string(&path) {
                Err(_) => {
                    println!(
                        "Error: Invalid File Path {:}, Creating a Default Board",
                        path
                    );
                    None
                }
                Ok(s) => Some(s),
            }
        }
        false => None,
    }
}

fn get_time_limit(init: &Option<String>) -> u32 {
    match init {
        Some(fs) => match fs.lines().nth(9) {
            Some(s) => s.parse::<u32>().unwrap_or_else(|_| {
                println!("Error Reading file could not parse 9th line to u32");
                read_number("Please enter a time limit in seconds")
            }),
            None => {
                println!("Error Reading file no 9th line found");
                read_number("Please enter a time limit in seconds")
            }
        },
        None => read_number("Please enter a time limit in seconds"),
    }
}

fn main() {
    let init = get_init_board();
    let mut b = Board::new(&init);
    let (red, black) = get_game_mode();
    let time_limit = get_time_limit(&init);
    game_loop(&mut b, red, black, time_limit);
}
