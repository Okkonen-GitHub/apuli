use crate::cprint;


#[derive(PartialEq, Clone)]
pub struct TileManager {
    pub tiles: Vec<Tile>
}


#[derive(Debug, PartialEq, Clone)]
pub enum TileState {
    Black,
    Gray,
    Blue,
    Orange,
}
#[derive(Debug, PartialEq, Clone)]
pub struct Tile {
    pub state: TileState,
    pub position: usize,
    pub character: char,
}


impl TileManager {

    pub fn update_tile(&mut self, mut tile: Tile) -> () {
        if tile.character == ' ' {return;} // do nothing if the character is blank
        
        for (index, managed_tile) in self.tiles.clone().iter().enumerate() {
            if managed_tile.position == tile.position && managed_tile.character == tile.character {
                self.tiles.remove(index);
            }
        }

        match tile.state {
            TileState::Black => { tile.state = TileState::Gray }
            TileState::Gray => { tile.state = TileState::Blue },
            TileState::Blue => { tile.state = TileState::Orange },
            TileState::Orange => { tile.state = TileState::Black },
        }
        if tile.state != TileState::Black { // no black tiles to the list?
            self.tiles.push(tile);
        } 
    }

    pub fn new() -> Self {
        Self { tiles: vec![] }
    }
    
}
