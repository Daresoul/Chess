mod chess_error;
mod log;
pub(crate) mod r#move;

pub mod game {
    use std::fmt::Display;
    use array2d::Array2D;
    use crate::chess::color::color::Color;
    use crate::chess::game::log::log::Log;
    use crate::chess::game::chess_error::chess_error::ChessError;
    use crate::chess::game::r#move::chess_move::{ChessMove, MoveType};
    use crate::chess::piece::piece::Piece;
    use crate::chess::position::position::Position;

    #[derive(Clone)]
    pub struct Game {
        pub board: Array2D<u8>,
        pub turn: u32,
        pub log: Log,
    }

    impl Game {
        pub fn new() -> Game {
            return Game {
                board: Array2D::filled_with(0, 8, 8),
                turn: 0,
                log: Log::new()
            }
        }

        pub fn default() -> Game {
            return Game::create_board_from_string("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR", 0);
        }

        pub fn create_board_from_string(positions: &str, turn: u32) -> Game {
            let mut game = Game::new();
            game.turn = turn;
            let mut x = 0; // horizontal <---->
            let mut y = 0; // up and down
            for c in positions.chars() {
                match c {
                    '/' => {
                        y += 1;
                        x = 0
                    },
                    '1' => x += 1,
                    '2' => x += 2,
                    '3' => x += 3,
                    '4' => x += 4,
                    '5' => x += 5,
                    '6' => x += 6,
                    '7' => x += 7,
                    '8' => x += 8,
                    'r' => match game.board.set(y, x, Piece::Rook + Color::Black) {
                        Ok(_) => x += 1,
                        Err(e) => println!("Couldnt set the board {e:?}")
                    },
                    'b' => match game.board.set(y, x, Piece::Bishop + Color::Black) {
                        Ok(_) => x += 1,
                        Err(e) => println!("Couldnt set the board {e:?}")
                    },
                    'n' => match game.board.set(y, x, Piece::Knight + Color::Black) {
                        Ok(_) => x += 1,
                        Err(e) => println!("Couldnt set the board {e:?}")
                    },
                    'k' => match game.board.set(y, x, Piece::King + Color::Black) {
                        Ok(_) => x += 1,
                        Err(e) => println!("Couldnt set the board {e:?}")
                    },
                    'q' => match game.board.set(y, x, Piece::Queen + Color::Black) {
                        Ok(_) => x += 1,
                        Err(e) => println!("Couldnt set the board {e:?}")
                    },
                    'p' => match game.board.set(y, x, Piece::Pawn + Color::Black) {
                        Ok(_) => x += 1,
                        Err(e) => println!("Couldnt set the board {e:?}"),
                    }
                    'R' => match game.board.set(y, x, Piece::Rook + Color::White) {
                        Ok(_) => x += 1,
                        Err(e) => println!("Couldnt set the board {e:?}")
                    }, //for rooks
                    'B' => match game.board.set(y, x, Piece::Bishop + Color::White) {
                        Ok(_) => x += 1,
                        Err(e) => println!("Couldnt set the board {e:?}")
                    },
                    'N' => match game.board.set(y, x, Piece::Knight + Color::White) {
                        Ok(_) => x += 1,
                        Err(e) => println!("Couldnt set the board {e:?}")
                    },
                    'K' => match game.board.set(y, x, Piece::King + Color::White) {
                        Ok(_) => x += 1,
                        Err(e) => println!("Couldnt set the board {e:?}")
                    },
                    'Q' => match game.board.set(y, x, Piece::Queen + Color::White) {
                        Ok(_) => x += 1,
                        Err(e) => println!("Couldnt set the board {e:?}")
                    },
                    'P' => match game.board.set(y, x, Piece::Pawn + Color::White) {
                        Ok(_) => x += 1,
                        Err(e) => println!("Couldnt set the board {e:?}")
                    },
                    _ => println!("piece not known.")
                }
            }
            return game
        }

        fn option_to_printable_string<T: Display>(option: &Option<T>) -> String {
            let mut string = String::new();
            match option {
                Some(t) => {
                    let formatted = format!("Some({})", t.to_string());
                    string.push_str(formatted.as_str());

                },
                None => {
                    string.push_str("None");
                }
            }

            return string
        }

