use std::fmt;

#[derive(Clone)]
pub struct Position {
    pub column: usize, // x
    pub row: usize  // y
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Position {column, row} => write!(f, "({}, {})", column, row)
        }

    }
}