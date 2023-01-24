use std::fmt;
use crate::chess::{Color, Piece, Position};

type Result<T> = std::result::Result<T, ChessError>;

#[derive(Debug, Clone)]
pub enum ChessError {
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
        write!(f, "invalid first item to double")
    }
}

impl ChessError {
    pub fn new_move_piece_error(from: &Position, to: &Position, piece: &Piece, color: &Color) -> ChessError {
        return ChessError::MovePieceError {
            from: from.clone(),
            to: to.clone(),
            piece: piece.clone(),
            color: color.clone()
        }
    }

    pub fn new_piece_not_found_error(from: &Position) -> ChessError {
        return ChessError::PieceNotFound {
            from: from.clone(),
        }
    }
}