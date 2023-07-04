use anyhow::{anyhow, bail, Context, Result};
use colored::{ColoredString, Colorize};
use serde::{Deserialize, Serialize};
use std::{collections::HashSet, str::FromStr};

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
    type Error = anyhow::Error;
    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '1' => Ok(Self::Black),
            '2' => Ok(Self::Red),
            '3' => Ok(Self::KingBlack),
            '4' => Ok(Self::KingRed),
            '0' => Ok(Self::Empty),
            _ => Err(anyhow!("Invalid piece value {}", value)),
        }
    }
}

impl From<BoardPiece> for char {
    fn from(value: BoardPiece) -> Self {
        match value {
            BoardPiece::Black => '1',
            BoardPiece::Red => '2',
            BoardPiece::KingBlack => '3',
            BoardPiece::KingRed => '4',
            BoardPiece::Empty => '0',
        }
    }
}
impl TryFrom<&BoardPiece> for ColoredString {
    type Error = anyhow::Error;
    fn try_from(value: &BoardPiece) -> Result<Self, Self::Error> {
        Ok(match value {
            BoardPiece::Red => "#".white().on_red(),
            BoardPiece::KingRed => "K".white().on_red(),
            BoardPiece::Black => "#".white().on_black(),
            BoardPiece::KingBlack => "K".white().on_black(),
            BoardPiece::Empty => {
                bail!("is space");
            }
        })
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

impl From<Player> for u8 {
    fn from(value: Player) -> Self {
        match value {
            Player::Black => 1,
            Player::Red => 2,
        }
    }
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

#[derive(Debug, Clone)]
pub struct Players {
    players: [PlayerInfo; 2],
    black: usize,
    red: usize,
}
impl Default for Players {
    fn default() -> Self {
        Self {
            players: [
                PlayerInfo {
                    moves: Vec::new(),
                    can_jump: false,
                    piece_locs: HashSet::with_capacity(12),
                    player: Player::Black,
                },
                PlayerInfo {
                    moves: Vec::new(),
                    can_jump: false,
                    piece_locs: HashSet::with_capacity(12),
                    player: Player::Red,
                },
            ],
            black: 0,
            red: 1,
        }
    }
}
impl Players {
    fn swap(&mut self) {
        self.players.rotate_left(1);
        self.black = 1 - self.black;
        self.red = 1 - self.red;
    }

    fn get_current_players_mut(&mut self) -> (&mut PlayerInfo, &mut PlayerInfo) {
        let (a, b) = self.players.split_at_mut(1);
        (&mut a[0], &mut b[0])
    }
    fn get_current_players(&self) -> (&PlayerInfo, &PlayerInfo) {
        (&self.players[0], &self.players[1])
    }

    fn get_current_player(&self) -> &PlayerInfo {
        &self.players[0]
    }
    fn get_current_player_mut(&mut self) -> &mut PlayerInfo {
        &mut self.players[0]
    }
    #[allow(dead_code)]
    fn get_other_player(&self) -> &PlayerInfo {
        &self.players[1]
    }
    #[allow(dead_code)]
    fn get_other_player_mut(&mut self) -> &mut PlayerInfo {
        &mut self.players[1]
    }

    #[allow(dead_code)]
    fn get_red(&self) -> &PlayerInfo {
        &self.players[self.red]
    }
    #[allow(dead_code)]
    fn get_red_mut(&mut self) -> &mut PlayerInfo {
        &mut self.players[self.red]
    }

    #[allow(dead_code)]
    fn get_black(&self) -> &PlayerInfo {
        &self.players[self.black]
    }
    #[allow(dead_code)]
    fn get_black_mut(&mut self) -> &mut PlayerInfo {
        &mut self.players[self.black]
    }
}

const BOARD_SIZE: usize = 8;
#[derive(Debug, Clone)]
pub struct Board {
    board: [[BoardPiece; BOARD_SIZE]; BOARD_SIZE],
    players: Players, // black_info: Rc<RefCell<PlayerInfo>>,
                      // red_info: Rc<RefCell<PlayerInfo>>,
                      // current_player: Rc<RefCell<PlayerInfo>>,
}

pub struct OutputFileBoard<'a>(&'a Board);

impl<'a> std::fmt::Display for OutputFileBoard<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self.0.board.iter().rev() {
            for col in row.iter() {
                let char: char = (*col).into();
                write!(f, "{} ", char)?;
            }
            write!(f, "\n")?;
        }
        let player: u8 = self.0.players.get_current_player().player.into();
        write!(f, "{}", player)
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
                            let pre_square = bp.try_into();
                            let square = pre_square.as_ref().unwrap_or(&space);

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

impl FromStr for Board {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (board, players) = s.split('\n').enumerate().try_fold(
            ([[BoardPiece::Empty; 8]; 8], Players::default()),
            |(mut board, mut players), (i, row)| {
                if i > 8 {
                    return Ok((board, players));
                }
                if i > 7 {
                    match row
                        .parse::<u32>()
                        .context("Invalid File input: Player is not a number")?
                    {
                        1 => {}
                        2 => players.swap(),
                        _ => {
                            bail!(
                                "Invalid File input: Player # must be 1 or 0, defaulting to Black"
                            );
                        }
                    };
                    return Ok((board, players));
                }
                let mut col_i = ((i % 2) == 0) as usize;
                for c in row.chars().into_iter() {
                    if c == ' ' || c == '0' {
                        continue;
                    }
                    if col_i > 7 {
                        bail!(
                            "File Format Error too many pieces on a row, col_i: {:?}  c:{:?}",
                            col_i,
                            c
                        );
                    }
                    board[7 - i][col_i] = c.try_into()?;
                    col_i += 2;
                }
                Ok((board, players))
            },
        )?;

