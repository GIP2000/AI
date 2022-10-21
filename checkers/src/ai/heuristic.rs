use crate::board::{Board, BoardPiece, Cord, Player};

type PieceType = (BoardPiece, Cord);

pub struct Heuristic {
    // Normal Piece Value
    n_piece_val: i32,
    // King Piece Value
    k_piece_val: i32,
    // Distance from Home Row Multiplier
    d_hr_mul: i32,
    // Bonus for piece in true center
    true_center: i32,
    // Bonus for piece in off center
    off_center: i32,
    // Bonus for piece in Defending goal center
    goalies_center: i32,
    // Bonus for piece in Defending goal side
    goalies_side: i32,
    // Mobility bonus move multiplier
    per_move_val: i32,
    // Mobility bonus move multiplier for Jumps
    per_jump_move_val: i32,
}

impl Heuristic {
    #[allow(dead_code)]
    pub fn new(
        n_piece_val: i32,
        k_piece_val: i32,
        d_hr_mul: i32,
        true_center: i32,
        off_center: i32,
        goalies_center: i32,
        goalies_side: i32,
        per_move_val: i32,
        per_jump_move_val: i32,
    ) -> Self {
        Self {
            n_piece_val,
            k_piece_val,
            d_hr_mul,
            true_center,
            off_center,
            goalies_center,
            goalies_side,
            per_move_val,
            per_jump_move_val,
        }
    }

    pub fn default_new() -> Self {
        Self {
            n_piece_val: 100,
            k_piece_val: 150,
            d_hr_mul: 5,
            true_center: 50,
            off_center: 25,
            goalies_center: 25,
            goalies_side: 25,
            per_move_val: 5,
            per_jump_move_val: 8,
        }
    }

    pub fn h(&self, state: &Board, is_max: bool) -> i32 {
        let (my_pieces, other_pieces) = state.get_pieces();
        let mut score = 0;

        // let is_end_game = my_pieces.len() + other_pieces.len() < 6;

        let per_piece = |pt: &PieceType, plyr: Player| {
            let (bp, bc) = pt;
            let mut current_score = self.piece_type_value(bp);
            if !bp.is_king() {
                current_score += self.depth_distance(
                    bc,
                    match plyr {
                        Player::Red => 0,
                        Player::Black => 7,
                    },
                );
            }
            current_score += self.in_center(bc);
            current_score += self.in_goal(
                bc,
                match plyr {
                    Player::Red => 7,
                    Player::Black => 0,
                },
            );

            return current_score;
        };

        let fold_func = |plyr: Player| {
            return move |prev: i32, pt: &PieceType| {
                return prev + per_piece(pt, plyr);
            };
        };

        score += my_pieces
            .iter()
            .fold(0, fold_func(state.get_current_player()));
        score -= other_pieces
            .iter()
            .fold(0, fold_func(state.get_current_player().get_other()));
        score += self.mobility(state);

        if !is_max {
            score = -score;
        }
        return score;
    }
    fn mobility(&self, state: &Board) -> i32 {
        match state.get_player_info().borrow().get_can_jump() {
            true => {
                state.get_player_info().borrow().get_moves().len() as i32 * self.per_jump_move_val
            }
            false => state.get_player_info().borrow().get_moves().len() as i32 * self.per_move_val,
        }
    }

    fn in_goal(&self, cords: &Cord, home_row: usize) -> i32 {
        let &(row, col) = cords;

        if row == home_row {
            if col > 1 && col < 6 {
                return self.goalies_center;
            }
            return self.goalies_side;
        }

        return 0;
    }

    fn in_center(&self, cords: &Cord) -> i32 {
        let &(row, col) = cords;
        if row == 3 || row == 4 {
            if col == 3 || col == 4 {
                return self.true_center;
            }
            return self.off_center;
        }
        return 0;
    }

    fn depth_distance(&self, cords: &Cord, goal: i32) -> i32 {
        return (cords.1 as i32 - goal).abs() * self.d_hr_mul;
    }

    fn piece_type_value(&self, piece: &BoardPiece) -> i32 {
        match piece.is_king() {
            true => self.k_piece_val,
            false => self.n_piece_val,
        }
    }
}
