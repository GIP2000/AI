mod board; 
mod ai; 
use board::Board;
use std::io::stdin;
fn main() {
    let mut b:Board = Board::new(); 
    loop {
        println!("{:}",b);
        b.print_moves();
        while !b.do_move(read_user_input()) {
            println!("Please Enter a Number in the range");
        }
    }
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
