/// Indicates what the input wants to do.  May not always be relevant
#[derive(Clone)]
pub enum GameInputEvent {
    MoveLeft,
    MoveRight,
    MoveUp,
    MoveDown,
}
