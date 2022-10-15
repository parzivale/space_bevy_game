use leafwing_input_manager::prelude::*;

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug)]
pub enum PlayerActions {
    Forward,
    Left,
    Right,
    Backward,
    Jump,
    Look,
}