        pub fn available_moves_to_string(available_moves: &Vec<ChessMove>) -> String {
            let mut string = String::new();
            string.push_str("[\n");
            for chess_move in available_moves.iter() {
                match chess_move.move_type {
                    MoveType::Move => {
                        string.push_str("move: ")
                    },
                    MoveType::EnPassant(_kill_pos) => {
                        string.push_str("Ein: ")
                    },
                    MoveType::Castle(_from, _to) => {
                        string.push_str("Castle: ")
                    },
                    MoveType::Promote => {
                        string.push_str("Promote: ")
                    }
                }

                let formatted_move = format!(
                    "({}, {}, {}) ==> ({}, {})\n",
                    chess_move.from.to_string(),
                    chess_move.piece.to_string(),
                    chess_move.color.to_string(),
                    Self::option_to_printable_string(&chess_move.piece_at_position).to_string(),
                    chess_move.to.to_string()
                );
                string.push_str(&formatted_move)
            }
            string.push_str("]\n");
            return string;
        }

        fn create_log(&mut self, chess_move: &ChessMove) {
            self.log.append(vec![chess_move.clone()]);
        }

        pub fn get_turn(&self) -> Color {
            if self.turn % 2 == 0 {
                return Color::White;
            }

            return Color::Black
        }

        fn is_check(&self) -> Vec<Color> {
            let all_moves = self.get_all_available_moves();
            let mut checks: Vec<Color> = vec![];

            for chess_move in all_moves {
                match chess_move.piece_at_position
                {
                    None => continue,
                    Some(captured_piece) => {
                        if captured_piece == Piece::King {
                            checks.append(&mut vec![Color::to_opposite(chess_move.color)])
                        }
                    }
                }
            }
            return checks
        }

        fn is_valid_move(mut game: Game, chess_move: &ChessMove) -> bool {
            let turn = game.get_turn();

            match game.move_piece(&turn, chess_move) {
                Ok(_t) => (),
                Err(err) => println!("{}", err),
            }

            let checks = game.is_check();

            for check_color in checks.iter() {
                if *check_color == turn {
                    return false;
                }
            }

            return true
        }

        pub fn move_exists_in_list(available_moves: &Vec<ChessMove>, new_pos: &Position) -> Option<ChessMove> {
            for chess_move in available_moves {
                if chess_move.to == *new_pos {
                    return Some(chess_move.clone())
                }
            }
            return None;
        }

        fn move_piece(&mut self, turn: &Color, chess_move: &ChessMove) -> Result<Game, ChessError> {
            if chess_move.color == *turn {
                match self.board.set(chess_move.from.row, chess_move.from.column, 0) {
                    Ok(_) => (),
                    Err(_) => panic!("board couldnt be set."),
                };
                match self.board.set(chess_move.to.row, chess_move.to.column, chess_move.piece + chess_move.color.clone()) {
                    Ok(_) => (),
                    Err(_) => panic!("board couldnt be set."),
                };

                match chess_move.move_type {
                    MoveType::Move => (),
                    MoveType::EnPassant(take_position) => {
                        match self.board.set(take_position.row, take_position.column, 0) {
                            Ok(_) => (),
                            Err(_) => panic!("board couldnt be set."),
                        };
                    },
                    MoveType::Castle(from, to) => {
                        match self.board.set(from.row, from.column, 0) {
                            Ok(_) => (),
                            Err(_) => panic!("board couldnt be set."),
                        };
                        match self.board.set(to.row, to.column, Piece::Rook + chess_move.color.clone()) {
                            Ok(_) => (),
                            Err(_) => panic!("board couldnt be set."),
                        };
                    },
                    MoveType::Promote => ()
                }

                self.create_log(&chess_move);
                return Ok(self.clone());
            }

            return Err(ChessError::move_piece_error(&chess_move.from, &chess_move.to, &chess_move.piece, &chess_move.color));
        }

        pub fn try_move_piece(&mut self, from: &Position, to: &Position) -> Result<Game, ChessError> {
            let piece_available_moves = self.get_available_moves(from);

            let can_move = Self::move_exists_in_list(&piece_available_moves, to);

            let turn = self.get_turn();

            match can_move {
                Some(chess_move) => {
                    if Self::is_valid_move(self.clone(), &chess_move)
                    {
                        self.move_piece(&turn, &chess_move).unwrap();
                        self.turn += 1;
                        return Ok(self.clone());
                    } else {
                        let (piece, color) = self.get_piece_from_position(&from).unwrap();
                        return Err(ChessError::invalid_move("Some String1".to_string(), from, to, &piece, &color));
                    }
                },
                None => {
                    match self.get_piece_from_position(&from) {
                        Some((piece, color)) => return Err(ChessError::invalid_move("Some String2".to_string(), from, to, &piece, &color)),
                        None => return Err(ChessError::piece_not_found_error(from))
                    }
                }
            }
        }

