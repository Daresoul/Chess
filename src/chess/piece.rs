pub mod piece {
    use std::{fmt, ops};
    use std::fmt::Display;
    use crate::chess::color::color;
    use crate::chess::color::color::Color;

    const LOW_3_BITS_MASK: u8 = 0b0000_0111;

    #[derive(Debug, Copy, Clone)]
    pub enum Piece {
        Pawn = 1,
        // 1 or 9
        Bishop = 2,
        // 2 or 10
        Knight = 3,
        // 3 or 11
        Rook = 4,
        // 4 or 12
        Queen = 5,
        // 5 or 13
        King = 6
        // 6 or 14
    }

    impl Piece {
        pub fn from_u8(value: u8) -> Piece {
            match value & LOW_3_BITS_MASK {
                1 => Piece::Pawn,
                2 => Piece::Bishop,
                3 => Piece::Knight,
                4 => Piece::Rook,
                5 => Piece::Queen,
                6 => Piece::King,
                _ => panic!("Unknown value: {}", value),
            }
        }

        pub fn to_u8(self) -> u8 {
            return match self {
                Piece::Pawn => 1_u8,
                Piece::Bishop => 2_u8,
                Piece::Knight => 3_u8,
                Piece::Rook => 4_u8,
                Piece::Queen => 5_u8,
                Piece::King => 6_u8,
            }
        }

        pub fn get_piece_enum(input: u8) -> Piece {
            return Piece::from_u8(input)
        }
    }

    impl fmt::Display for Piece {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match *self {
                Piece::Pawn => write!(f, "Piece(Pawn)"),
                Piece::Bishop => write!(f, "Piece(Bishop)"),
                Piece::Knight => write!(f, "Piece(Knight)"),
                Piece::Rook => write!(f, "Piece(Rook)"),
                Piece::Queen => write!(f, "Piece(Queen)"),
                Piece::King => write!(f, "Piece(King)"),
            }
        }
    }

    impl ops::Add<Piece> for Color {
        type Output = u8;

        fn add(self, _rhs: Piece) -> u8 {
            return Piece::to_u8(_rhs) + Color::to_u8(&self);
        }
    }

    impl PartialEq for Piece {
        fn eq(&self, _rhs: &Self) -> bool {
            return self.to_u8() == _rhs.to_u8()
        }
    }
}