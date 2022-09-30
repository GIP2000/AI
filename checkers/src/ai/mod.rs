use crate::board::Board; 

pub fn predict_move(b: Board) -> usize {
    let player_info_rc = b.get_player_info(); 
    let player_info = player_info_rc.borrow();
    let a = player_info.get_moves();
    
    if a.len() == 1 {
        return 0;
    }
    1


}

