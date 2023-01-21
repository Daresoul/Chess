use array2d::Array2D;

#[derive(Clone)]
pub struct Game {
    pub board: Array2D<u8>,
    pub turn: u32
}