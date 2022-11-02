use colored::Colorize;
use serde::{Deserialize, Serialize};
use std::cell::RefCell;
use std::collections::HashSet;
use std::rc::Rc;

pub type Cord = (usize, usize);

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum BoardPiece {
    Red,
    KingRed,
    Black,
    KingBlack,
    Empty,
}

impl TryFrom<char> for BoardPiece {
    type Error = ();
    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '1' => Ok(Self::Black),
            '2' => Ok(Self::Red),
            '3' => Ok(Self::KingBlack),
            '4' => Ok(Self::KingRed),
            '0' => Ok(Self::Empty),
            _ => Err(()),
        }
    }
}

impl BoardPiece {
    pub fn is_red(&self) -> bool {
        const LAST_RED: u32 = 1;
        *self as u32 <= LAST_RED
    }
    pub fn is_black(&self) -> bool {
        const LAST_RED: u32 = 1;
        const LAST_BLACK: u32 = 3;
        *self as u32 <= LAST_BLACK && *self as u32 > LAST_RED
    }

    pub fn is_king(&self) -> bool {
        *self as u32 % 2 != 0 && *self != BoardPiece::Empty
    }

    fn promote(&self) -> Self {
        if self.is_red() {
            Self::KingRed
        } else if self.is_black() {
            Self::KingBlack
        } else {
            Self::Empty
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Player {
    Black = 1,
    Red = -1,
}

impl Player {
    pub fn get_other(&self) -> Self {
        match self {
            Self::Black => Self::Red,
            Self::Red => Self::Black,
        }
    }

    fn does_piece_match(&self, piece: BoardPiece) -> bool {
        match *self {
            Self::Black => piece.is_black(),
            Self::Red => piece.is_red(),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Moves {
    jump_path: HashSet<Cord>,
    start_loc: Cord,
    end_loc: Cord,
}
impl std::fmt::Display for Moves {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        let (start_row, start_col) = self.start_loc;
        let (end_row, end_col) = self.end_loc;
        write!(
            fmt,
            "start: {},{} -> end: {},{} jumps: {:?}",
            start_row, start_col, end_row, end_col, self.jump_path
        )
    }
}

impl Moves {
    pub fn new_empty() -> Self {
        Moves {
            start_loc: (9, 9),
            end_loc: (9, 9),
            jump_path: HashSet::new(),
        }
    }

    pub fn is_jump(&self) -> bool {
        !self.jump_path.is_empty()
    }
}

#[derive(Debug, Clone)]
pub struct PlayerInfo {
    moves: Vec<Moves>,
    can_jump: bool,
    piece_locs: HashSet<Cord>,
    player: Player,
}

impl std::fmt::Display for PlayerInfo {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        let printable = self
            .moves
            .iter()
            .enumerate()
            .fold(String::from(""), |acc, (i, mv)| {
                format!("{}{}. {}\n", acc, i, mv)
            });
        write!(fmt, "{}", printable)
    }
}

impl PlayerInfo {
    pub fn get_moves(&self) -> &Vec<Moves> {
        &self.moves
    }

    pub fn get_can_jump(&self) -> bool {
        return self.can_jump;
    }
}

#[derive(Debug)]
pub struct Board {
    board: [[BoardPiece; 8]; 8],
    black_info: Rc<RefCell<PlayerInfo>>,
    red_info: Rc<RefCell<PlayerInfo>>,
    current_player: Rc<RefCell<PlayerInfo>>,
}

impl Clone for Board {
    fn clone(&self) -> Self {
        let red_info = Rc::new((*(self.red_info)).clone());
        let black_info = Rc::new((*(self.black_info)).clone());
        let current_player = match self.current_player.borrow().player {
            Player::Red => red_info.clone(),
            Player::Black => black_info.clone(),
        };
        Self {
            board: self.board.clone(),
            red_info,
            black_info,
            current_player,
        }
    }
}

impl std::fmt::Display for Board {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(
            fmt,
            "    0  1  2  3  4  5  6  7\n{}",
            self.board
                .iter()
                .enumerate()
                .fold(String::from(""), |acc, (row_count, row)| {
                    let (ends, mid) = row.iter().enumerate().fold(
                        (String::from(""), String::from("")),
                        |(ends, mid), (col_count, bp)| {
                            let space = match col_count % 2 == row_count % 2 {
                                true => " ".on_green(),
                                false => " ".on_magenta(),
                            };
                            let square = match bp {
                                BoardPiece::Red => "#".white().on_red(),
                                BoardPiece::KingRed => "K".white().on_red(),
                                BoardPiece::Black => "#".white().on_black(),
                                BoardPiece::KingBlack => "K".white().on_black(),
                                BoardPiece::Empty => space.clone(),
                            };
                            (
                                format!("{}{}{}{}", ends, space, space, space),
                                format!("{}{}{}{}", mid, space, square, space),
                            )
                        },
                    );
                    format!("   {}\n{}. {}\n   {}\n{}", ends, row_count, mid, ends, acc)
                })
        )
    }
}

impl Board {
    pub fn new(file_input: &Option<String>) -> Self {
        let board = match file_input {
            Some(s) => {
                let mut board = [[BoardPiece::Empty; 8]; 8];
                for (i, row) in s.split('\n').enumerate() {
                    if i > 7 {
                        break;
                    }
                    let mut col_i = ((i % 2) == 0) as usize;
                    for c in row.chars().into_iter() {
                        if c == ' ' {
                            continue;
                        }
                        if col_i > 7 {
                            println!(
                                "File Format Error too many pieces on a row, col_i: {:?}  c:{:?}",
                                col_i, c
                            );
                            return Self::new(&None);
                        }
                        match c.try_into() {
                            Ok(bp) => {
                                board[7 - i][col_i] = bp;
                                col_i += 2;
                            }
                            Err(_) => {
                                println!("Error Reading input {:}, making default Board!", c);
                                return Self::new(&None);
                            }
                        }
                    }
                }
                board
            }
            None => [
                [
                    BoardPiece::Black,
                    BoardPiece::Empty,
                    BoardPiece::Black,
                    BoardPiece::Empty,
                    BoardPiece::Black,
                    BoardPiece::Empty,
                    BoardPiece::Black,
                    BoardPiece::Empty,
                ],
                [
                    BoardPiece::Empty,
                    BoardPiece::Black,
                    BoardPiece::Empty,
                    BoardPiece::Black,
                    BoardPiece::Empty,
                    BoardPiece::Black,
                    BoardPiece::Empty,
                    BoardPiece::Black,
                ],
                [
                    BoardPiece::Black,
                    BoardPiece::Empty,
                    BoardPiece::Black,
                    BoardPiece::Empty,
                    BoardPiece::Black,
                    BoardPiece::Empty,
                    BoardPiece::Black,
                    BoardPiece::Empty,
                ],
                [
                    BoardPiece::Empty,
                    BoardPiece::Empty,
                    BoardPiece::Empty,
                    BoardPiece::Empty,
                    BoardPiece::Empty,
                    BoardPiece::Empty,
                    BoardPiece::Empty,
                    BoardPiece::Empty,
                ],
                [
                    BoardPiece::Empty,
                    BoardPiece::Empty,
                    BoardPiece::Empty,
                    BoardPiece::Empty,
                    BoardPiece::Empty,
                    BoardPiece::Empty,
                    BoardPiece::Empty,
                    BoardPiece::Empty,
                ],
                [
                    BoardPiece::Empty,
                    BoardPiece::Red,
                    BoardPiece::Empty,
                    BoardPiece::Red,
                    BoardPiece::Empty,
                    BoardPiece::Red,
                    BoardPiece::Empty,
                    BoardPiece::Red,
                ],
                [
                    BoardPiece::Red,
                    BoardPiece::Empty,
                    BoardPiece::Red,
                    BoardPiece::Empty,
                    BoardPiece::Red,
                    BoardPiece::Empty,
                    BoardPiece::Red,
                    BoardPiece::Empty,
                ],
                [
                    BoardPiece::Empty,
                    BoardPiece::Red,
                    BoardPiece::Empty,
                    BoardPiece::Red,
                    BoardPiece::Empty,
                    BoardPiece::Red,
                    BoardPiece::Empty,
                    BoardPiece::Red,
                ],
            ],
        };

        let black_info_r = Rc::new(RefCell::new(PlayerInfo {
            moves: Vec::new(),
            can_jump: false,
            piece_locs: HashSet::with_capacity(12),
            player: Player::Black,
        }));
        let red_info_r = Rc::new(RefCell::new(PlayerInfo {
            moves: Vec::new(),
            can_jump: false,
            piece_locs: HashSet::with_capacity(12),
            player: Player::Red,
        }));
        let mut obj = Self {
            board,
            current_player: match file_input {
                None => black_info_r.clone(),
                Some(s) => match s.lines().nth(8) {
                    Some(ns) => {
                        match ns.parse::<u32>() {
                            Err(_) => {
                                println!("Invalid File input: Player is not a number, defaulting to Black");
                                black_info_r.clone()
                            }
                            Ok(n) => match n {
                                1 => black_info_r.clone(),
                                2 => red_info_r.clone(),
                                _ => {
                                    println!("Invalid File input: Player # must be 1 or 0, defaulting to Black");
                                    black_info_r.clone()
                                }
                            },
                        }
                    }
                    None => {
                        println!("Invalid File input: No player to choose, defaulting to Black");
                        black_info_r.clone()
                    }
                },
            },
            black_info: black_info_r,
            red_info: red_info_r,
        };

        for (row, row_arr) in obj.board.iter().enumerate() {
            for (col, el) in row_arr.iter().enumerate() {
                if el.is_red() {
                    obj.red_info.borrow_mut().piece_locs.insert((row, col));
                } else if el.is_black() {
                    obj.black_info.borrow_mut().piece_locs.insert((row, col));
                }
            }
        }
        obj.calc_moves();
        obj
    }

    pub fn swap_current_player(&mut self) {
        let player = self.current_player.borrow().player;
        match player {
            Player::Red => self.current_player = self.black_info.clone(),
            Player::Black => self.current_player = self.red_info.clone(),
        };
        self.calc_moves();
    }

    pub fn get_pieces(&self) -> (Vec<(BoardPiece, Cord)>, Vec<(BoardPiece, Cord)>) {
        let cp = self.current_player.borrow();
        let op = match cp.player {
            Player::Red => self.black_info.borrow(),
            Player::Black => self.red_info.borrow(),
        };

        // let mut mine: Vec<(BoardPiece, Cord)> = Vec::new();
        // let mut other: Vec<(BoardPiece, Cord)> = Vec::new();
        //
        // for &(row, col) in cp.piece_locs.iter() {
        //     mine.push((self.board[row][col], (row, col)));
        // }
        //
        // for &(row, col) in op.piece_locs.iter() {
        //     other.push((self.board[row][col], (row, col)));
        // }

        let mine = cp
            .piece_locs
            .iter()
            .map(|&(row, col)| (self.board[row][col], (row, col)))
            .collect();

        let other = op
            .piece_locs
            .iter()
            .map(|&(row, col)| (self.board[row][col], (row, col)))
            .collect();

        return (mine, other);
    }

    pub fn get_player_info(&self) -> Rc<RefCell<PlayerInfo>> {
        self.current_player.clone()
    }

    pub fn print_moves(&self) {
        println!(
            "Player: {:?}\n{}",
            self.current_player.borrow().player,
            self.current_player.borrow()
        );
    }

    fn calc_jumps(&self, row: usize, col: usize, p_info: &mut Vec<Moves>, player: Player) {
        self.dfs_jumps(
            row,
            col,
            Moves {
                start_loc: (row, col),
                end_loc: (9, 9),
                jump_path: HashSet::new(),
            },
            p_info,
            player,
        );
    }

    fn dfs_jumps(
        &self,
        row: usize,
        col: usize,
        path_par: Moves,
        p_info: &mut Vec<Moves>,
        player: Player,
    ) {
        let mut nothing_found = true;

        let can_jump = |enemy_row: i32, enemy_col: i32, new_row: i32, new_col: i32| -> bool {
            !self.is_off_screen(enemy_row, enemy_col)
                && !self.is_off_screen(new_row, new_col)
                && !path_par
                    .jump_path
                    .contains(&(enemy_row as usize, enemy_col as usize))
                && player
                    .get_other()
                    .does_piece_match(self.board[enemy_row as usize][enemy_col as usize])
                && (self.board[new_row as usize][new_col as usize] == BoardPiece::Empty
                    || path_par
                        .jump_path
                        .contains(&(new_row as usize, new_col as usize))
                    || (path_par.start_loc.0 as i32 == new_row
                        && path_par.start_loc.1 as i32 == new_col))
        };

        // check right ;
        if can_jump(
            row as i32 + player as i32,
            col as i32 + 1,
            row as i32 + 2 * (player as i32),
            col as i32 + 2,
        ) {
            nothing_found = false;
            let mut path = path_par.clone();
            path.jump_path
                .insert(((row as i32 + player as i32) as usize, col + 1));
            self.dfs_jumps(
                (row as i32 + 2 * (player as i32)) as usize,
                col + 2,
                path,
                p_info,
                player,
            );
        }

        // check left
        if can_jump(
            row as i32 + player as i32,
            col as i32 - 1,
            row as i32 + 2 * (player as i32),
            col as i32 - 2,
        ) {
            nothing_found = false;
            let mut path = path_par.clone();
            path.jump_path
                .insert(((row as i32 + player as i32) as usize, col - 1));
            self.dfs_jumps(
                (row as i32 + 2 * (player as i32)) as usize,
                col - 2,
                path,
                p_info,
                player,
            );
        }
        // /check back right
        if self.board[path_par.start_loc.0][path_par.start_loc.1].is_king()
            && can_jump(
                row as i32 - player as i32,
                col as i32 + 1,
                row as i32 - 2 * (player as i32),
                col as i32 + 2,
            )
        {
            nothing_found = false;
            let mut path = path_par.clone();
            path.jump_path
                .insert(((row as i32 - player as i32) as usize, col + 1));
            self.dfs_jumps(
                (row as i32 - 2 * (player as i32)) as usize,
                col + 2,
                path,
                p_info,
                player,
            );
        }

        // check back left
        if self.board[path_par.start_loc.0][path_par.start_loc.1].is_king()
            && can_jump(
                row as i32 - player as i32,
                col as i32 - 1,
                row as i32 - 2 * (player as i32),
                col as i32 - 2,
            )
        {
            nothing_found = false;
            let mut path = path_par.clone();
            path.jump_path
                .insert(((row as i32 - player as i32) as usize, col - 1));
            self.dfs_jumps(
                (row as i32 - 2 * (player as i32)) as usize,
                col - 2,
                path,
                p_info,
                player,
            );
        }
        if nothing_found {
            p_info.push(path_par);
            p_info.last_mut().unwrap().end_loc = (row, col);
        }
    }

    fn calc_moves(&mut self) {
        let mut p_info = &mut *self.current_player.borrow_mut();
        p_info.moves.clear();
        p_info.can_jump = false;

        for &(row, col) in p_info.piece_locs.iter() {
            if self.is_move_legal(row, col, Move::Jump, p_info) {
                if !p_info.can_jump {
                    p_info.moves.clear();
                    p_info.can_jump = true;
                }
                self.calc_jumps(row, col, &mut p_info.moves, p_info.player);
            }
            if p_info.can_jump {
                continue;
            }

            if self.is_move_legal(row, col, Move::ForwardRight, p_info) {
                p_info.moves.push(Moves {
                    start_loc: (row, col),
                    end_loc: (((row as i32) + (p_info.player as i32)) as usize, col + 1),
                    jump_path: HashSet::new(),
                });
            }
            if self.is_move_legal(row, col, Move::ForwardLeft, p_info) {
                p_info.moves.push(Moves {
                    start_loc: (row, col),
                    end_loc: (((row as i32) + (p_info.player as i32)) as usize, col - 1),
                    jump_path: HashSet::new(),
                });
            }
            if self.is_move_legal(row, col, Move::BackwardRight, p_info) {
                p_info.moves.push(Moves {
                    start_loc: (row, col),
                    end_loc: (((row as i32) - (p_info.player as i32)) as usize, col + 1),
                    jump_path: HashSet::new(),
                });
            }
            if self.is_move_legal(row, col, Move::BackwardLeft, p_info) {
                p_info.moves.push(Moves {
                    start_loc: (row, col),
                    end_loc: (((row as i32) - (p_info.player as i32)) as usize, col - 1),
                    jump_path: HashSet::new(),
                });
            }
        }
    }

    pub fn is_game_over(&self) -> Option<Player> {
        let p_info = self.current_player.borrow();
        if p_info.moves.is_empty() {
            return Option::Some(p_info.player.get_other());
        }
        return Option::None;
    }

    pub fn do_move(&mut self, mv: usize) -> bool {
        let mut player_info = self.current_player.borrow_mut();
        let mut other_player = match player_info.player {
            Player::Red => self.black_info.borrow_mut(),
            Player::Black => self.red_info.borrow_mut(),
        };

        let move_obj = match player_info.moves.get(mv) {
            Some(m) => m,
            None => return false,
        };

        let (start_row, start_col) = move_obj.start_loc;
        let (end_row, end_col) = move_obj.end_loc;

        if end_row == 7 || end_row == 0 {
            self.board[end_row][end_col] = self.board[start_row][start_col].promote();
        } else {
            self.board[end_row][end_col] = self.board[start_row][start_col];
        }
        self.board[start_row][start_col] = BoardPiece::Empty;

        for &(row, col) in move_obj.jump_path.iter() {
            self.board[row][col] = BoardPiece::Empty;
            other_player.piece_locs.remove(&(row, col));
        }

        player_info.piece_locs.remove(&(start_row, start_col));
        player_info.piece_locs.insert((end_row, end_col));

        let last_player = player_info.player;
        // let next_player = other_player.player;

        drop(player_info);
        drop(other_player);

        match last_player {
            Player::Red => self.current_player = self.black_info.clone(),
            Player::Black => self.current_player = self.red_info.clone(),
        };

        self.calc_moves();
        true
    }

    fn is_off_screen(&self, row: i32, col: i32) -> bool {
        row >= self.board.len() as i32 || row < 0 || col >= self.board[0].len() as i32 || col < 0
    }

    fn is_move_legal(&self, row: usize, col: usize, mv: Move, player_info: &PlayerInfo) -> bool {
        let piece = self.board[row][col];
        // check its the correct player's turn for the selected piece commented out because I am
        // only doing this off of the list of current players pieces
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

        let can_move_to = |new_row, new_col| {
            if self.is_off_screen(new_row, new_col) {
                false
            } else {
                self.board[new_row as usize][new_col as usize] == BoardPiece::Empty
            }
        };

        let can_jump =
            |enemy_row: i32, enemy_col: i32, new_row: i32, new_col: i32, player: Player| -> bool {
                !(self.is_off_screen(enemy_row, enemy_col)
                    || self.is_off_screen(new_row, new_col)
                    || self.board[enemy_row as usize][enemy_col as usize] == BoardPiece::Empty
                    || player.does_piece_match(self.board[enemy_row as usize][enemy_col as usize])
                    || self.board[new_row as usize][new_col as usize] != BoardPiece::Empty)
            };

        match mv {
            Move::ForwardRight => {
                let new_row = row as i32 + player_info.player as i32;
                let new_col = col as i32 + 1;
                can_move_to(new_row, new_col)
            }
            Move::ForwardLeft => {
                let new_row = row as i32 + player_info.player as i32;
                let new_col = col as i32 - 1;
                can_move_to(new_row, new_col)
            }
            Move::BackwardRight => {
                let new_row = row as i32 - player_info.player as i32;
                let new_col = col as i32 + 1;
                can_move_to(new_row, new_col)
            }
            Move::BackwardLeft => {
                let new_row = row as i32 - player_info.player as i32;
                let new_col = col as i32 - 1;
                can_move_to(new_row, new_col)
            }
            Move::Jump => {
                let mut can_jump_acc = false;
                if piece.is_king() {
                    can_jump_acc |= can_jump(
                        row as i32 - player_info.player as i32,
                        col as i32 + 1,
                        row as i32 - 2 * (player_info.player as i32),
                        col as i32 + 2,
                        player_info.player,
                    );
                    can_jump_acc |= can_jump(
                        row as i32 - player_info.player as i32,
                        col as i32 - 1,
                        row as i32 - 2 * (player_info.player as i32),
                        col as i32 - 2,
                        player_info.player,
                    );
                }
                can_jump_acc |= can_jump(
                    row as i32 + player_info.player as i32,
                    col as i32 + 1,
                    row as i32 + 2 * (player_info.player as i32),
                    col as i32 + 2,
                    player_info.player,
                );
                can_jump_acc |= can_jump(
                    row as i32 + player_info.player as i32,
                    col as i32 - 1,
                    row as i32 + 2 * (player_info.player as i32),
                    col as i32 - 2,
                    player_info.player,
                );
                can_jump_acc
            }
        }
    }

    pub fn get_current_player(&self) -> Player {
        self.current_player.borrow().player
    }
}
