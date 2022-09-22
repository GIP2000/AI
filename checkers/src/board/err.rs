#[derive(Debug)]
pub enum MoveError {
    PlayerCantJump,
    PlayerMustJump, 
    InvalidJump,
    OffBoardError,
    BackwardsNotKing, 
    NotPlayerTurn,
    TriedToMoveEmpty,
    SpaceOccupied
}
