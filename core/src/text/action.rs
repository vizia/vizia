use super::Movement;

pub enum Action {
    Move(Movement),

    MoveSelecting(Movement),
}
