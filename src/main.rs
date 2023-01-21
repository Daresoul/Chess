extern crate core;

mod chess;

fn main() {
    chess::game_loop();
    //let game = struct::create_board_from_string("r1bk3r/p2pBpNp/n4n2/1p1NP2P/6P1/3P4/P1P1K3/q5b1", 10);
    /*let game = chess::create_board_from_string("rnbqkbnr/pppppppp/8/3R4/8/8/PPPPPPPP/1NBQKBNR", 0);
    chess::print_board(game.clone());
    let position = chess::Position {
        column: 4,
        row: 7,
    };

    let all_available_moves = chess::get_all_available_moves(game.clone());
    println!("len: {}", all_available_moves.len());
    chess::print_available_moves(all_available_moves);
    let game = chess::move_piece(game.clone(), Position {column: 3, row: 3}, Position {column: 3, row: 1});
    chess::print_board(game.clone());*/
    /*let available_moves = chess::get_available_moves(game.clone(), position.clone());
    chess::print_available_moves(available_moves);*/

    /*let game = struct::move_piece(game.clone(), position.clone(), position2.clone());
    struct::print_board(game.clone());*/
}
