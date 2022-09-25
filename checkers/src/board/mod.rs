mod err; 
use std::collections::HashSet;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, Copy, Clone, PartialEq,Eq)]
enum BoardPiece {
    Red,
    KingRed, 
    Black, 
    KingBlack, 
    Empty
}

impl BoardPiece {
    fn is_red(&self) -> bool {
        const LAST_RED: u32 = 1; 
        *self as u32 <= LAST_RED 
    }
    
    fn is_black(&self) -> bool {
        const LAST_RED: u32 = 1; 
        const LAST_BLACK: u32 = 3; 
        *self as u32 <= LAST_BLACK && *self as u32 > LAST_RED
    }

    fn is_king (&self) -> bool {
        *self as u32 % 2 != 0 && *self != BoardPiece::Empty
    }

    fn promote (&self) -> Self {
       if self.is_red() {
           Self::KingRed
       } else if self.is_black() {
           Self::KingBlack
       } else {
           Self::Empty
       }
    }

}


#[derive(Debug,Copy,Clone,PartialEq,Eq)]
enum Player {
    Black = 1,
    Red = -1
}

impl Player {
    fn get_other(&self) -> Self {
        match self {
            Self::Black => Self::Red, 
            Self::Red => Self::Black
        }
    }
    
    fn does_piece_match(&self, piece:BoardPiece) -> bool {
        match *self {
            Self::Black => piece.is_black(), 
            Self::Red => piece.is_red()
        }
    }

}

#[derive(Debug,Copy,Clone,PartialEq,Eq)]
pub enum Move {
    ForwardRight, 
    ForwardLeft, 
    Jump,
    BackwardRight = 10, 
    BackwardLeft = 11,
}

impl Move {
    fn must_be_king(&self) -> bool {
        *self as u32 > 9
    }
}

#[derive(Debug,Clone)]
struct Moves {
    jump_path: HashSet<(usize,usize)>, 
    start_loc: (usize,usize), 
    end_loc:(usize,usize)
}
impl std::fmt::Display for Moves {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> Result<(),std::fmt::Error> {
        let (start_row,start_col) = self.start_loc;
        let (end_row,end_col) = self.end_loc;
        write!(fmt, "start: {},{} -> end: {},{} jumps: {:?}",start_row,start_col,end_row,end_col,self.jump_path)
    }
}


#[derive(Debug)]
struct PlayerInfo {
    moves: Vec<Moves>, 
    can_jump: bool,
    piece_locs:HashSet<(usize,usize)>,
    player: Player
}
impl std::fmt::Display for PlayerInfo {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> Result<(),std::fmt::Error> {
        let mut printable = String::from("");
        for (i,mv) in self.moves.iter().enumerate() {
            printable = format!("{}{}. {}\n",printable,i,mv);
        }
        write!(fmt,"{}",printable)
    }
}

#[derive(Debug)]
pub struct Board {
    board: [[BoardPiece;8];8],
    black_info: Rc<RefCell<PlayerInfo>>, 
    red_info:Rc<RefCell<PlayerInfo>>,
    current_player: Rc<RefCell<PlayerInfo>>
}

impl std::fmt::Display for Board {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> Result<(),std::fmt::Error> {
        let mut printable: String = String::from(""); 
        for row in self.board.iter() {
            let mut row_str = String::from("|"); 
            for el in row {
                let c: &str = match el {
                    BoardPiece::Red => "R" , 
                    BoardPiece::KingRed => "RK", 
                    BoardPiece::Black => "B", 
                    BoardPiece::KingBlack => "BK", 
                    BoardPiece::Empty => "_"
                };
                row_str = format!("{}{}|",row_str,c);
            }
            printable = format!("{}\n{}",row_str,printable);
        }
        write!(fmt,"{}",printable)
    }
}

