/*use std::borrow::Borrow;
use std::fmt::Display;
use std::io::{stdin, stdout, Write};
use std::vec;
use array2d::Array2D;
pub use crate::chess::color::color;
pub use crate::chess::game::Game;
pub use crate::chess::piece::piece;
pub use crate::chess::position::position;
pub use crate::chess::image_struct::Images;
use crate::chess::game::log;*/

pub(crate) mod piece;
pub(crate) mod color;
pub(crate) mod position;
mod utils;
pub(crate) mod game;
mod image_struct;


// 1: pawn
// 2: bishop
// 3: knight
// 4: rook
// 5: queen
// 6: king

//const LOW_3_BITS_MASK: u8 = 0b0000_0111;
//const FOURTH_BIT_BITS_MASK: u8 = 0b0000_1000;

/*pub fn game_loop() -> () {
    // "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR"
    let mut game = create_board_from_string("k7/3Q3R/8/8/8/8/8/K7", 0);
    print_board(game.clone());
    let available_moves = get_all_turn_available_moves(game.clone());
    print_available_moves(available_moves);

    loop // TODO: make a game is over function
    {
        println!("Which piece to move: ");

        let x1 = read_line();
        if x1 == 8 { continue }

        let y1 = read_line();
        if y1 == 8 { continue }

        let x2 = read_line();
        if x2 == 8 { continue }

        let y2 = read_line();
        if y2 == 8 { continue }

        game = try_move_piece(game.clone(), position {column: x1, row: y1}, position { column: x2, row: y2 });
        print_board(game.clone());
        let available_moves = get_all_turn_available_moves(game.clone());
        print_available_moves(available_moves);
    }

    return ()
}


pub fn read_line() -> usize {
    let mut s = String::new();

    let _= stdout().flush();

    stdin().read_line(&mut s).expect("Not inputtet correctly.");

    if let Some('\n') = s.chars().next_back() {
        s.pop();
    }

    if let Some('\r') = s.chars().next_back() {
        s.pop();
    }

    let mut result: usize = 8;
    match s.parse::<usize>() {
        Ok(res) => result = res,
        Err(err) => println!("couldnt parse the given position with err: {}", err)
    }

    return result
}*/

/*pub fn begin() -> Game {
    return Game {
        board: Array2D::filled_with(0, 8, 8),
        turn: 0,
        log: game::log::new()
    }
}

pub fn print_board(game: Game) {
    for y in 0..game.board.num_rows() {
        print!("{} |", y);
        for x in 0..game.board.num_columns() {
            match game.board.get(y, x) {
                None => println!("failed..."),
                Some(t) => {
                    if *t > 9 {
                        print!("  {:?} ", t)
                    }
                    else {
                        print!("  {:?}  ", t)
                    }
                }
            }
        }
        print!("\n")
    }*/

    /*let letters = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'];

    print!("   ");
    for i in letters.iter() {
        print!("  {}  ", i)
    }
    print!("\n")*/

    /*print!("   ");
    for _ in 0..8 {
        print!("¯¯¯¯¯")
    }

    print!("\n   ");
    for i in 0..8 {
        print!("  {}  ", (i))
    }
    print!("\n")
}

pub fn print_available_moves(available_moves: Vec<((position, piece, color), (Option<piece>, position))>) {
    println!("[");
    for ((piece_current_position, piece, color), (future_position_piece, future_position)) in available_moves.iter() {

        match future_position_piece {
            None => print!("({}, {}, {}) ==> (None ,{})\n", piece_current_position, piece, color, future_position),
            Some(piece_at_position) => print!("({}, {}, {}) ==> ({} ,{})\n", piece_current_position, piece, color, piece_at_position, future_position),
        }

    }
    println!("]");
}

pub fn available_moves_to_string(available_moves: &Vec<((position, piece, color), (Option<piece>, position))>) -> String {
    let mut string = String::new();
    string.push_str("[\n");
    for ((piece_current_position, piece, color), (future_position_piece, future_position)) in available_moves.iter() {

        match future_position_piece {
            None => {
                string.push_str("(");
                string.push_str(&piece_current_position.to_string());
                string.push_str(", ");
                string.push_str(&piece.to_string());
                string.push_str(", ");
                string.push_str(&color.to_string());
                string.push_str(") ==> (None, ");
                string.push_str(&future_position.to_string());
                string.push_str(")\n");
            },
            Some(piece_at_position) => {
                string.push_str("(");
                string.push_str(&piece_current_position.to_string());
                string.push_str(", ");
                string.push_str(&piece.to_string());
                string.push_str(", ");
                string.push_str(&color.to_string());
                string.push_str(") ==> (");
                string.push_str(&piece_at_position.to_string());
                string.push_str(", ");
                string.push_str(&future_position.to_string());
                string.push_str(")\n");
            },
        }

    }
    string.push_str("]\n");
    return string;
}*/

