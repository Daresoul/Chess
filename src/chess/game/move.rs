pub mod chess_move {
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
}