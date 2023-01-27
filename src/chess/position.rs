pub mod position {
    use std::fmt;

    #[derive(Debug, Clone, Copy)]
    pub struct Position {
        pub column: usize, // x
        pub row: usize  // y
    }

    impl Position {
        pub fn new(column: usize, row: usize) -> Position {
            return Position {
                column,
                row
            }
        }
    }

    impl PartialEq<Position> for Position {
        fn eq(&self, other: &Position) -> bool {
            return self.row == other.row && self.column == other.column
        }
    }

    impl fmt::Display for Position {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match *self {
                Position { column, row } => write!(f, "({}, {})", column, row)
            }
        }
    }
}