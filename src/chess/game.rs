use array2d::Array2D;
use crate::chess::Position;

#[derive(Clone)]
pub struct Game {
    pub board: Array2D<u8>,
    pub turn: u32,
    pub log: Vec<(Position, Position)>
}