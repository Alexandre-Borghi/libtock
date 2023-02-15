use crate::position::Position;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Player {
    pawns: [Position; 4],
}
