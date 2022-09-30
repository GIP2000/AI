mod board; 
mod ai; 
use board::{Board,Player};
use ai::predict_move;
use std::io::stdin;

type MoveFinderType = fn(Board) -> usize;

fn game_loop (b: &mut Board, red_mover: MoveFinderType, black_mover: MoveFinderType ) {
    let (mut is_game_over, mut is_tie, mut winner) = b.is_game_over();
    while !is_game_over {
        println!("{:}",b);
        b.print_moves();
        let mv = match b.get_current_player() {
            Player::Red => red_mover(b.clone()),
            Player::Black => black_mover(b.clone())
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


fn read_user_input(b:Board) -> usize {

    let mut s = String::new();
    match stdin().read_line(&mut s) {
        Err(_) => {
            println!("Please Enter a Valid Number");
            read_user_input(b)
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
                    read_user_input(b)
                },
                Ok(x) => {
                   x as usize 
                }
            }
        }
    }

}


fn get_single_game_mode(input: &str) -> fn(Board)->usize {
    println!("{:}",input);
    let mut s = String::new(); 
    match stdin().read_line(&mut s) {
        Err(_) => {
            println!("Invalid Input");
            get_single_game_mode(input)
        },
        Ok(_) => {
            if let Some('\n')=s.chars().next_back() {
                s.pop();
            }
            if let Some('\r')=s.chars().next_back() {
                s.pop();
            }

            match s.as_str() {
                "y" => read_user_input,
                "n" => predict_move,
                _=> {
                    println!("Invalid Input please enter y/n");
                    get_single_game_mode(input)
                }
            }

        }
    }

}



fn get_game_mode() -> (MoveFinderType, MoveFinderType) {
    (
        get_single_game_mode("Would you like to play for red (y/n)"),
        get_single_game_mode("Would you like to play for Blue (y/n)")
    )
}

fn main() {
    let mut b = Board::new(false); 

    let (red,black) = get_game_mode(); 
    

    game_loop(&mut b, red, black);
}
