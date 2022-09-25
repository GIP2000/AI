mod board; 
mod ai; 
use board::Board;
use std::io::stdin;
fn main() {
    let mut b:Board = Board::new(); 

    let (mut is_game_over, mut is_tie, mut winner) = b.is_game_over();
    while !is_game_over {
        println!("{:}",b);
        b.print_moves();
        while !b.do_move(read_user_input()) {
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


fn read_user_input() -> usize {

    let mut s = String::new();
    match stdin().read_line(&mut s) {
        Err(_) => {
            println!("Please Enter a Valid Number");
            read_user_input()
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
                    read_user_input()
                },
                Ok(x) => {
                   x as usize 
                }
            }
        }
    }

}
