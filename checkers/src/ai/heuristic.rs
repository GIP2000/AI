use crate::board::{Board, BoardPiece, Cord, Player};
use rand::Rng;

type PieceType = (BoardPiece, Cord);

#[derive(Clone, Debug)]
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
    //Agression multiplier
    aggresion_multiplier: i32,
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
        aggresion_multiplier: i32,
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
            aggresion_multiplier,
        }
    }

    pub fn default_new() -> Self {
        Self {
            n_piece_val: 100,
            k_piece_val: 150,
            d_hr_mul: 5,
            true_center: 5,
            off_center: 3,
            goalies_center: 8,
            goalies_side: 4,
            per_move_val: 4,
            per_jump_move_val: 8,
            aggresion_multiplier: 5,
        }
    }

    pub fn mutate(&self) -> Self {
        let mut rng = rand::thread_rng();
        let rng_n_piece_val = std::cmp::max(1, self.n_piece_val / 10);
        let rng_k_piece_val = std::cmp::max(1, self.k_piece_val / 10);
        let rng_d_hr_mul = std::cmp::max(1, self.d_hr_mul / 10);
        let rng_true_center = std::cmp::max(1, self.true_center / 10);
        let rng_off_center = std::cmp::max(1, self.off_center / 10);
        let rng_goalies_center = std::cmp::max(1, self.goalies_center / 10);
        let rng_goalies_side = std::cmp::max(1, self.goalies_side / 10);
        let rng_per_move_val = std::cmp::max(1, self.per_move_val / 10);
        let rng_per_jump_move_val = std::cmp::max(1, self.per_jump_move_val / 10);
        let rng_aggresion_multiplier = std::cmp::max(1, self.aggresion_multiplier / 10);

        Self::new(
            std::cmp::min(
                0,
                self.n_piece_val + rng.gen_range(-rng_n_piece_val..rng_n_piece_val),
            ),
            std::cmp::min(
                0,
                self.k_piece_val + rng.gen_range(-rng_k_piece_val..rng_k_piece_val),
            ),
            std::cmp::min(
                0,
                self.d_hr_mul + rng.gen_range(-rng_d_hr_mul..rng_d_hr_mul),
            ),
            std::cmp::min(
                0,
                self.true_center + rng.gen_range(-rng_true_center..rng_true_center),
            ),
            std::cmp::min(
                0,
                self.off_center + rng.gen_range(-rng_off_center..rng_off_center),
            ),
            std::cmp::min(
                0,
                self.goalies_center + rng.gen_range(-rng_goalies_center..rng_goalies_center),
            ),
            std::cmp::min(
                0,
                self.goalies_side + rng.gen_range(-rng_goalies_side..rng_goalies_side),
            ),
            std::cmp::min(
                0,
                self.per_move_val + rng.gen_range(-rng_per_move_val..rng_per_move_val),
            ),
            std::cmp::min(
                0,
                self.per_jump_move_val
                    + rng.gen_range(-rng_per_jump_move_val..rng_per_jump_move_val),
            ),
            std::cmp::min(
                0,
                self.aggresion_multiplier
                    + rng.gen_range(-rng_aggresion_multiplier..rng_aggresion_multiplier),
            ),
        )
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
            } else {
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

        // fn fold_func(
        //     plyr: Player,
        //     op_list: &Vec<(BoardPiece, (usize, usize))>,
        //     max_distance: &mut i32,
        // ) -> impl for<'a> Fn(i32, &'a (BoardPiece, (usize, usize))) -> i32 {
        //     return move |prev: i32, pt: &PieceType| {
        //         return prev + per_piece(pt, plyr, op_list);
        //     };
        // }

        let fold_func = |plyr: Player| {
            return move |prev: (i32, &Vec<BoardPiece, Cord>), pt: &PieceType| {
                return (prev.0 + per_piece(pt, plyr), prev.1);
            };
        };

        score += my_pieces
            .iter()
            .fold((0, &other_pieces), fold_func(state.get_current_player()))
            .0;
        // score += score_adder;

        score -= other_pieces
            .iter()
            .fold((0, &my_pieces), fold_func(state.get_current_player()))
            .0;
        // score -= score_subber;

        score += self.mobility(state);

        score += self.aggresion_value(my_pieces.len() as f32, other_pieces.len() as f32);

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
        return (7 - (cords.1 as i32 - goal).abs()) * self.d_hr_mul;
    }

    fn piece_type_value(&self, piece: &BoardPiece) -> i32 {
        match piece.is_king() {
            true => self.k_piece_val,
            false => self.n_piece_val,
        }
    }

    fn aggresion_value(&self, cp_piece_count: f32, op_piece_count: f32) -> i32 {
        let mut sign: f32 = 1f32;
        let mut big: f32 = cp_piece_count;
        let mut little: f32 = op_piece_count;
        if op_piece_count == cp_piece_count {
            return 0;
        } else if op_piece_count > cp_piece_count {
            sign = -1f32;
            big = op_piece_count;
            little = cp_piece_count;
        }
        return ((big / little) * self.aggresion_multiplier as f32 * sign).ceil() as i32;
    }
}
