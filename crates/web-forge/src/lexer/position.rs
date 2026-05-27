/// A position object.
#[derive(Debug, Clone, Copy)]
pub struct Position {
    pub index: usize,
    pub line_number: usize,
    pub column_number: usize,
}

impl Position {
    /// Constructs a new position object.
    ///
    /// - `index`: the index of the position
    /// - `line_number`: the line number
    /// - `column_number`: the column number
    pub fn new(index: usize, line_number: usize, column_number: usize) -> Self {
        Self {
            index,
            line_number,
            column_number,
        }
    }

    /// Advances the index and line/column numbers of the position.
    ///
    /// - `current_char`: the character to advance to
    pub fn advance(&mut self, current_char: Option<char>) {
        self.index += 1;
        self.column_number += 1;

        if let Some(character) = current_char
            && character == '\n'
        {
            self.line_number += 1;
            self.column_number = 1;
        }
    }
}