impl Board {
    pub fn new() -> Self {
        let black_info_r = Rc::new(RefCell::new(PlayerInfo{
                moves: Vec::new(),
                can_jump: false,
                piece_locs: HashSet::with_capacity(12),
                player: Player::Black
            }));
        let mut obj = Self {
            board: [
                [BoardPiece::Black,BoardPiece::Empty,BoardPiece::Black,BoardPiece::Empty,BoardPiece::Black,BoardPiece::Empty,BoardPiece::Black,BoardPiece::Empty],
                [BoardPiece::Empty,BoardPiece::Black,BoardPiece::Empty,BoardPiece::Black,BoardPiece::Empty,BoardPiece::Black,BoardPiece::Empty,BoardPiece::Black],
                [BoardPiece::Black,BoardPiece::Empty,BoardPiece::Black,BoardPiece::Empty,BoardPiece::Black,BoardPiece::Empty,BoardPiece::Black,BoardPiece::Empty],
                [BoardPiece::Empty,BoardPiece::Empty,BoardPiece::Empty,BoardPiece::Empty,BoardPiece::Empty,BoardPiece::Empty,BoardPiece::Empty,BoardPiece::Empty],
                [BoardPiece::Empty,BoardPiece::Empty,BoardPiece::Empty,BoardPiece::Empty,BoardPiece::Empty,BoardPiece::Empty,BoardPiece::Empty,BoardPiece::Empty],
                [BoardPiece::Empty,BoardPiece::Red,BoardPiece::Empty,BoardPiece::Red,BoardPiece::Empty,BoardPiece::Red,BoardPiece::Empty,BoardPiece::Red],
                [BoardPiece::Red,BoardPiece::Empty,BoardPiece::Red,BoardPiece::Empty,BoardPiece::Red,BoardPiece::Empty,BoardPiece::Red,BoardPiece::Empty],
                [BoardPiece::Empty,BoardPiece::Red,BoardPiece::Empty,BoardPiece::Red,BoardPiece::Empty,BoardPiece::Red,BoardPiece::Empty,BoardPiece::Red],
            ],
            current_player: black_info_r.clone(),
            black_info: black_info_r,
            red_info: Rc::new(RefCell::new(PlayerInfo {
                moves: Vec::new(),
                can_jump: false,
                piece_locs: HashSet::with_capacity(12),
                player: Player::Red
            }))
        }; 

        for (row,row_arr) in obj.board.iter().enumerate() {
            for (col,el) in row_arr.iter().enumerate() {
              if el.is_red() { 
                   obj.red_info.borrow_mut().piece_locs.insert((row,col));
               } else if el.is_black() {
                   obj.black_info.borrow_mut().piece_locs.insert((row,col));
               }
            }
        }
        println!("black_info {:?}",obj.black_info);
        obj.calc_moves(); 
        obj
    }
    pub fn print_moves(&self) {
        println!("Player: {:?}\n{}",self.current_player.borrow().player,self.current_player.borrow());
    }

    fn calc_jumps(&self, row:usize, col: usize,p_info: &mut Vec<Moves>,player: Player ) {
        self.dfs_jumps(row,col,Moves{start_loc: (row,col),end_loc: (9,9),jump_path: HashSet::new()},p_info,player);
    }

