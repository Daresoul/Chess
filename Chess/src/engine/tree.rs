pub mod game_tree {
    use crate::chess::chess_move::chess_move::ChessMove;
    use crate::chess::game::game::Game;

    #[derive(Clone)]
    pub enum GameTree {
    //  Tree(game state, alpha, beta, other moves)
        Tree(Game, Option<ChessMove>, i32, i32, Vec<GameTree>),
    //  Leaf(game state, leaf value)
        Leaf(Game, Option<ChessMove>, i32)
    }

    impl GameTree {
        pub fn init_tree(game: Game) -> GameTree {
            return GameTree::Tree(game, None,i32::MAX, i32::MIN, vec![])
        }

        pub fn new_tree(game: Game, chess_move: ChessMove) -> GameTree {
            return GameTree::Tree(game, Some(chess_move),i32::MAX, i32::MIN, vec![])
        }

        pub fn new_leaf(game: Game, chess_move: ChessMove) -> GameTree {
            return GameTree::Leaf(game, Some(chess_move),0)
        }

        pub fn append(parent: GameTree, game_tree: GameTree) -> GameTree {
            match parent {
                GameTree::Tree(game, chess_move, alpha, beta, trees) => {
                    let mut new_trees = trees.clone();
                    new_trees.append(&mut vec![game_tree.clone()]);
                    return GameTree::Tree(game, chess_move, alpha, beta, new_trees)
                },
                GameTree::Leaf(game, chess_move, eval) => {
                    return GameTree::Leaf(game, chess_move, eval)
                }
            }
        }
    }
}