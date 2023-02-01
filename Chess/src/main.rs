extern crate core;

use std::env;
use crate::chess::game::game::Game;
use crate::engine::engine_moves::engine_moves::EngineMoves;
use crate::engine::tree::game_tree::GameTree;

mod main_graphics;
mod chess;
mod engine;

fn run_engine() {
    let mut game = Game::default(); //Game::create_board_from_string("2ppp3/2pqp3/2ppp4/8/8/2PPP3/2PQP3/2PPP3", 0);
    println!("Game made");
    let tree = engine::engine::engine::tree_init(game, 3);

    let moves = engine::engine::engine::show_all_moves(&tree);
    engine::engine_moves::engine_moves::EngineMoves::print_list_of_engine_moves(&moves);
    println!("moves len: {}", moves.len());
    println!("Tree made, counting");
    let count = engine::engine::engine::count_nodes(&tree);
    println!("nodes: {}", count);
    let leaves = engine::engine::engine::count_leaves(&tree);
    println!("leaves: {}", leaves);
    ()
}

fn main() {
    let use_graphics = match env::var("graphics") {
        Ok(t) => t,
        Err(err) => "0".to_string()
     };

    if use_graphics == "1" {
        main_graphics::run_graphics()
    }
    else {
        run_engine()
    }
}