        pub fn get_all_turn_available_moves(&self) -> Vec<ChessMove> {
            let available_moves = self.get_all_available_moves();
            let turn = self.get_turn();
            let mut moves = vec![];

            for chess_move in available_moves {
                if turn == chess_move.color {

                    moves.append(&mut vec![chess_move]);
                }
            }

            return moves;
        }

        pub fn get_all_available_moves(&self) -> Vec<ChessMove> {
            let mut moves = vec![];
            for x in 0..self.board.num_columns() {
                for y in 0..self.board.num_rows() {
                    let position = Position::new(x, y);
                    moves.append(&mut self.get_available_moves(&position))
                }
            }

            return moves
        }


        pub fn get_available_moves(&self, pos: &Position) -> Vec<ChessMove> {
            match self.get_piece_from_position(pos) {
                None => vec![],
                Some((piece, color)) => {
                    match piece {
                        Piece::Rook => return self.available_rook_moves(&piece, &color, pos),
                        Piece::Knight => return self.available_knight_moves(&piece, &color, pos),
                        Piece::King => return self.available_king_moves(&piece, &color, pos),
                        Piece::Bishop => return self.available_bishop_moves(&piece, &color, pos),
                        Piece::Queen => return self.available_queen_moves(&piece, &color, pos),
                        Piece::Pawn => return self.available_pawn_moves(&piece, &color, pos),
                    }
                }
            }
        }

        pub fn get_piece_from_position(&self, pos: &Position) -> Option<(Piece, Color)> {
            match self.board.get(pos.row, pos.column) {
                None => panic!("Couldnt get the position: ({}, {}).", pos.column, pos.row),
                Some(piece_value) => {
                    if *piece_value == 0 {
                        return None
                    }

                    let piece_color = Color::get_piece_color(*piece_value);
                    let piece = Piece::get_piece_enum(*piece_value);

                    return Some((piece, piece_color))
                }
            }
        }

        fn create_iter(start: &usize, end: &usize, rev: &bool) -> Vec<usize> {
            let mut moves: Vec<usize> = vec![];

            for i in *start..*end {
                moves.append(&mut vec![i])
            }

            if *rev {
                moves.reverse();
            }

            return moves;
        }

        fn rook_row_column(iter_value: &usize, val: &usize, direction: u8) -> Position {
            if direction == 1 || direction == 3 {
                return Position::new(*iter_value, *val)
            } else {
                return Position::new(*val, *iter_value)
            }
        }

        fn array_moves(&self, pos: &Position, piece: &Piece, color: &Color, offset: Vec<(i8, i8, usize, usize)>) -> Vec<ChessMove> {
            let mut moves: Vec<ChessMove> = vec![];

            for (x, y, x_usize, y_usize) in offset.iter() {
                let new_column =
                    if *x > 0 { // *x = 1
                        if pos.column > 7 - x_usize { continue } // 4 > 8 - 1
                        pos.column + x_usize
                    } else {
                        if pos.column < *x_usize { continue }
                        pos.column - x_usize
                    };

                let new_row =
                    if *y > 0 { // *y = -2
                        if pos.row > 7 - y_usize { continue }
                        pos.row + y_usize
                    } else {
                        if pos.row < *y_usize { continue } // 4 < 2
                        pos.row - y_usize
                    };

                let move_position = Position::new(new_column, new_row);

                match self.get_piece_from_position(&move_position) {
                    None => {
                        moves.append(&mut vec![ChessMove::new(
                            pos.clone(),
                            move_position,
                            piece.clone(),
                            color.clone(),
                            None,
                            MoveType::Move
                        )]);
                    },
                    Some((captured_piece, capture_color)) => {
                        if *color != capture_color {
                            moves.append(&mut vec![ChessMove::new(
                                pos.clone(),
                                move_position,
                                piece.clone(),
                                color.clone(),
                                Some(captured_piece),
                                MoveType::Move
                            )]);
                        } else { continue }
                    }
                }
            }

            return moves;
        }

