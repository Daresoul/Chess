pub mod engine {
    use std::path::Component::RootDir;
    use crate::chess::chess_move::chess_move::ChessMove;
    use crate::chess::game::game::Game;
    use crate::engine::engine_moves::engine_moves::EngineMoves;
    use crate::engine::tree::game_tree::GameTree;
    use crate::engine::tree::game_tree::GameTree::Leaf;

    pub fn tree_init(game: Game, depth: usize) -> GameTree {
        if depth < 1 {
            panic!("Depth have to start at 1.")
        }

        let all_moves = game.get_all_turn_available_moves();

        let mut root = GameTree::init_tree(game.clone());

        for chess_move in all_moves.iter() {
            let mut new_game = game.clone();
            let mut after_move = new_game.move_piece(&new_game.get_turn(), &chess_move).unwrap();
            after_move.turn += 1;
            let x = generate_tree(&after_move, chess_move, depth - 1);
            root = GameTree::append(root, x);
        }

        return root;
    }


    pub fn generate_tree(game: &Game, move_to_here: &ChessMove, depth: usize) -> GameTree {
        let all_moves = game.get_all_turn_available_moves();

        if all_moves.len() == 0 || depth == 0 {
            let game_tree = GameTree::new_leaf(game.clone(),move_to_here.clone());
            return game_tree;
        }

        let mut game_tree = GameTree::new_tree(game.clone(), move_to_here.clone());

        for chess_move in all_moves.iter() {
            if Game::is_valid_move(game.clone(), chess_move) {
                let mut new_game = game.clone();
                let mut after_move = new_game.move_piece(&new_game.get_turn(), &chess_move).unwrap();
                after_move.turn += 1;
                game_tree = GameTree::append(game_tree, generate_tree(&after_move, chess_move, depth-1));
            }
        }

        return game_tree;
    }

    pub fn show_all_moves<'a>(game_tree: &'a GameTree) -> Vec<EngineMoves> {
        match game_tree {
            GameTree::Leaf(game, chess_move_option, value) => {
                match chess_move_option {
                    Some(chess_move) => {
                        let engine_moves = EngineMoves {
                            moves: vec![chess_move.clone()]
                        };
                        return vec![engine_moves]
                    },
                    None => panic!("Leafes should always have a move")
                }
            },
            GameTree::Tree(game, chess_move, alpha, beta, children) => {
                match chess_move {
                    Some(chess_move) => {
                        println!("Sub tree");
                        let mut possibilities = vec![];

                        for tree in children {
                            let mut engine_moves = show_all_moves(tree);
                            for mut x in engine_moves {
                                x.add_move(chess_move.clone());
                                possibilities.append(&mut vec![x])
                            }
                        }

                        return possibilities
                    },
                    None => {
                        // For root
                        println!("Root");
                        let mut from_root = vec![];
                        for i in children {
                            let mut engine_move = show_all_moves(i);
                            from_root.append(&mut engine_move)
                        }
                        return from_root;
                    }
                }
            }
        }
    }

    pub fn count_nodes(game_tree: &GameTree) -> usize {
        match game_tree {
            GameTree::Leaf(x,y,z) => return 1,
            GameTree::Tree(x,y,z,t,f) => {
                let mut sum = 0;

                for tree in f.iter() {
                    sum += count_nodes(tree)
                }

                return 1 + sum;
            }
        }
    }

    pub fn count_leaves(game_tree: &GameTree) -> usize {
        match game_tree {
            GameTree::Leaf(x,y,z) => return 1,
            GameTree::Tree(x,y,z,t,f) => {
                let mut sum = 0;

                for tree in f.iter() {
                    sum += count_leaves(tree)
                }

                return sum
            }
        }
    }
}