        Ok(Board::new(board, players))
    }
}

impl Default for Board {
    fn default() -> Self {
        let board = [
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
        ];
        Self::new(board, Players::default())
    }
}

impl Board {
    pub fn display_file<'a>(&'a self) -> OutputFileBoard<'a> {
        OutputFileBoard(self)
    }
    fn new(board: [[BoardPiece; 8]; 8], players: Players) -> Self {
        let mut obj = Self { board, players };
        for (row, row_arr) in obj.board.iter().enumerate() {
            for (col, el) in row_arr.iter().enumerate() {
                if el.is_red() {
                    obj.players.get_red_mut().piece_locs.insert((row, col));
                } else if el.is_black() {
                    obj.players.get_black_mut().piece_locs.insert((row, col));
                }
            }
        }

        obj.calc_moves();
        obj
    }

    pub fn swap_current_player(&mut self) {
        self.players.swap();
        self.calc_moves();
    }

    pub fn get_pieces(&self) -> (Vec<(BoardPiece, Cord)>, Vec<(BoardPiece, Cord)>) {
        let (cp, op) = self.players.get_current_players();

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

    pub fn get_player_info(&self) -> &PlayerInfo {
        &self.players.get_current_player()
    }

    pub fn print_moves(&self) {
        let player = self.players.get_current_player();
        println!("Player: {:?}\n{}", player.player, player);
    }

    fn calc_jumps(&mut self, row: usize, col: usize) {
        self.dfs_jumps(
            row,
            col,
            Moves {
                start_loc: (row, col),
                end_loc: (9, 9),
                jump_path: HashSet::new(),
            },
        );
    }

    fn can_jump(
        &self,
        enemy_row: i32,
        enemy_col: i32,
        new_row: i32,
        new_col: i32,
        path_par: &Moves,
        player: Player,
    ) -> bool {
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
    }

    fn dfs_jumps(&mut self, row: usize, col: usize, path_par: Moves) {
        let player = self.players.get_current_player().player;
        // let p_info = &mut self.players.get_current_player_mut().moves;
        let mut nothing_found = true;

        // check right ;
        if self.can_jump(
            row as i32 + player as i32,
            col as i32 + 1,
            row as i32 + 2 * (player as i32),
            col as i32 + 2,
            &path_par,
            player,
        ) {
            nothing_found = false;
            let mut path = path_par.clone();
            path.jump_path
                .insert(((row as i32 + player as i32) as usize, col + 1));
            self.dfs_jumps((row as i32 + 2 * (player as i32)) as usize, col + 2, path);
        }

        // check left
        if self.can_jump(
            row as i32 + player as i32,
            col as i32 - 1,
            row as i32 + 2 * (player as i32),
            col as i32 - 2,
            &path_par,
            player,
        ) {
            nothing_found = false;
            let mut path = path_par.clone();
            path.jump_path
                .insert(((row as i32 + player as i32) as usize, col - 1));
            self.dfs_jumps((row as i32 + 2 * (player as i32)) as usize, col - 2, path);
        }
        // /check back right
        if self.board[path_par.start_loc.0][path_par.start_loc.1].is_king()
            && self.can_jump(
                row as i32 - player as i32,
                col as i32 + 1,
                row as i32 - 2 * (player as i32),
                col as i32 + 2,
                &path_par,
                player,
            )
        {
            nothing_found = false;
            let mut path = path_par.clone();
            path.jump_path
                .insert(((row as i32 - player as i32) as usize, col + 1));
            self.dfs_jumps((row as i32 - 2 * (player as i32)) as usize, col + 2, path);
        }

        // check back left
        if self.board[path_par.start_loc.0][path_par.start_loc.1].is_king()
            && self.can_jump(
                row as i32 - player as i32,
                col as i32 - 1,
                row as i32 - 2 * (player as i32),
                col as i32 - 2,
                &path_par,
                player,
            )
        {
            nothing_found = false;
            let mut path = path_par.clone();
            path.jump_path
                .insert(((row as i32 - player as i32) as usize, col - 1));
            self.dfs_jumps((row as i32 - 2 * (player as i32)) as usize, col - 2, path);
        }
        if nothing_found {
            let p_info = &mut self.players.get_current_player_mut().moves;
            p_info.push(path_par);
            p_info.last_mut().unwrap().end_loc = (row, col);
        }
    }

    fn calc_moves(&mut self) {
        // let mut p_info = &mut *self.current_player.borrow_mut();

        let rows = self
            .players
            .get_current_player()
            .piece_locs
            .iter()
            .cloned()
            .collect::<Box<_>>();

        {
            let p_info = self.players.get_current_player_mut();
            p_info.moves.clear();
            p_info.can_jump = false;
        }

        for &(row, col) in rows.into_iter() {
            if self.is_move_legal(row, col, Move::Jump) {
                if !self.players.get_current_player().can_jump {
                    self.players.get_current_player_mut().moves.clear();
                    self.players.get_current_player_mut().can_jump = true;
                }
                self.calc_jumps(row, col);
            }
            if self.players.get_current_player().can_jump {
                continue;
            }

            if self.is_move_legal(row, col, Move::ForwardRight) {
                let p_info = self.players.get_current_player_mut();
                p_info.moves.push(Moves {
                    start_loc: (row, col),
                    end_loc: (((row as i32) + (p_info.player as i32)) as usize, col + 1),
                    jump_path: HashSet::new(),
                });
            }
            if self.is_move_legal(row, col, Move::ForwardLeft) {
                let p_info = self.players.get_current_player_mut();
                p_info.moves.push(Moves {
                    start_loc: (row, col),
                    end_loc: (((row as i32) + (p_info.player as i32)) as usize, col - 1),
                    jump_path: HashSet::new(),
                });
            }
            if self.is_move_legal(row, col, Move::BackwardRight) {
                let p_info = self.players.get_current_player_mut();
                p_info.moves.push(Moves {
                    start_loc: (row, col),
                    end_loc: (((row as i32) - (p_info.player as i32)) as usize, col + 1),
                    jump_path: HashSet::new(),
                });
            }
            if self.is_move_legal(row, col, Move::BackwardLeft) {
                let p_info = self.players.get_current_player_mut();
                p_info.moves.push(Moves {
                    start_loc: (row, col),
                    end_loc: (((row as i32) - (p_info.player as i32)) as usize, col - 1),
                    jump_path: HashSet::new(),
                });
            }
        }
    }

    pub fn is_game_over(&self) -> Option<Player> {
        let p_info = self.players.get_current_player();
        if p_info.moves.is_empty() {
            return Option::Some(p_info.player.get_other());
        }
        return Option::None;
    }

    pub fn do_move(&mut self, mv: usize) -> bool {
        // let mut player_info = self.current_player.borrow_mut();
        // let mut other_player = match player_info.player {
        //     Player::Red => self.black_info.borrow_mut(),
        //     Player::Black => self.red_info.borrow_mut(),
        // };

        let (player_info, other_player) = self.players.get_current_players_mut();
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

        // let last_player = player_info.player;
        // let next_player = other_player.player;

        // drop(player_info);
        // drop(other_player);

        self.players.swap();

        // match last_player {
        //     Player::Red => self.current_player = self.black_info.clone(),
        //     Player::Black => self.current_player = self.red_info.clone(),
        // };

        self.calc_moves();
        true
    }

    fn is_off_screen(&self, row: i32, col: i32) -> bool {
        row >= self.board.len() as i32 || row < 0 || col >= self.board[0].len() as i32 || col < 0
    }

    fn is_move_legal(&self, row: usize, col: usize, mv: Move) -> bool {
        let player_info = self.players.get_current_player();
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
        self.players.get_current_player().player
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_display() -> anyhow::Result<()> {
        let board1 = Board::default();
        println!("{}", board1.display_file());
        let board2: Board = format!("{}", board1.display_file()).parse()?;

        println!("{}", board1);
        println!("{}", board2);

        for (row1, row2) in board1.board.iter().zip(board2.board.iter()) {
            for (col1, col2) in row1.iter().zip(row2.iter()) {
                assert_eq!(*col1, *col2)
            }
        }
        Ok(())
    }
}
