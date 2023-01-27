pub mod chess_error {
    use std::fmt;
    use crate::chess::color::color;
    use crate::chess::piece::piece;
    use crate::chess::piece::piece::Piece;
    use crate::chess::position::position;
    use crate::chess::position::position::Position;
    use crate::chess::color::color::Color;

    type Result<T> = std::result::Result<T, ChessError>;

    #[derive(Debug, Clone)]
    pub enum ChessError {
        InvalidMove {
            message: String,
            from: Position,
            to: Position,
            piece: Piece,
            color: Color
        },
        PieceNotFound {
            from: Position
        },
        MovePieceError {
            from: Position,
            to: Position,
            piece: Piece,
            color: Color
        }
    }

    impl fmt::Display for ChessError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match self {
                ChessError::MovePieceError { from, to, piece, color } =>
                    write!(f, "Move piece error: {} => {} for {} of {} color.", from, to, piece, color),
                ChessError::PieceNotFound { from } =>
                    write!(f, "piece not found at: {}", from),
                ChessError::InvalidMove { message, from, to, piece, color } =>
                    write!(f, "Invalid move for piece with message: {}\nCouldnt move {} => {} for {} of {} color.", message, from, to, piece, color)
            }
        }
    }

    impl ChessError {
        pub fn move_piece_error(from: &Position, to: &Position, piece: &Piece, color: &Color) -> ChessError {
            return ChessError::MovePieceError {
                from: from.clone(),
                to: to.clone(),
                piece: piece.clone(),
                color: color.clone()
            }
        }

        pub fn piece_not_found_error(from: &Position) -> ChessError {
            return ChessError::PieceNotFound {
                from: from.clone(),
            }
        }

        pub fn invalid_move(s: String, from: &Position, to: &Position, piece: &Piece, color: &Color) -> ChessError {
            return ChessError::InvalidMove {
                message: s.clone(),
                from: from.clone(),
                to: to.clone(),
                piece: piece.clone(),
                color: color.clone()
            }
        }
    }
}