use crate::board::{Board, BoardPiece, Cord, Player};

const N_PIECE_VAL: i32 = 100;
const K_PIECE_VAL: i32 = 300;
const D_HR_MUL: i32 = 5;
const TRUE_CENTER: i32 = 50;
const OFF_CENTER: i32 = 25;
const GOALIES_CENTER: i32 = 25;
const GOALIES_SIDE: i32 = 25;
const PER_MOVE_VAL: i32 = 5;
const PER_JUMP_MOVE_VAL: i32 = 8;

type PieceType = (BoardPiece, Cord);

pub fn h(state:&Board, is_max: bool) -> i32 {
    let (my_pieces, other_pieces) = state.get_pieces();
    let mut score = 0;

    // let is_end_game = my_pieces.len() + other_pieces.len() < 6;

    let per_piece = |pt: &PieceType, plyr: Player|{
        let (bp, bc) = pt;
        let mut current_score = piece_type_value(bp);
        current_score += depth_distance(bc, match plyr {
            Player::Red => 0,
            Player::Black => 7
        });
        current_score += in_center(bc);
        current_score += in_goal(bc, match plyr {
            Player::Red => 7,
            Player::Black => 0
        });

        return current_score;
    };

    let fold_func = |plyr: Player| {
        return move |prev: i32, pt: &PieceType| {
            return prev + per_piece(pt, plyr);
        }
    };

    score += my_pieces.iter().fold(0, fold_func(state.get_current_player()));
    score -= other_pieces.iter().fold(0, fold_func(state.get_current_player().get_other()));
    score += mobility(state);

    if !is_max {
        score = -score;
    }
    return score;
}

fn mobility(state: &Board) -> i32 {
    match state.get_player_info().borrow().get_can_jump() {
        true => state.get_player_info().borrow().get_moves().len() as i32 * PER_JUMP_MOVE_VAL,
        false => state.get_player_info().borrow().get_moves().len() as i32 * PER_MOVE_VAL,
    }
}

fn in_goal(cords: &Cord, home_row: usize) -> i32 {
    let &(row, col) = cords;

    if row == home_row {
        if col > 1 && col < 6 {
            return GOALIES_CENTER;
        }
        return GOALIES_SIDE;
    }

    return 0;
}

fn in_center(cords: &Cord) -> i32 {
    let &(row,col) = cords;
    if row == 3 || row == 4 {
        if col == 3 || col == 4 {
            return TRUE_CENTER;
        }
        return OFF_CENTER;
    }
    return 0;
}

fn depth_distance(cords: &Cord, goal: i32) -> i32 {
    return (cords.1 as i32 - goal).abs() * D_HR_MUL;
}

fn piece_type_value(piece: &BoardPiece) -> i32 {
    match piece.is_king() {
        true => K_PIECE_VAL,
        false => N_PIECE_VAL
    }
}