        // Direction:
// 0 = up
// 1 = right
// 2 = down
// 3 = left
        fn rook_moves(&self, pos: &Position, piece: &Piece, color: &Color, direction: u8) -> Vec<ChessMove> {
            let mut moves: Vec<ChessMove> = vec![];
            let iter =
                if direction == 0 { Self::create_iter(&0, &pos.row, &true) } else if direction == 1 { Self::create_iter(&(pos.column + 1), &8, &false) } else if direction == 2 { Self::create_iter(&(pos.row + 1), &8, &false) } else { Self::create_iter(&0, &pos.column, &true) };

            for i in iter.iter() {
                let value = if direction == 1 || direction == 3 { pos.row } else { pos.column };
                let move_position = Self::rook_row_column(i, &value, direction);
                match self.get_piece_from_position(&move_position) {
                    None => {
                        moves.append(&mut vec![ChessMove::new(
                            pos.clone(),
                            move_position,
                            piece.clone(),
                            color.clone(),
                            None,
                            MoveType::Move
                        )]);
                    },
                    Some((captured_piece, capture_color)) => {
                        if *color != capture_color {
                            moves.append(&mut vec![ChessMove::new(
                                pos.clone(),
                                move_position,
                                piece.clone(),
                                color.clone(),
                                Some(captured_piece),
                                MoveType::Move
                            )]);
                        }

                        break;
                    }
                }
            }

            return moves;
        }

        // Direction
// 0: diagonally up left - -
// 1: diagonally up right + -
// 2: diagonally down right + +
// 3: diagonally down left - +
        fn diagonal_moves(&self, pos: &Position, piece: &Piece, color: &Color, direction: u8) -> Vec<ChessMove> {
            let mut moves: Vec<ChessMove> = vec![];

            for i in 0..8 {
                let move_position = if direction == 0 {
                    if pos.column >= (i + 1) && pos.row >= (i + 1) {
                        Position::new(pos.column - (i + 1), pos.row - (i + 1))
                    } else {
                        break;
                    }
                } else if direction == 1 {
                    if pos.column < 8 - (i + 1) && pos.row >= (i + 1) {
                        Position { column: pos.column + (i + 1), row: pos.row - (i + 1) }
                    } else {
                        break;
                    }
                } else if direction == 2 {
                    if pos.column < 8 - (i + 1) && pos.row < 8 - (i + 1) {
                        Position { column: pos.column + (i + 1), row: pos.row + (i + 1) }
                    } else {
                        break;
                    }
                } else {
                    if pos.column >= (i + 1) && pos.row < 8 - (i + 1) {
                        Position { column: pos.column - (i + 1), row: pos.row + (i + 1) }
                    } else {
                        break;
                    }
                };

                match self.get_piece_from_position(&move_position) {
                    None => {
                        moves.append(&mut vec![ChessMove::new(
                            pos.clone(),
                            move_position,
                            piece.clone(),
                            color.clone(),
                            None,
                            MoveType::Move
                        )]);
                    },
                    Some((captured_piece, capture_color)) => {
                        if *color != capture_color {
                            moves.append(&mut vec![ChessMove::new(
                                pos.clone(),
                                move_position,
                                piece.clone(),
                                color.clone(),
                                Some(captured_piece),
                                MoveType::Move
                            )]);
                        }

                        break;
                    }
                }
            }

            return moves;
        }

        fn available_rook_moves(&self, piece: &Piece, color: &Color, pos: &Position) -> Vec<ChessMove> {
            let mut moves: Vec<ChessMove> = vec![];

            moves.append(&mut self.rook_moves(pos, piece, color, 0));
            moves.append(&mut self.rook_moves(pos, piece, color, 1));
            moves.append(&mut self.rook_moves(pos, piece, color, 2));
            moves.append(&mut self.rook_moves(pos, piece, color, 3));

            return moves;
        }

        fn available_knight_moves(&self, piece: &Piece, color: &Color, pos: &Position) -> Vec<ChessMove> {
            let mut moves: Vec<ChessMove> = vec![];

            let knight_offsets: Vec<(i8, i8, usize, usize)> = vec![
                (-1, -2, 1, 2), (1, -2, 1, 2), (2, -1, 2, 1), (2, 1, 2, 1),
                (1, 2, 1, 2), (-1, 2, 1, 2), (-2, 1, 2, 1), (-2, -1, 2, 1)
            ];

            moves.append(&mut self.array_moves(pos, piece, color, knight_offsets));

            return moves;
        }