/*fn is_checkmate(game: Game) -> bool {
    let mut game_copy = game.clone();
    let turn = color::to_opposite(get_turn(game_copy.clone()));

    for ((position, _, piece_color),(_, enemy_position)) in get_all_turn_available_moves(game.clone()) {
        if piece_color == turn {
            game_copy = move_piece(game_copy.clone(), turn.clone(), position.clone(), enemy_position.clone());

            let checks = is_check(game_copy.clone());

            let mut same_colored_check = false;
            for check_color in checks.iter() {
                if *check_color == turn {
                    same_colored_check = true;
                }
            }

            if !same_colored_check {
                return false;
            }
        }
    }

    return true
}

fn is_check(game: Game) -> Vec<color> {
    let all_moves = get_all_available_moves(game.clone());
    let mut checks: Vec<color> = vec![];

    for ((pos, piece, color), (option_piece, position)) in all_moves {
        match option_piece
        {
            None => continue,
            Some(captured_piece) => {
                if captured_piece == piece::King {
                    checks.append(&mut vec![color::to_opposite(color)])
                }
            }
        }
    }
    return checks
}

pub fn get_all_turn_available_moves(game: Game) -> Vec<((position, piece, color), (Option<piece>, position))> {
    let available_moves = get_all_available_moves(game.clone());
    let turn = get_turn(game.clone());
    let mut moves = vec![];

    for ((pos, piece, color), (option_piece, position)) in available_moves {
        if turn == color {
            let mut vec = vec![((pos, piece, color), (option_piece, position))];
            moves.append(&mut vec );
        }
    }

    return moves;
}

pub fn get_all_available_moves(game: Game) -> Vec<((position, piece, color), (Option<piece>, position))> {
    let mut moves = vec![];
    for x in 0..game.board.num_columns() {
        for y in 0..game.board.num_rows() {
            let position = position {column: x, row: y};
            moves.append( &mut get_available_moves(game.clone(), position))
        }
    }

    return moves
}

pub fn get_available_moves(game: Game, pos: position) -> Vec<((position, piece, color), (Option<piece>, position))> {
    match get_piece_from_position(game.board.clone(), pos.clone()) {
        None => vec![],
        Some((piece, color)) => {
            match piece {
                piece::Pawn => return available_pawn_moves(game, piece, color, pos),
                piece::Rook => return available_rook_moves(game, piece, color, pos),
                piece::Bishop => return available_bishop_moves(game, piece, color, pos),
                piece::Knight => return available_knight_moves(game, piece, color, pos),
                piece::Queen => return available_queen_moves(game, piece, color, pos),
                piece::King => return available_king_moves(game, piece, color, pos)
            }
        }
    }
}

pub fn get_piece_from_position(board: Array2D<u8>, pos: position) -> Option<(piece, color)> {
    match board.get(pos.row, pos.column) {
         None => panic!("Couldnt get the position: ({}, {}).", pos.column, pos.row),
        Some(piece_value) => {
            if *piece_value == 0 {
                return None
            }

            let piece_color = get_piece_color(*piece_value);
            let piece = get_piece(*piece_value);

            return Some((piece, piece_color))
        }
    }
}

fn get_piece(input: u8) -> piece {
    return piece::from_u8(input & LOW_3_BITS_MASK)
}

fn get_piece_color(input: u8) -> color {
    if input & FOURTH_BIT_BITS_MASK == 0 {
        return color::White
    }
    return color::Black
}

pub fn get_turn(game: Game) -> color {
    if game.turn % 2 == 0 {
        return color::White;
    }

    return color::Black
}

pub fn create_board_from_string(positions: &str, turn: u32) -> Game {
    let mut game = begin();
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
            'r' => match game.board.set(y, x, piece::Rook + color::Black) {
                Ok(_) => x += 1,
                Err(e) => println!("Couldnt set the board {e:?}")
            },
            'b' => match game.board.set(y, x, piece::Bishop + color::Black) {
                Ok(_) => x += 1,
                Err(e) => println!("Couldnt set the board {e:?}")
            },
            'n' => match game.board.set(y, x, piece::Knight + color::Black) {
                Ok(_) => x += 1,
                Err(e) => println!("Couldnt set the board {e:?}")
            },
            'k' => match game.board.set(y, x, piece::King + color::Black) {
                Ok(_) => x += 1,
                Err(e) => println!("Couldnt set the board {e:?}")
            },
            'q' => match game.board.set(y, x, piece::Queen + color::Black) {
                Ok(_) => x += 1,
                Err(e) => println!("Couldnt set the board {e:?}")
            },
            'p' => match game.board.set(y, x, piece::Pawn + color::Black) {
                Ok(_) => x += 1,
                Err(e) => println!("Couldnt set the board {e:?}"),
            }
            'R' => match game.board.set(y, x, piece::Rook + color::White) {
                Ok(_) => x += 1,
                Err(e) => println!("Couldnt set the board {e:?}")
            }, //for rooks
            'B' => match game.board.set(y, x, piece::Bishop + color::White) {
                Ok(_) => x += 1,
                Err(e) => println!("Couldnt set the board {e:?}")
            },
            'N' => match game.board.set(y, x, piece::Knight + color::White) {
                Ok(_) => x += 1,
                Err(e) => println!("Couldnt set the board {e:?}")
            },
            'K' => match game.board.set(y, x, piece::King + color::White) {
                Ok(_) => x += 1,
                Err(e) => println!("Couldnt set the board {e:?}")
            },
            'Q' => match game.board.set(y, x, piece::Queen + color::White) {
                Ok(_) => x += 1,
                Err(e) => println!("Couldnt set the board {e:?}")
            },
            'P' => match game.board.set(y, x, piece::Pawn + color::White) {
                Ok(_) => x += 1,
                Err(e) => println!("Couldnt set the board {e:?}")
            },
            _ => println!("piece not known.")
        }
    }
    return game
}

pub fn move_exists_in_list(available_moves: Vec<((position, piece, color), (Option<piece>, position))>, to: position) -> bool {
    for (_, (_ ,new_pos)) in available_moves {
        if new_pos.column == to.column && new_pos.row == to.row {
            return true;
        }
    }
    return false;
}

///
///
/// # Arguments
///
/// * `game`: Will be copied to make changes
/// * `color`: color of the turn
/// * `from`: position it comes from
/// * `to`: position it goes to
///
/// returns: bool
fn is_valid_move(game: Game, from: position, to: position) -> bool {
    let mut game_copy = game.clone();
    let turn = get_turn(game_copy.clone());

    game_copy = move_piece(game_copy.clone(), turn.clone(), from.clone(), to.clone());

    let checks = is_check(game_copy.clone());

    for check_color in checks.iter() {
        if *check_color == turn {
            return false;
        }
    }

    return true
}

fn move_piece(mut game: Game, turn: color, from: position, to: position) -> Game {
    match get_piece_from_position(game.board.clone(), from.clone()) {
        Some((piece, color)) => {
            if color == turn {
                match game.board.set(from.row, from.column, 0) {
                    Ok(_) => (),
                    Err(_) => panic!("board couldnt be set."),
                };
                match game.board.set(to.row, to.column, piece + color) {
                    Ok(_) => (),
                    Err(_) => panic!("board couldnt be set."),
                };
                return game;
            }

            panic!("Trying to move {} from {} of the color {}, to {} but it is {} turn.", piece, from, color, to, turn)
        }
        None => panic!("Couldnt find a piece at the position.")
    }
}

pub fn try_move_piece(game_non_mut: Game, from: position, to: position) -> Game {
    let mut game = game_non_mut.clone();
    let piece_available_moves = get_available_moves(game.clone(), from.clone());

    let can_move = move_exists_in_list(piece_available_moves, to.clone());

    let turn = get_turn(game.clone() );

    if can_move {
        if is_valid_move(game.clone(), from.clone(), to.clone())
        {
            let mut new_game = move_piece(game.clone(), turn, from, to);
            if is_checkmate(new_game.clone()) {
                println!("Is checkmate")
            }
            new_game.turn += 1;
            return new_game
        }
        else {
            println!("Couldnt move piece because of check.");
            return game;
        }
    }
    return game;
}

/***************************************************************************************************
**************************** _ __ ___     ___   __   __   ___   ___ ********************************
****************************| '_ ` _ \   / _ \  \ \ / /  / _ \ / __|********************************
****************************| | | | | | | (_) |  \ V /  |  __/ \__ \********************************
****************************|_| |_| |_|  \___/    \_/    \___| |___/********************************
****************************************************************************************************
***************************************************************************************************/ */

