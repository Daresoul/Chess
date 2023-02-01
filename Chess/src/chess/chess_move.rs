pub mod chess_move {
    use std::fmt;
    use std::fmt::write;
    use crate::chess::color::color::Color;
    use crate::chess::piece::piece::Piece;
    use crate::chess::position::position::Position;

    #[derive(Clone)]
    pub struct ChessMove {
        pub from: Position,
        pub to: Position,
        pub piece: Piece,
        pub color: Color,
        pub piece_at_position: Option<Piece>,
        pub move_type: MoveType
    }

    //
    // Move: a standard move
    // EnPassant(take position): using ein peasant, containing the position to remove the pawn.
    // Castle(rook from, rook to): holding the data of the rook movement in castling only to should be necessarry tho.
    // Promote: Holding data about the promotion

    #[derive(Debug, Clone)]
    pub enum MoveType {
        Move,
        EnPassant(Position),
        Castle(Position, Position),
        Promote(Piece)
    }

    impl ChessMove {
        pub fn new(
            from: Position,
            to: Position,
            piece: Piece,
            color: Color,
            piece_at_position: Option<Piece>,
            move_type: MoveType
        ) -> ChessMove
        {
            return ChessMove {
                from, to, piece, color, piece_at_position, move_type
            }
        }
    }

    impl fmt::Display for ChessMove {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let mut s = String::new();
            match self.move_type {
                MoveType::Move => s.push_str("Move: "),
                MoveType::EnPassant(position) => s.push_str("En Passant: "),
                MoveType::Castle(from, to) => s.push_str("Castle: "),
                MoveType::Promote(piece) => {
                    let x = format!("Promote ({}): ", piece);
                    s.push_str(x.as_str())
                }
            }

            let x = format!("{} of color {} from {} => {}", self.piece, self.color, self.from, self.to);
            s.push_str(x.as_str());
            write!(f, "{}", s)
        }
    }
}