//! Checker board
/// Color of the checker piece.
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum PieceColor {
    White,
    Black,
}

/// Individual game piece.
#[derive(Debug, Copy, Clone)]
pub struct Piece {
    pub color: PieceColor,
    pub crowned: bool,
}

impl Piece {
    pub fn new(color: PieceColor) -> Self {
        Self {
            color,
            ..Self::default()
        }
    }
    pub fn white_piece() -> Self {
        Self::new(PieceColor::White)
    }
    pub fn black_piece() -> Self {
        Self::new(PieceColor::Black)
    }
    pub fn crowned(p: Self) -> Self {
        Self { crowned: true, ..p }
    }
    pub fn should_crowned(self, to: Coordinate) -> bool {
        let y = to.1;
        match self.color {
            PieceColor::Black => y == 0,
            PieceColor::White => y == 7,
        }
    }
    pub fn is_black(self) -> bool {
        self.color == PieceColor::Black
    }
    pub fn is_white(self) -> bool {
        self.color == PieceColor::White
    }
    pub fn is_crowned(self) -> bool {
        self.crowned
    }
}

impl Default for Piece {
    fn default() -> Self {
        Self {
            color: PieceColor::White,
            crowned: false,
        }
    }
}

/// Game board coordinate.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Coordinate(pub usize, pub usize);

impl Coordinate {
    pub fn on_board(self) -> bool {
        let Coordinate(x, y) = self;
        x <= 7 && y <= 7
    }
    pub fn jump_targets_from(&self) -> impl Iterator<Item = Coordinate> {
        let mut jumps = Vec::new();
        let Coordinate(x, y) = *self;
        if y >= 2 {
            jumps.push(Coordinate(x + 2, y - 2));
        }
        jumps.push(Coordinate(x + 2, y + 2));
        if x >= 2 && y >= 2 {
            jumps.push(Coordinate(x - 2, y - 2));
        }
        if x >= 2 {
            jumps.push(Coordinate(x - 2, y + 2));
        }
        jumps.into_iter()
    }
    pub fn move_targets_from(&self) -> impl Iterator<Item = Coordinate> {
        let mut moves = Vec::new();
        let Coordinate(x, y) = *self;
        if x >= 1 {
            moves.push(Coordinate(x - 1, y + 1));
        }
        moves.push(Coordinate(x + 1, y + 1));
        if y >= 1 {
            moves.push(Coordinate(x + 1, y - 1));
        }
        if x >= 1 && y >= 1 {
            moves.push(Coordinate(x - 1, y - 1));
        }
        moves.into_iter()
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Move {
    pub from: Coordinate,
    pub to: Coordinate,
}

impl Move {
    pub fn new(from: (usize, usize), to: (usize, usize)) -> Self {
        Self {
            from: Coordinate(from.0, from.1),
            to: Coordinate(to.0, to.1),
        }
    }
    pub fn midpiece_coordinate(&self) -> Option<Coordinate> {
        let Coordinate(fx, fy) = self.from;
        let Coordinate(tx, ty) = self.to;
        let x = ((fx as isize - tx as isize).abs() / 2) as usize;
        let y = ((fy as isize - ty as isize).abs() / 2) as usize;
        Some(Coordinate(x, y))
    }
}
