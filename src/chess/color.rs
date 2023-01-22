use std::{fmt, ops};
use crate::chess::piece::Piece;

#[derive(Clone, Eq, PartialEq)]
pub enum Color {
    White = 0,
    Black = 8
}

impl Color {
    pub fn from_u8(value: u8) -> Color {
        match value {
            0 => Color::White,
            8 => Color::Black,
            _ => panic!("Unknown value: {}", value),
        }
    }

    pub fn to_u8(value: Color) -> u8 {
        return match value {
            Color::White => 0_u8,
            Color::Black => 8_u8,
        }
    }

    pub fn to_opposite(value: Color) -> Color {
        match value {
            Color::White => Color::Black,
            Color::Black => Color::White
        }
    }
}

impl ops::Add<Color> for Piece {
    type Output = u8;

    fn add(self, _rhs: Color) -> u8 {
        return Piece::to_u8(self) + Color::to_u8(_rhs);
    }
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Color::White => write!(f, "Color(White)"),
            Color::Black => write!(f, "Color(Black)")
        }

    }
}