        fn available_king_moves(&self, piece: &Piece, color: &Color, pos: &Position) -> Vec<ChessMove> {
            let mut moves: Vec<ChessMove> = vec![];

            let king_offset: Vec<(i8, i8, usize, usize)> = vec![
                (0, -1, 0, 1), (1, -1, 1, 1), (1, 0, 1, 0), (1, 1, 1, 1),
                (0, 1, 0, 1), (-1, 1, 1, 1), (-1, 0, 1, 0), (-1, -1, 1, 1)
            ];

            moves.append(&mut self.array_moves(pos, piece, color, king_offset));

            // castling

            let king_starter_square =
                match color {
                    Color::White => Position::new(4, 7),
                    Color::Black => Position::new(4, 0)
                };

            let rook_left_starter_square = match color {
                Color::White => Position::new(0, 7),
                Color::Black => Position::new(0, 0)
            };

            let rook_right_starter_square = match color {
                Color::White => Position::new(7, 7),
                Color::Black => Position::new(7, 0)
            };

            match self.get_piece_from_position(&king_starter_square) {
                None => return moves,
                Some(_) => ()
            }

            // TODO: optimize performance
            if !self.log.piece_has_moved_from_starting_square(&king_starter_square) {
                if !self.log.piece_has_moved_from_starting_square(&rook_left_starter_square) {

                    let space_left_free =
                        match color {
                            Color::White =>
                                if  self.get_piece_from_position(&Position::new(1, 7)) == None &&
                                    self.get_piece_from_position(&Position::new(2, 7)) == None &&
                                    self.get_piece_from_position(&Position::new(3, 7)) == None {
                                        true
                                } else {false},
                            Color::Black =>
                                if  self.get_piece_from_position(&Position::new(3, 0)) == None &&
                                    self.get_piece_from_position(&Position::new(2, 0)) == None &&
                                    self.get_piece_from_position(&Position::new(1, 0)) == None {
                                    true
                                } else {false}
                        };

                    if space_left_free {
                        let to = Position::new(pos.column-2, pos.row);
                        let rook_to = Position::new(to.column + 1, pos.row);
                        let castle_left_move = ChessMove::new(pos.clone(), to, piece.clone(), color.clone(), None, MoveType::Castle(rook_left_starter_square, rook_to));
                        moves.append(&mut vec![castle_left_move])
                    }
                }

                if !self.log.piece_has_moved_from_starting_square(&rook_right_starter_square) {

                    let space_right_free =
                        match color {
                            Color::White =>
                                if  self.get_piece_from_position(&Position::new(5, 7)) == None &&
                                    self.get_piece_from_position(&Position::new(6, 7)) == None {
                                    true
                                } else {false},
                            Color::Black =>
                                if  self.get_piece_from_position(&Position::new(5, 0)) == None &&
                                    self.get_piece_from_position(&Position::new(6, 0)) == None {
                                    true
                                } else {false}
                        };

                    if space_right_free {
                        let to = Position::new(pos.column + 2, pos.row);
                        let rook_to = Position::new(to.column - 1, pos.row);
                        let castle_left_move = ChessMove::new(pos.clone(), to, piece.clone(), color.clone(), None, MoveType::Castle(rook_right_starter_square, rook_to));
                        moves.append(&mut vec![castle_left_move])
                    }
                }
            }


            return moves;
        }

        fn available_bishop_moves(&self, piece: &Piece, color: &Color, pos: &Position) -> Vec<ChessMove> {
            let mut moves: Vec<ChessMove> = vec![];

            moves.append(&mut self.diagonal_moves(pos, piece, color, 0));
            moves.append(&mut self.diagonal_moves(pos, piece, color, 1));
            moves.append(&mut self.diagonal_moves(pos, piece, color, 2));
            moves.append(&mut self.diagonal_moves(pos, piece, color, 3));

            return moves;
        }

        fn available_queen_moves(&self, piece: &Piece, color: &Color, pos: &Position) -> Vec<ChessMove> {
            let mut moves: Vec<ChessMove> = vec![];

            moves.append(&mut self.rook_moves(pos, piece, color, 0));
            moves.append(&mut self.rook_moves(pos, piece, color, 1));
            moves.append(&mut self.rook_moves(pos, piece, color, 2));
            moves.append(&mut self.rook_moves(pos, piece, color, 3));

            moves.append(&mut self.diagonal_moves(pos, piece, color, 0));
            moves.append(&mut self.diagonal_moves(pos, piece, color, 1));
            moves.append(&mut self.diagonal_moves(pos, piece, color, 2));
            moves.append(&mut self.diagonal_moves(pos, piece, color, 3));

            return moves;
        }

