use std::{fmt, ops};
use crate::chess::color::Color;

#[derive(Clone)]
pub enum Piece {

    Pawn = 1,   // 1 or 9
    Bishop = 2, // 2 or 10
    Knight = 3, // 3 or 11
    Rook = 4,   // 4 or 12
    Queen = 5,  // 5 or 13
    King = 6    // 6 or 14
}

impl Piece {
    pub fn from_u8(value: u8) -> Piece {
        match value {
            1 => Piece::Pawn,
            2 => Piece::Bishop,
            3 => Piece::Knight,
            4 => Piece::Rook,
            5 => Piece::Queen,
            6 => Piece::King,
            _ => panic!("Unknown value: {}", value),
        }
    }

    pub fn to_u8(value: Piece) -> u8 {
        return match value {
            Piece::Pawn => 1_u8,
            Piece::Bishop => 2_u8,
            Piece::Knight => 3_u8,
            Piece::Rook => 4_u8,
            Piece::Queen => 5_u8,
            Piece::King => 6_u8,
        }
    }
}

impl fmt::Display for Piece {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Piece::Pawn => write!(f, "Pieces(Pawn)"),
            Piece::Bishop => write!(f, "Pieces(Bishop)"),
            Piece::Knight => write!(f, "Pieces(Knight)"),
            Piece::Rook => write!(f, "Pieces(Rook)"),
            Piece::Queen => write!(f, "Pieces(Queen)"),
            Piece::King => write!(f, "Pieces(King)"),
        }

    }
}

impl ops::Add<Piece> for Color {
    type Output = u8;

    fn add(self, _rhs: Piece) -> u8 {
        return Piece::to_u8(_rhs) + Color::to_u8(self);
    }
}