fn create_entry_from_position(piece_info: (position, piece, color), move_to_pos_and_piece: (Option<piece>, position)) -> Vec<((position, piece, color), (Option<piece>, position))> {
    return vec![(piece_info, move_to_pos_and_piece)]
}

fn available_pawn_moves(game: Game, piece: piece, color: color, pos: position) -> Vec<((position, piece, color), (Option<piece>, position))> {
    // TODO: Add ein peasant
    // TODO: Add promotion

    let mut moves: Vec<((position, piece, color), (Option<piece>, position))> = vec![];
    let is_white = color == color::White;

    // Do single move
    let row_single = if is_white {pos.row - 1} else {pos.row + 1};
    let move_position = position { column: pos.column, row: row_single };
    match get_piece_from_position(game.board.clone(), move_position.clone()) {
        None => {
            let piece_info = (pos.clone(), piece.clone(), color.clone());
            let position_information = (None, move_position);
            moves.append(&mut create_entry_from_position(piece_info, position_information))
        }
        Some((p, c)) => println!("piece: {}, color: {}, at position {}", p, c, pos.clone())
    }

    if pos.column > 0 {
        let move_position = position { column: pos.column - 1, row: row_single };
        match get_piece_from_position(game.board.clone(), move_position.clone()) {
            None => (),
            Some((captured_piece, captured_color)) => {
                    let has_same_color = (captured_color.clone()) != (color.clone());
                    if has_same_color {
                        let piece_info = (pos.clone(), piece.clone(), color.clone());
                        let position_information = (Some(captured_piece), move_position);
                        moves.append(&mut create_entry_from_position(piece_info, position_information))
                    }
            }
        }
    }

    if pos.column < 7 {
        let move_position = position { column: pos.column + 1, row: row_single };
        match get_piece_from_position(game.board.clone(), move_position.clone()) {
            None => (),
            Some((captured_piece, captured_color)) => {
                let has_same_color = (captured_color.clone()) != (color.clone());
                if has_same_color {
                    let piece_info = (pos.clone(), piece.clone(), color.clone());
                    let position_information = (Some(captured_piece), move_position);
                    moves.append(&mut create_entry_from_position(piece_info, position_information))
                }
            }
        }
    }

    if (is_white && pos.row == 6) || (!is_white && pos.row == 1) {
        let row_double = if is_white {pos.row - 2} else {pos.row + 2};
        let move_position = position { column: pos.column, row: row_double };

        match get_piece_from_position(game.board.clone(), move_position.clone()) {
            None => {
                let piece_info = (pos.clone(), piece.clone(), color.clone());
                let position_information = (None, move_position);
                moves.append(&mut create_entry_from_position(piece_info, position_information))
            },
            Some(_) => ()
        }
    }

    return moves;
}