        fn available_pawn_moves(&self, piece: &Piece, color: &Color, pos: &Position) -> Vec<ChessMove> {
            // TODO: Add ein peasant
            // TODO: Add promotion
            let mut moves: Vec<ChessMove> = vec![];
            let is_white = *color == Color::White;

            // Do single move
            let row_single = if is_white { pos.row - 1 } else { pos.row + 1 };
            let move_position = Position::new(pos.column, row_single);
            match self.get_piece_from_position(&move_position) {
                None => {
                    moves.append(&mut vec![ChessMove::new(
                        pos.clone(),
                        move_position,
                        piece.clone(),
                        color.clone(),
                        None,
                        MoveType::Move
                    )]);
                }
                Some((_p, _c)) => ()//println!("piece: {}, color: {}, at position {}", p, c, pos.clone())
            }

            if pos.column > 0 {
                let move_position = Position::new(pos.column - 1, row_single);
                match self.get_piece_from_position(&move_position) {
                    None => (),
                    Some((captured_piece, captured_color)) => {
                        let has_same_color = captured_color != *color;
                        if has_same_color {
                            moves.append(&mut vec![ChessMove::new(
                                pos.clone(),
                                move_position,
                                piece.clone(),
                                color.clone(),
                                Some(captured_piece),
                                MoveType::Move
                            )]);
                        }
                    }
                }
            }

            if pos.column < 7 {
                let move_position = Position::new(pos.column + 1, row_single);
                match self.get_piece_from_position(&move_position) {
                    None => (),
                    Some((captured_piece, captured_color)) => {
                        let has_same_color = captured_color != *color;
                        if has_same_color {
                            moves.append(&mut vec![ChessMove::new(
                                pos.clone(),
                                move_position,
                                piece.clone(),
                                color.clone(),
                                Some(captured_piece),
                                MoveType::Move
                            )]);
                        }
                    }
                }
            }

            if (is_white && pos.row == 6) || (!is_white && pos.row == 1) {
                let row_double = if is_white { pos.row - 2 } else { pos.row + 2 };
                let move_position = Position::new(pos.column, row_double);

                match self.get_piece_from_position(&move_position) {
                    None => {
                        moves.append(&mut vec![ChessMove::new(
                            pos.clone(),
                            move_position,
                            piece.clone(),
                            color.clone(),
                            None,
                            MoveType::Move
                        )]);
                    },
                    Some(_) => ()
                }
            }

            let last_move =
                match self.log.get_last_move() {
                    Some(t) => t,
                    None => return moves
                };

            if last_move.piece == Piece::Pawn {
                let diff_len = match last_move.to.row > last_move.from.row {
                    true => {
                        last_move.to.row - last_move.from.row
                    },
                    false => {
                        last_move.from.row - last_move.to.row
                    }
                };

                if diff_len == 2 {
                    let target_pos = match color {
                        Color::White => Position::new(last_move.to.column, last_move.to.row - 1),
                        Color::Black => Position::new(last_move.to.column, last_move.to.row + 1)
                    };

                    if pos.column != 0 {
                        let take_pos_left = match color {
                            Color::White => Position::new(pos.column - 1, pos.row - 1),
                            Color::Black => Position::new(pos.column - 1, pos.row + 1)
                        };

                        if target_pos == take_pos_left {
                            let ein_peasant_left = ChessMove::new(
                                pos.clone(),
                                take_pos_left,
                                piece.clone(),
                                color.clone(),
                                None,
                                MoveType::EnPassant(last_move.to)
                            );
                            moves.append(&mut vec![ein_peasant_left]);
                        }
                    }
                    if pos.column != 7 {
                        let take_pos_right = match color {
                            Color::White => Position::new(pos.column + 1, pos.row - 1),
                            Color::Black => Position::new(pos.column + 1, pos.row + 1)
                        };

                        if target_pos == take_pos_right {
                            let ein_peasant_right = ChessMove::new(
                                pos.clone(),
                                take_pos_right,
                                piece.clone(),
                                color.clone(),
                                None,
                                MoveType::EnPassant(last_move.to)
                            );
                            moves.append(&mut vec![ein_peasant_right]);
                        }
                    }
                }
            }
            return moves;
        }
    }
}