    fn dfs_jumps(&self, row: usize, col:usize, path_par:Moves, p_info: &mut Vec<Moves>,player: Player) {

        let mut nothing_found = true; 

        let can_jump = |enemy_row: i32,enemy_col: i32,new_row: i32,new_col: i32| -> bool {
            !self.is_off_screen(enemy_row,enemy_col) && !self.is_off_screen(new_row,new_col ) &&
                !path_par.jump_path.contains(&(enemy_row as usize, enemy_col as usize)) && 
                player.get_other().does_piece_match(self.board[enemy_row as usize][enemy_col as usize]) &&
                (self.board[new_row as usize][new_col as usize] == BoardPiece::Empty || 
                 path_par.jump_path.contains(&(new_row as usize, new_col as usize))
                )
        };

        // check right ;
        if can_jump(row as i32 + player as i32, col as i32 + 1, row as i32 + 2*(player as i32), col as i32 + 2)
        {
            nothing_found = false; 
            let mut path = path_par.clone(); 
            path.jump_path.insert(((row as i32 + player as i32) as usize,col + 1));
            self.dfs_jumps((row as i32 + 2*(player as i32)) as usize,col + 2,path,p_info,player);
        }

        // check left
        if can_jump(row as i32 + player as i32, col as i32 - 1, row as i32 + 2*(player as i32), col as i32 - 2)
        {
            println!("FL");
            nothing_found = false; 
            let mut path = path_par.clone(); 
            path.jump_path.insert(((row as i32 + player as i32) as usize,col - 1));
            self.dfs_jumps((row as i32 + 2*(player as i32)) as usize,col - 2,path,p_info,player);

        }
        // /check back right
        if  self.board[path_par.start_loc.0][path_par.start_loc.1].is_king() &&
            can_jump(row as i32 - player as i32, col as i32 + 1, row as i32 - 2*(player as i32), col as i32 + 2)
        {
            nothing_found = false; 
            let mut path = path_par.clone(); 
            path.jump_path.insert(((row as i32 - player as i32) as usize,col + 1));
            self.dfs_jumps((row as i32 - 2*(player as i32)) as usize,col + 2,path,p_info,player);
        }

        // check back left 
        if  self.board[path_par.start_loc.0][path_par.start_loc.1].is_king() &&
            can_jump(row as i32 - player as i32, col as i32 - 1, row as i32 - 2*(player as i32), col as i32 - 2)
        {
            nothing_found = false; 
            let mut path = path_par.clone(); 
            path.jump_path.insert(((row as i32 - player as i32) as usize,col - 1));
            self.dfs_jumps((row as i32 - 2*(player as i32)) as usize,col - 2,path,p_info,player);

        }
        if nothing_found {
            p_info.push(path_par);
            p_info.last_mut().unwrap().end_loc = (row,col);
        }
    }

    fn calc_moves(&mut self) {

        let mut p_info = &mut *self.current_player.borrow_mut();
        p_info.moves.clear();
        p_info.can_jump = false; 

        for &(row,col) in p_info.piece_locs.iter() {
            if self.is_move_legal(row,col,Move::Jump,p_info) {
                if !p_info.can_jump {
                    p_info.moves.clear(); 
                    p_info.can_jump = true; 
                }
                self.calc_jumps(row,col,&mut p_info.moves, p_info.player);
            }
            if p_info.can_jump {
                continue; 
            }

            if self.is_move_legal(row,col,Move::ForwardRight,p_info) {
                p_info.moves.push(Moves {
                    start_loc: (row,col), 
                    end_loc: (((row as i32) + (p_info.player as i32)) as usize,col + 1),
                    jump_path: HashSet::new()
                });
            }
            if self.is_move_legal(row,col,Move::ForwardLeft,p_info){
                p_info.moves.push(Moves {
                    start_loc: (row,col), 
                    end_loc: (((row as i32) + (p_info.player as i32)) as usize,col - 1),
                    jump_path: HashSet::new()
                });
            }
            if self.is_move_legal(row,col,Move::BackwardRight,p_info){
                p_info.moves.push(Moves {
                    start_loc: (row,col), 
                    end_loc: (((row as i32) - (p_info.player as i32)) as usize,col + 1),
                    jump_path: HashSet::new()
                });
            }
            if self.is_move_legal(row,col,Move::BackwardLeft,p_info){
                p_info.moves.push(Moves {
                    start_loc: (row,col), 
                    end_loc: (((row as i32) - (p_info.player as i32)) as usize,col - 1),
                    jump_path: HashSet::new()
                });
            }
        }
    }