fn rook_row_column(iter_value: usize, val: usize, direction: u8) -> position {
    if direction == 1 || direction == 3 {
        return position {column: iter_value, row: val}
    }
    else {
        return position {column: val, row: iter_value}
    }
}

fn create_iter(start: usize, end: usize, rev: bool) -> Vec<usize> {
    let mut moves: Vec<usize> = vec![];

    for i in start..end {
        moves.append(&mut vec![i])
    }

    if rev {
        moves.reverse();
    }

    return moves;
}

// Direction:
// 0 = up
// 1 = right
// 2 = down
// 3 = left
fn rook_moves(game: Game, pos: position, piece: piece, color: color, direction: u8) -> Vec<((position, piece, color), (Option<piece>, position))> {
    let mut moves: Vec<((position, piece, color), (Option<piece>, position))> = vec![];
    let iter =
        if direction == 0 {create_iter(0, pos.row.clone(), true)}
        else if direction == 1 {create_iter(pos.column.clone() + 1, 8, false)}
        else if direction == 2 {create_iter(pos.row.clone() + 1, 8, false) }
        else { create_iter(0, pos.column, true)};

    for i in iter.iter() {
        let value = if direction == 1 || direction == 3 { pos.row }
            else { pos.column };
        let move_position = rook_row_column(i.clone(), value, direction);
        match get_piece_from_position(game.board.clone(), move_position.clone()) {
            None => {
                let piece_info = (pos.clone(), piece.clone(), color.clone());
                let position_information = (None, move_position);
                moves.append(&mut create_entry_from_position(piece_info, position_information))
            },
            Some((captured_piece, capture_color)) => {
                if color != capture_color {
                    let piece_info = (pos.clone(), piece.clone(), color.clone());
                    let position_information = (Some(captured_piece), move_position);
                    moves.append(&mut create_entry_from_position(piece_info, position_information));
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
fn bishop_moves(game: Game, pos: position, piece: piece, color: color, direction: u8) -> Vec<((position, piece, color), (Option<piece>, position))> {
    let mut moves: Vec<((position, piece, color), (Option<piece>, position))> = vec![];
    let iter_vec = create_iter(0, pos.row.clone(), true);
    for (i, _) in iter_vec.iter().enumerate() {

        let move_position = if direction == 0 {
                if pos.column >= (i+1) && pos.row >= (i+1) {
                    position { column: pos.column - (i + 1), row: pos.row - (i + 1) }
                } else {
                    continue;
                }
            } else if direction == 1 {
                if pos.column < 8 - (i + 1) && pos.row >= (i+1) {
                    position { column: pos.column + (i + 1), row: pos.row - (i + 1) }
                } else {
                    continue;
                }
            } else if direction == 2 {
                if pos.column < 8 - (i + 1) && pos.row < 8 - (i + 1) {
                    position { column: pos.column + (i+1), row: pos.row + (i+1)}
                } else {
                    continue;
                }
            } else {
                if pos.column >= (i+1) && pos.row < 8 - (i + 1) {
                    position { column: pos.column - (i+1), row: pos.row + (i+1)}
                } else {
                    continue;
                }
            };
        match get_piece_from_position(game.board.clone(), move_position.clone()) {
            None => {
                let piece_info = (pos.clone(), piece.clone(), color.clone());
                let position_information = (None, move_position);
                moves.append(&mut create_entry_from_position(piece_info, position_information))
            },
            Some((captured_piece, capture_color)) => {
                if color != capture_color {
                    let piece_info = (pos.clone(), piece.clone(), color.clone());
                    let position_information = (Some(captured_piece), move_position);
                    moves.append(&mut create_entry_from_position(piece_info, position_information));
                }

                break;
            }
        }
    }

    return moves;
}

fn knight_moves(game: Game, pos: position, piece: piece, color: color) -> Vec<((position, piece, color), (Option<piece>, position))> {
    let mut moves: Vec<((position, piece, color), (Option<piece>, position))> = vec![];

    let knight_offsets: [(i8, i8, usize, usize); 8] = [
        (-1, -2, 1, 2), (1, -2, 1, 2), (2, -1, 2, 1), (2, 1, 2, 1),
        (1, 2, 1, 2), (-1, 2, 1, 2), (-2, 1, 2, 1), (-2, -1, 2, 1)
    ];

    for (x,y, x_usize, y_usize) in knight_offsets.iter() {

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

        let move_position = position {column: new_column, row: new_row};

        match get_piece_from_position(game.board.clone(), move_position.clone()) {
            None => {
                let piece_info = (pos.clone(), piece.clone(), color.clone());
                let position_information = (None, move_position);
                moves.append(&mut create_entry_from_position(piece_info, position_information))
            },
            Some((captured_piece, capture_color)) => {
                if color != capture_color {
                    let piece_info = (pos.clone(), piece.clone(), color.clone());
                    let position_information = (Some(captured_piece), move_position);
                    moves.append(&mut create_entry_from_position(piece_info, position_information));
                }
                else { continue }
            }
        }
    }

    return moves;
}

fn king_moves(game: Game, pos: position, piece: piece, color: color) -> Vec<((position, piece, color), (Option<piece>, position))> {
    let mut moves: Vec<((position, piece, color), (Option<piece>, position))> = vec![];

    let king_offset: [(i8, i8, usize, usize); 8] = [
        (0, -1, 0, 1), (1, -1, 1, 1), (1, 0, 1, 0), (1, 1, 1, 1),
        (0, 1, 0, 1), (-1, 1, 1, 1), (-1, 0, 1, 0), (-1, -1, 1, 1)
    ];

    for (x,y, x_usize, y_usize) in king_offset.iter() {
        let new_column =
            if *x > 0 {
                if pos.column > 7 - x_usize { continue }
                pos.column + x_usize
            } else {
                if pos.column < *x_usize { continue }
                pos.column - x_usize
            };

        let new_row =
            if *y > 0 {
                if pos.row > 7 - y_usize { continue }
                pos.row + y_usize
            } else {
                if pos.row < *y_usize { continue }
                pos.row - y_usize
            };

        let move_position = position { column: new_column, row: new_row };


        match get_piece_from_position(game.board.clone(), move_position.clone()) {
            None => {
                let piece_info = (pos.clone(), piece.clone(), color.clone());
                let position_information = (None, move_position);
                moves.append(&mut create_entry_from_position(piece_info, position_information))
            },
            Some((captured_piece, capture_color)) => {
                if color != capture_color {
                    let piece_info = (pos.clone(), piece.clone(), color.clone());
                    let position_information = (Some(captured_piece), move_position);
                    moves.append(&mut create_entry_from_position(piece_info, position_information));
                } else { continue }
            }
        }
    }

    return moves;
}

fn available_rook_moves(game: Game, piece: piece, color: color, pos: position) -> Vec<((position, piece, color), (Option<piece>, position))> {
    let mut moves: Vec<((position, piece, color), (Option<piece>, position))> = vec![];

    moves.append(&mut rook_moves(game.clone(), pos.clone(), piece.clone(), color.clone(), 0));
    moves.append(&mut rook_moves(game.clone(), pos.clone(), piece.clone(),color.clone(), 1));
    moves.append(&mut rook_moves(game.clone(), pos.clone(), piece.clone(),color.clone(), 2));
    moves.append(&mut rook_moves(game.clone(), pos.clone(), piece.clone(),color.clone(), 3));

    return moves;
}

fn available_bishop_moves(game: Game, piece: piece, color: color, pos: position) -> Vec<((position, piece, color), (Option<piece>, position))> {
    let mut moves: Vec<((position, piece, color), (Option<piece>, position))> = vec![];

    moves.append(&mut bishop_moves(game.clone(), pos.clone(), piece.clone(), color.clone(), 0));
    moves.append(&mut bishop_moves(game.clone(), pos.clone(), piece.clone(), color.clone(), 1));
    moves.append(&mut bishop_moves(game.clone(), pos.clone(), piece.clone(), color.clone(), 2));
    moves.append(&mut bishop_moves(game.clone(), pos.clone(), piece.clone(), color.clone(), 3));

    return moves;
}

fn available_knight_moves(game: Game, piece: piece, color: color, pos: position) -> Vec<((position, piece, color), (Option<piece>, position))> {
    let mut moves: Vec<((position, piece, color), (Option<piece>, position))> = vec![];

    moves.append(&mut knight_moves(game.clone(), pos.clone(), piece.clone(), color.clone()));

    return moves;
}

fn available_queen_moves(game: Game, piece: piece, color: color, pos: position) -> Vec<((position, piece, color), (Option<piece>, position))> {
    let mut moves: Vec<((position, piece, color), (Option<piece>, position))> = vec![];

    moves.append(&mut rook_moves(game.clone(), pos.clone(), piece.clone(), color.clone(), 0));
    moves.append(&mut rook_moves(game.clone(), pos.clone(), piece.clone(),color.clone(), 1));
    moves.append(&mut rook_moves(game.clone(), pos.clone(), piece.clone(),color.clone(), 2));
    moves.append(&mut rook_moves(game.clone(), pos.clone(), piece.clone(),color.clone(), 3));

    moves.append(&mut bishop_moves(game.clone(), pos.clone(), piece.clone(), color.clone(), 0));
    moves.append(&mut bishop_moves(game.clone(), pos.clone(), piece.clone(), color.clone(), 1));
    moves.append(&mut bishop_moves(game.clone(), pos.clone(), piece.clone(), color.clone(), 2));
    moves.append(&mut bishop_moves(game.clone(), pos.clone(), piece.clone(), color.clone(), 3));

    return moves;
}

fn available_king_moves(game: Game, piece: piece, color: color, pos: position) -> Vec<((position, piece, color), (Option<piece>, position))> {
    let mut moves: Vec<((position, piece, color), (Option<piece>, position))> = vec![];

    moves.append(&mut king_moves(game.clone(), pos.clone(), piece.clone(), color.clone()));

    return moves;
}*/