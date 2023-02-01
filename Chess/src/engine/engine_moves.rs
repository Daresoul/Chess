pub mod engine_moves {
    use crate::chess::chess_move::chess_move::ChessMove;

    #[derive(Clone)]
    pub struct EngineMoves {
        pub(crate) moves: Vec<ChessMove>
    }

    impl EngineMoves {
        pub fn add_move(&mut self, chess_move: ChessMove) {
            self.moves.append(&mut vec![chess_move.clone()])
        }

        pub fn print_engine_move(&self) {
            println!("[");

            for x in self.moves.iter().rev() {
                println!("{}", x)
            }

            println!("]")
        }

        pub fn print_list_of_engine_moves(engine_moves: &Vec<EngineMoves>) {
            for i in engine_moves {
                i.print_engine_move()
            }
        }
    }
}