    pub fn do_move(&mut self, mv: usize) -> bool{ 

        let mut player_info = self.current_player.borrow_mut(); 
        let mut other_player = match player_info.player {
            Player::Red => self.black_info.borrow_mut(), 
            Player::Black => self.red_info.borrow_mut()
        };
        let move_obj = match player_info.moves.get(mv) {
            Some(m) => m, 
            None => return false 
        };

        let (start_row,start_col) = move_obj.start_loc; 
        let (end_row,end_col) = move_obj.end_loc; 

        if end_row == 7 || end_row == 0 {
            self.board[end_row][end_col] = self.board[start_row][start_col].promote();
        } else {
            self.board[end_row][end_col] = self.board[start_row][start_col];
        }
        self.board[start_row][start_col] = BoardPiece::Empty;

        for &(row,col) in move_obj.jump_path.iter() {
            self.board[row][col] = BoardPiece::Empty;
            other_player.piece_locs.remove(&(row,col));
        }

        player_info.piece_locs.remove(&(start_row,start_col));
        player_info.piece_locs.insert((end_row,end_col));

        let last_player = player_info.player; 

        drop(player_info);
        drop(other_player);

        match last_player {
            Player::Red => self.current_player = self.red_info.clone(),
            Player::Black => self.current_player = self.black_info.clone()
        };

        self.calc_moves();
        true
    }

    fn is_off_screen(&self, row: i32, col: i32) -> bool {
        row >= self.board.len() as i32 || row < 0 || col >= self.board[0].len() as i32 || col < 0 
    }

    fn is_move_legal(&self,row: usize, col: usize, mv: Move, player_info: &PlayerInfo) -> bool {
        let piece = self.board[row][col]; 
        // check its the correct player's turn for the selected piece commented out because I am
        // only doing this for the 
        // if !((piece.is_red() && self.player_turn == Player::Red ) || (piece.is_black() && self.player_turn == Player::Black)){
        //     return false; 
        // }

        // let player_info = self.current_player.borrow();
        if player_info.can_jump && mv != Move::Jump {
            return false; 
        }

        if !piece.is_king() && mv.must_be_king() {
            return false; 
        }

        let can_move_to = |new_row,new_col| {
            if self.is_off_screen(new_row,new_col){
                false
            } else {
                self.board[new_row as usize][new_col as usize] == BoardPiece::Empty
            }
        };

        let can_jump = |enemy_row: i32,enemy_col: i32, new_row: i32,new_col: i32, player: Player| -> bool {
            !(self.is_off_screen(enemy_row,enemy_col) || self.is_off_screen(new_row,new_col) || 
                self.board[enemy_row as usize][enemy_col as usize] == BoardPiece::Empty || 
                player.does_piece_match(self.board[enemy_row as usize][enemy_col as usize]) || 
                self.board[new_row as usize][new_col as usize] != BoardPiece::Empty)
        };


        match mv {
            Move::ForwardRight => {
                let new_row = row as i32 + player_info.player as i32; 
                let new_col = col as i32 + 1; 
                can_move_to(new_row,new_col)
            }, 
            Move::ForwardLeft => {
                let new_row = row as i32 + player_info.player as i32; 
                let new_col = col as i32 - 1; 
                can_move_to(new_row,new_col)
            }, 
            Move::BackwardRight => {
                let new_row = row as i32 - player_info.player as i32; 
                let new_col = col as i32 + 1; 
                can_move_to(new_row,new_col)
            },
            Move::BackwardLeft => {
                let new_row = row as i32 - player_info.player as i32; 
                let new_col = col as i32 - 1; 
                can_move_to(new_row,new_col)

            }, 
            Move::Jump => {
                let mut can_jump_acc = false; 
                if piece.is_king() {
                    can_jump_acc |= can_jump(row as i32 - player_info.player as i32, col as i32 + 1, row as i32 - 2*(player_info.player as i32), col as i32 + 2, player_info.player);
                    can_jump_acc |= can_jump(row as i32 - player_info.player as i32, col as i32 - 1, row as i32 - 2*(player_info.player as i32), col as i32 - 2, player_info.player);
                }
                can_jump_acc |= can_jump(row as i32 + player_info.player as i32, col as i32 + 1, row as i32 + 2*(player_info.player as i32), col as i32 + 2, player_info.player);
                can_jump_acc |= can_jump(row as i32 + player_info.player as i32, col as i32 - 1, row as i32 + 2*(player_info.player as i32), col as i32 - 2, player_info.player);
                can_jump_acc
            },
        }
    }
}

