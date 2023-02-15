/// Position of a pawn on the board.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Position {
    /// The pawn is in the starting camp, ready to be got out by playing an ace
    /// or a king. The inner value is in 0..=3, 0 being the left-most spot and 3
    /// being the right-most one.
    Start(u8),

    /// The pawn is out on the board, and can either get eaten and go back to
    /// the starting camp or get in the end camp. The inner value is in 0..72.
    Out(u8),

    /// The pawn is safe in the end camp, and can only go forward if not in its
    /// final spot. The inner value is in 0..=3, where 0 is the bottom-most spot
    /// and 3 is the top-most (furthest from the '18' spot).
    End(u8),
}

impl Default for Position {
    fn default() -> Self {
        Self::Start(0)
    }
}
