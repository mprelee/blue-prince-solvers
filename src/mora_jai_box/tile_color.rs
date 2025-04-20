
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum TileColor {
    Gray,
    Black,
    Red,
    Green,
    Yellow,
    Pink,
    Violet,
    Orange,
    White,
    Blue,
}
use TileColor::*;

#[repr(usize)]
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
enum TilePosition {
    TopLeft = 0,
    TopCenter = 1,
    TopRight = 2,
    MiddleLeft = 3,
    MiddleCenter = 4,
    MiddleRight = 5,
    BottomLeft = 6,
    BottomCenter = 7,
    BottomRight = 8,
}
use TilePosition::*;
impl TilePosition {
    const ALL: [TilePosition; 9] = [
        TopLeft,
        TopCenter,
        TopRight,
        MiddleLeft,
        MiddleCenter,
        MiddleRight,
        BottomLeft,
        BottomCenter,
        BottomRight,
    ];
    pub fn get_neighbors(&self) -> Vec<TilePosition> {
        match self {
            TopLeft => vec![TopCenter, MiddleLeft],
            TopCenter => vec![TopLeft, TopRight, MiddleCenter],
            TopRight => vec![TopCenter, MiddleRight],
            MiddleLeft => vec![TopLeft, MiddleCenter, BottomLeft],
            MiddleCenter => vec![TopCenter, MiddleLeft, MiddleRight, BottomCenter],
            MiddleRight => vec![TopRight, MiddleCenter, BottomRight],
            BottomLeft => vec![MiddleLeft, BottomCenter],
            BottomCenter => vec![MiddleCenter, BottomLeft, BottomRight],
            BottomRight => vec![MiddleRight, BottomCenter],
        }
    }
    pub fn count_neighbors(&self) -> usize {
        self.get_neighbors().len()
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct Tiles([TileColor; 9]);

impl Tiles {
    pub fn new(tiles: [TileColor; 9]) -> Self {
        Self(tiles)
    }

    pub fn get_color(&self, position: TilePosition) -> TileColor {
        self.0[position as usize]
    }

    fn set_color(&mut self, position: TilePosition, color: TileColor) {
        self.0[position as usize] = color;
    }

    fn swap(&mut self, pos1: TilePosition, pos2: TilePosition) {
        let newcolor1 = self.get_color(pos2);
        let newcolor2 = self.get_color(pos1);
        self.set_color(pos1, newcolor1);
        self.set_color(pos2, newcolor2);
    }

    // Gray — Tile acts as an empty space and has no function.
    fn _gray_press(&self) { /* do nothing */ }

    // Black — Moves all of the tiles in the row one to the right.
    fn black_press(&mut self, pos: TilePosition) {
        let mut next = self.clone();
        match pos {
            TopLeft | TopCenter | TopRight => {
                next.set_color(TopLeft, self.get_color(TopRight));
                next.set_color(TopCenter, self.get_color(TopLeft));
                next.set_color(TopRight, self.get_color(TopCenter));
            }
            MiddleLeft | MiddleCenter | MiddleRight => {
                next.set_color(MiddleLeft, self.get_color(MiddleRight));
                next.set_color(MiddleCenter, self.get_color(MiddleLeft));
                next.set_color(MiddleRight, self.get_color(MiddleCenter));
            }
            BottomLeft | BottomCenter | BottomRight => {
                next.set_color(BottomLeft, self.get_color(BottomRight));
                next.set_color(BottomCenter, self.get_color(BottomLeft));
                next.set_color(BottomRight, self.get_color(BottomCenter));
            }
        }
        *self = next;
    }
    // Red — Turns all white tiles black and all black tiles red.
    fn red_press(&mut self) {
        let mut next = self.clone();
        for pos in TilePosition::ALL {
            if self.get_color(pos) == White {
                next.set_color(pos, Black);
            } else if self.get_color(pos) == Black {
                next.set_color(pos, Red);
            }
        }
        *self = next;
    }
    // Green — Tile swaps positions with a tile in the opposite position.
    fn green_press(&mut self, pos: TilePosition) {
        let swap_pos = match pos {
            TopLeft => BottomRight,
            TopCenter => BottomCenter,
            TopRight => BottomLeft,
            MiddleLeft => MiddleRight,
            MiddleCenter => MiddleCenter,
            MiddleRight => MiddleLeft,
            BottomLeft => TopRight,
            BottomCenter => TopCenter,
            BottomRight => TopLeft,
        };
        self.swap(pos, swap_pos);
    }
    // Yellow — Tile moves up one position (swaps with the tile above it).
    fn yellow_press(&mut self, pos: TilePosition) {
        let swap_pos = match pos {
            MiddleLeft => TopLeft,
            MiddleCenter => TopCenter,
            MiddleRight => TopRight,
            BottomLeft => MiddleLeft,
            BottomCenter => MiddleCenter,
            BottomRight => MiddleRight,
            _ => pos,
        };
        self.swap(pos, swap_pos);
    }
    // Orange — Tile changes colors to match the majority of its adjacent tiles. 
    // If there adjacent tiles are evenly split, the tile will not change colors.
    fn orange_press(&self) -> Self {
        todo!()
    }
    // White — Tile expands outwards if there are any adjacent gray tiles. 
    // If there are no adjacent gray tiles, the white tile will disappear.
    fn white_press(&mut self, pos: TilePosition) {
        for neighbor in pos.get_neighbors() {
            if self.get_color(neighbor) == Gray {
                self.set_color(pos, White);
            }
        }
        self.set_color(pos, Gray);
    }
    // Blue — Tile will copy the ability of the tile in the middle of the 3x3 grid.
    fn blue_press(&mut self, pos: TilePosition) {
        self.set_color(pos, self.get_color(MiddleCenter));
    }
    // Violet — Tile moves down one position.
    fn violet_press(&mut self, pos: TilePosition) {
        let swap_pos = match pos {
            TopLeft => MiddleLeft,
            TopCenter => MiddleCenter,
            TopRight => MiddleRight,
            MiddleLeft => BottomLeft,
            MiddleCenter => BottomCenter,
            MiddleRight => BottomRight,
            _ => pos,
        };
        self.swap(pos, swap_pos);
    }

}
