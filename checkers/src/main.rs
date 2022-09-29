mod board; 
mod ai; 
use board::Board;
use std::io::stdin;


fn game_loop (red_mover: fn(&mut Board) -> usize, black_mover: fn(&mut Board) -> usize) {
    let mut b:Board = Board::new(); 

    let (mut is_game_over, mut is_tie, mut winner) = b.is_game_over();
    while !is_game_over {
        println!("{:}",b);
        b.print_moves();
        let mv = match b.get_current_player() {
            board::Player::Red => red_mover(&mut b),
            board::Player::Black => black_mover(&mut b)
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

fn main() {
    game_loop(read_user_input, read_user_input);
}


fn read_user_input(b: &mut Board) -> usize {

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
