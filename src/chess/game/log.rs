pub mod log {
    use crate::chess::game::r#move::chess_move::ChessMove;
    use crate::chess::position::position::Position;

    #[derive(Clone)]
    pub struct Log {
        log: Vec<ChessMove>
    }

    impl Log {
        pub fn new() -> Log {
            return Log {
                log: vec![]
            }
        }

        pub fn append(&mut self, mut append: Vec<ChessMove>) {
            self.log.append(&mut append)
        }

        pub fn get_last_move(&self) -> Option<ChessMove> {
            if self.log.len() > 1 {
                match self.log.get(self.log.len() - 1) {
                    None => panic!("Couldnt get the last log..."),
                    Some(t) => return Some(t.clone())
                }
            }

            return None
        }

        pub fn piece_has_moved_from_starting_square(&self, pos: &Position) -> bool {
            for chess_move in self.log.iter() {
                if chess_move.from == *pos {
                    return true;
                }
            }

            return false;
        }
    }
}