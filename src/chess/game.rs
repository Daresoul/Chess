mod chess_error;

use array2d::Array2D;
use crate::chess::{Color, get_piece_color, Piece, Position};
use crate::chess::game::chess_error::ChessError;

#[derive(Clone)]
pub struct Game {
    pub board: Array2D<u8>,
    pub turn: u32,
    pub log: Vec<(Position, Position)>,
}

impl Game {
    pub fn new() -> Game {
        return Game {
            board: Array2D::filled_with(0, 8, 8),
            turn: 0,
            log: vec![]
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
                _ => println!("Piece not known.")
            }
        }
        return game
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

        for ((pos, piece, color), (option_piece, position)) in all_moves {
            match option_piece
            {
                None => continue,
                Some(captured_piece) => {
                    if captured_piece == Piece::King {
                        checks.append(&mut vec![Color::to_opposite(color)])
                    }
                }
            }
        }
        return checks
    }

    fn is_valid_move(mut game: Game, from: &Position, to: &Position) -> bool {
        let turn = game.get_turn();

        match game.move_piece(&turn, from, to) {
            Ok(t) => (),
            Err(err) => panic!("lol"),
        }

        let checks = game.is_check();

        for check_color in checks.iter() {
            if *check_color == turn {
                return false;
            }
        }

        return true
    }

    pub fn move_exists_in_list(available_moves: &Vec<((Position, Piece, Color), (Option<Piece>, Position))>, to: &Position) -> bool {
        for (_, (_ ,new_pos)) in available_moves {
            if new_pos.column == to.column && new_pos.row == to.row {
                return true;
            }
        }
        return false;
    }

    fn move_piece(&mut self, turn: &Color, from: &Position, to: &Position) -> Result<Game, ChessError> {
        match self.get_piece_from_position(from) {
            Some((piece, color)) => {
                if color == *turn {
                    match self.board.set(from.row, from.column, 0) {
                        Ok(_) => (),
                        Err(_) => panic!("board couldnt be set."),
                    };
                    match self.board.set(to.row, to.column, piece + color) {
                        Ok(_) => (),
                        Err(_) => panic!("board couldnt be set."),
                    };
                    return Ok(self.clone());
                }

                let error = format!("Trying to move {} from {} of the color {}, to {} but it is {} turn.", piece, from, color, to, turn);
                return Err(ChessError::new_move_piece_error(from, to, &piece, &color))
            }
            None => return Err(ChessError::new_piece_not_found_error(from))
        }
    }

    pub fn try_move_piece(&mut self, from: &Position, to: &Position) -> Result<Game, ChessError> {
        let piece_available_moves = self.get_available_moves(from);

        let can_move = Self::move_exists_in_list(&piece_available_moves, to);

        let turn = self.get_turn();

        if can_move {
            if Self::is_valid_move(self.clone(), from, to)
            {
                self.move_piece(&turn, from, to);
                /*if is_checkmate(new_game.clone()) {
                    println!("Is checkmate")
                }*/
                self.turn += 1;
                return Ok(self.clone());
            }
            else {
                return Err(ChessError::new_piece_not_found_error(from));
            }
        }

        return Err(ChessError::new_piece_not_found_error(from));
    }

    pub fn get_all_turn_available_moves(&self) -> Vec<((Position, Piece, Color), (Option<Piece>, Position))> {
        let available_moves = self.get_all_available_moves();
        let turn = self.get_turn();
        let mut moves = vec![];

        for ((pos, piece, color), (option_piece, position)) in available_moves {
            if turn == color {
                let mut vec = vec![((pos, piece, color), (option_piece, position))];
                moves.append(&mut vec );
            }
        }

        return moves;
    }

    pub fn get_all_available_moves(&self) -> Vec<((Position, Piece, Color), (Option<Piece>, Position))> {
        let mut moves = vec![];
        for x in 0..self.board.num_columns() {
            for y in 0..self.board.num_rows() {
                let position = Position::new(x, y);
                moves.append( &mut self.get_available_moves(&position))
            }
        }

        return moves
    }


    pub fn get_available_moves(&self, pos: &Position) -> Vec<((Position, Piece, Color), (Option<Piece>, Position))> {
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

    fn create_entry_from_position(piece_info: (Position, Piece, Color), move_to_pos_and_piece: (Option<Piece>, Position)) -> Vec<((Position, Piece, Color), (Option<Piece>, Position))> {
        return vec![(piece_info, move_to_pos_and_piece)]
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
            return Position::new(*iter_value,*val)
        }
        else {
            return Position::new(*val,*iter_value)
        }
    }

    fn array_moves(&self, pos: &Position, piece: &Piece, color: &Color, offset: Vec<(i8, i8, usize, usize)>) -> Vec<((Position, Piece, Color), (Option<Piece>, Position))> {
        let mut moves: Vec<((Position, Piece, Color), (Option<Piece>, Position))> = vec![];

        for (x,y, x_usize, y_usize) in offset.iter() {

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

            match self.get_piece_from_position( &move_position) {
                None => {
                    let piece_info = (pos.clone(), piece.clone(), color.clone());
                    let position_information = (None, move_position);
                    moves.append(&mut Game::create_entry_from_position(piece_info, position_information))
                },
                Some((captured_piece, capture_color)) => {
                    if *color != capture_color {
                        let piece_info = (pos.clone(), piece.clone(), color.clone());
                        let position_information = (Some(captured_piece), move_position);
                        moves.append(&mut Game::create_entry_from_position(piece_info, position_information));
                    }
                    else { continue }
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
    fn rook_moves(&self, pos: &Position, piece: &Piece, color: &Color, direction: u8) -> Vec<((Position, Piece, Color), (Option<Piece>, Position))> {
        let mut moves: Vec<((Position, Piece, Color), (Option<Piece>, Position))> = vec![];
        let iter =
            if direction == 0 {Self::create_iter(&0, &pos.row, &true)}
            else if direction == 1 {Self::create_iter(&(pos.column + 1), &8, &false)}
            else if direction == 2 {Self::create_iter(&(pos.row + 1), &8, &false) }
            else { Self::create_iter(&0, &pos.column, &true)};

        for i in iter.iter() {
            let value = if direction == 1 || direction == 3 { pos.row }
            else { pos.column };
            let move_position = Self::rook_row_column(i, &value, direction);
            match self.get_piece_from_position(&move_position) {
                None => {
                    let piece_info = (pos.clone(), piece.clone(), color.clone());
                    let position_information = (None, move_position);
                    moves.append(&mut Self::create_entry_from_position(piece_info, position_information))
                },
                Some((captured_piece, capture_color)) => {
                    if *color != capture_color {
                        let piece_info = (pos.clone(), piece.clone(), color.clone());
                        let position_information = (Some(captured_piece), move_position);
                        moves.append(&mut Self::create_entry_from_position(piece_info, position_information));
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
    fn bishop_moves(&self, pos: &Position, piece: &Piece, color: &Color, direction: u8) -> Vec<((Position, Piece, Color), (Option<Piece>, Position))> {
        let mut moves: Vec<((Position, Piece, Color), (Option<Piece>, Position))> = vec![];
        let iter_vec = Self::create_iter(&0, &pos.row, &true);
        for (i, _) in iter_vec.iter().enumerate() {

            let move_position = if direction == 0 {
                if pos.column >= (i+1) && pos.row >= (i+1) {
                    Position { column: pos.column - (i + 1), row: pos.row - (i + 1) }
                } else {
                    continue;
                }
            } else if direction == 1 {
                if pos.column < 8 - (i + 1) && pos.row >= (i+1) {
                    Position { column: pos.column + (i + 1), row: pos.row - (i + 1) }
                } else {
                    continue;
                }
            } else if direction == 2 {
                if pos.column < 8 - (i + 1) && pos.row < 8 - (i + 1) {
                    Position { column: pos.column + (i+1), row: pos.row + (i+1)}
                } else {
                    continue;
                }
            } else {
                if pos.column >= (i+1) && pos.row < 8 - (i + 1) {
                    Position { column: pos.column - (i+1), row: pos.row + (i+1)}
                } else {
                    continue;
                }
            };
            match self.get_piece_from_position(&move_position) {
                None => {
                    let piece_info = (pos.clone(), piece.clone(), color.clone());
                    let position_information = (None, move_position);
                    moves.append(&mut Self::create_entry_from_position(piece_info, position_information))
                },
                Some((captured_piece, capture_color)) => {
                    if *color != capture_color {
                        let piece_info = (pos.clone(), piece.clone(), color.clone());
                        let position_information = (Some(captured_piece), move_position);
                        moves.append(&mut Self::create_entry_from_position(piece_info, position_information));
                    }

                    break;
                }
            }
        }

        return moves;
    }

    fn available_rook_moves(&self, piece: &Piece, color: &Color, pos: &Position) -> Vec<((Position, Piece, Color), (Option<Piece>, Position))> {
        let mut moves: Vec<((Position, Piece, Color), (Option<Piece>, Position))> = vec![];

        moves.append(&mut self.rook_moves(pos, piece, color, 0));
        moves.append(&mut self.rook_moves(pos, piece, color, 1));
        moves.append(&mut self.rook_moves(pos, piece, color, 2));
        moves.append(&mut self.rook_moves(pos, piece, color, 3));

        return moves;
    }

    fn available_knight_moves(&self, piece: &Piece, color: &Color, pos: &Position) -> Vec<((Position, Piece, Color), (Option<Piece>, Position))> {
        let mut moves: Vec<((Position, Piece, Color), (Option<Piece>, Position))> = vec![];

        let knight_offsets: Vec<(i8, i8, usize, usize)> = vec![
            (-1, -2, 1, 2), (1, -2, 1, 2), (2, -1, 2, 1), (2, 1, 2, 1),
            (1, 2, 1, 2), (-1, 2, 1, 2), (-2, 1, 2, 1), (-2, -1, 2, 1)
        ];

        moves.append(&mut self.array_moves(pos, piece, color, knight_offsets));

        return moves;
    }

    fn available_king_moves(&self, piece: &Piece, color: &Color, pos: &Position) -> Vec<((Position, Piece, Color), (Option<Piece>, Position))> {
        let mut moves: Vec<((Position, Piece, Color), (Option<Piece>, Position))> = vec![];

        let king_offset: Vec<(i8, i8, usize, usize)> = vec![
            (0, -1, 0, 1), (1, -1, 1, 1), (1, 0, 1, 0), (1, 1, 1, 1),
            (0, 1, 0, 1), (-1, 1, 1, 1), (-1, 0, 1, 0), (-1, -1, 1, 1)
        ];

        moves.append(&mut self.array_moves(pos, piece, color, king_offset));

        return moves;
    }

    fn available_bishop_moves(&self, piece: &Piece, color: &Color, pos: &Position) -> Vec<((Position, Piece, Color), (Option<Piece>, Position))> {
        let mut moves: Vec<((Position, Piece, Color), (Option<Piece>, Position))> = vec![];

        moves.append(&mut self.bishop_moves(pos, piece, color, 0));
        moves.append(&mut self.bishop_moves(pos, piece, color, 1));
        moves.append(&mut self.bishop_moves(pos, piece, color, 2));
        moves.append(&mut self.bishop_moves(pos, piece, color, 3));

        return moves;
    }

    fn available_queen_moves(&self, piece: &Piece, color: &Color, pos: &Position) -> Vec<((Position, Piece, Color), (Option<Piece>, Position))> {
        let mut moves: Vec<((Position, Piece, Color), (Option<Piece>, Position))> = vec![];

        moves.append(&mut self.rook_moves(pos, piece, color, 0));
        moves.append(&mut self.rook_moves(pos, piece, color, 1));
        moves.append(&mut self.rook_moves(pos, piece, color, 2));
        moves.append(&mut self.rook_moves(pos, piece, color, 3));

        moves.append(&mut self.bishop_moves(pos, piece, color, 0));
        moves.append(&mut self.bishop_moves(pos, piece, color, 1));
        moves.append(&mut self.bishop_moves(pos, piece, color, 2));
        moves.append(&mut self.bishop_moves(pos, piece, color, 3));

        return moves;
    }

    fn available_pawn_moves(&self, piece: &Piece, color: &Color, pos: &Position) -> Vec<((Position, Piece, Color), (Option<Piece>, Position))> {
        // TODO: Add ein peasant
        // TODO: Add promotion
        let mut moves: Vec<((Position, Piece, Color), (Option<Piece>, Position))> = vec![];
        let is_white = *color == Color::White;

        // Do single move
        let row_single = if is_white {pos.row - 1} else {pos.row + 1};
        let move_position = Position::new(pos.column, row_single);
        match self.get_piece_from_position(&move_position) {
            None => {
                let piece_info = (pos.clone(), piece.clone(), color.clone());
                let position_information = (None, move_position);
                moves.append(&mut Self::create_entry_from_position(piece_info, position_information))
            }
            Some((p, c)) => println!("piece: {}, color: {}, at position {}", p, c, pos.clone())
        }

        if pos.column > 0 {
            let move_position = Position::new(pos.column - 1,row_single );
            match self.get_piece_from_position(&move_position) {
                None => (),
                Some((captured_piece, captured_color)) => {
                    let has_same_color = captured_color != *color;
                    if has_same_color {
                        let piece_info = (pos.clone(), piece.clone(), color.clone());
                        let position_information = (Some(captured_piece), move_position);
                        moves.append(&mut Self::create_entry_from_position(piece_info, position_information))
                    }
                }
            }
        }

        if pos.column < 7 {
            let move_position = Position::new(pos.column + 1, row_single );
            match self.get_piece_from_position(&move_position) {
                None => (),
                Some((captured_piece, captured_color)) => {
                    let has_same_color = captured_color!= *color;
                    if has_same_color {
                        let piece_info = (pos.clone(), piece.clone(), color.clone());
                        let position_information = (Some(captured_piece), move_position);
                        moves.append(&mut Self::create_entry_from_position(piece_info, position_information))
                    }
                }
            }
        }

        if (is_white && pos.row == 6) || (!is_white && pos.row == 1) {
            let row_double = if is_white {pos.row - 2} else {pos.row + 2};
            let move_position = Position::new(pos.column,row_double );

            match self.get_piece_from_position(&move_position) {
                None => {
                    let piece_info = (pos.clone(), piece.clone(), color.clone());
                    let position_information = (None, move_position);
                    moves.append(&mut Self::create_entry_from_position(piece_info, position_information))
                },
                Some(_) => ()
            }
        }

        return moves;
    }
}