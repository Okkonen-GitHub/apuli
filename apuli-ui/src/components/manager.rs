



#[derive(Debug)]
pub enum TileState {
    Gray,
    Blue,
    Orange,
}
#[derive(Debug)]
pub struct Tile {
    pub state: TileState,
    pub positions: Vec<usize>,
}

