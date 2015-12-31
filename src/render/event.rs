use super::Direction;

pub enum Event {
    CameraTurn(Direction),
    Move(Direction),
    Jump,
    Sneak(bool),
    PlaceBlock(usize),
    RemoveBlock(usize),
}
