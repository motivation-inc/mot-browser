use crate::lexer::position::Position;

/// A span object.
pub struct Span {
    start: Position,
    end: Position,
}

impl Span {
    /// Constructs a new span object.
    ///
    /// - `start`: the starting position of the span
    /// - `end`: the ending position of the span
    pub fn new(start: Position, end: Position) -> Self {
        Self { start, end }
    }
}
