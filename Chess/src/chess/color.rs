pub mod color {
    use std::{fmt, ops};
    use crate::chess::piece::piece::Piece;

    const FOURTH_BIT_BITS_MASK: u8 = 0b0000_1000;

    #[derive(Debug, Clone)]
    pub enum Color {
        White = 0,
        Black = 8
    }

    impl Color {
        #[allow(dead_code)]
        pub fn from_u8(value: u8) -> Color {
            match value {
                0 => Color::White,
                8 => Color::Black,
                _ => panic!("Unknown value: {}", value),
            }
        }

        pub fn to_u8(value: &Color) -> u8 {
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

        pub fn get_piece_color(input: u8) -> Color {
            if input & FOURTH_BIT_BITS_MASK == 0 {
                return Color::White
            }
            return Color::Black
        }
    }

    impl ops::Add<Color> for Piece {
        type Output = u8;

        fn add(self, _rhs: Color) -> u8 {
            return Piece::to_u8(self) + Color::to_u8(&_rhs);
        }
    }

    impl PartialEq<Color> for Color {
        fn eq(&self, other: &Color) -> bool {
            return Color::to_u8(self) == Color::to_u8(other)
        }

        fn ne(&self, other: &Color) -> bool {
            return Color::to_u8(self) != Color::to_u8(other)
        }
    }

    impl fmt::Display for Color {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match *self {
                Color::White => write!(f, "White"),
                Color::Black => write!(f, "Black")
            }
        }
    }
}