//! Game engine
use super::board::{Coordinate, Move, Piece, PieceColor};

pub struct Engine {
    board: [[Option<Piece>; 8]; 8],
    current_turn: PieceColor,
    move_count: usize,
}

pub struct MoveResult {
    pub mv: Move,
    pub crowned: bool,
}

impl Engine {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn move_piece(&mut self, mv: &Move) -> Result<MoveResult, ()> {
        let legal_moves = self.legal_moves();
        if !legal_moves.contains(mv) {
            return Err(())
        }
        let Coordinate(fx, fy) = mv.from;
        let Coordinate(tx, ty) = mv.to;
        let piece = self.board[fx][fy].unwrap();
        let midpiece_coordinate = mv.midpiece_coordinate();
        if let Some(Coordinate(x, y)) = midpiece_coordinate {
            self.board[x][y] = None;
        }
        self.board[tx][ty] = Some(piece);
        self.board[fx][fy] = None;

        let crowned = if piece.should_crowned(mv.to) {
            self.crown_piece(mv.to);
            true
        } else {
            false
        };
        self.advance_turn();
        Ok(MoveResult {
            mv: mv.clone(),
            crowned,
        })
    }
    fn crown_piece(&mut self, to: Coordinate) {
        let Coordinate(x, y) = to;
        if let Some(piece) = self.board[x][y] {
            self.board[x][y] = Some(Piece::crowned(piece));
        }
    }
    fn advance_turn(&mut self) {
        self.current_turn = match self.current_turn {
            PieceColor::Black => PieceColor::White,
            PieceColor::White => PieceColor::Black,
        };
        self.move_count += 1;
    }
    fn legal_moves(&self) -> Vec<Move> {
        let mut moves: Vec<Move> = Vec::new();
        for col in 0..8 {
            for row in 0..8 {
                if let Some(piece) = self.board[col][row] {
                    if piece.color == self.current_turn {
                        let loc = Coordinate(col, row);
                        let mut vmoves = self.valid_moves_from(loc);
                        moves.append(&mut vmoves);
                    }
                }
            }
        }
        moves
    }
    fn valid_moves_from(&self, loc: Coordinate) -> Vec<Move> {
        let Coordinate(x, y) = loc;
        match self.board[x][y] {
            None => Vec::new(),
            Some(p) => {
                let mut jumps = loc
                    .jump_targets_from()
                    .filter(|t| self.valid_jump(&p, &loc, &t))
                    .map(|ref t| Move {
                        from: loc.clone(),
                        to: t.clone(),
                    })
                    .collect::<Vec<Move>>();
                let mut moves = loc
                    .move_targets_from()
                    .filter(|t| self.valid_move(&p, &loc, &t))
                    .map(|ref t| Move {
                        from: loc.clone(),
                        to: t.clone(),
                    })
                    .collect::<Vec<Move>>();
                jumps.append(&mut moves);
                jumps
            }
        }
    }
    fn valid_jump(&self, _p: &Piece, _from: &Coordinate, _to: &Coordinate) -> bool {
        // XXX
        true
    }
    fn valid_move(&self, _p: &Piece, _from: &Coordinate, _to: &Coordinate) -> bool {
        // XXX
        true
    }
    fn initialize_board(&mut self) {
        // white pieces
        [1, 3, 5, 7, 0, 2, 4, 6, 1, 3, 5, 7]
            .iter()
            .zip([0, 0, 0, 0, 1, 1, 1, 1, 2, 2, 2, 2].iter())
            .for_each(|(x, y)| {
                self.board[*x][*y] = Some(Piece::white_piece());
            });
        // black pieces
        [0, 2, 4, 6, 1, 3, 5, 7, 0, 2, 4, 6]
            .iter()
            .zip([5, 5, 5, 5, 6, 6, 6, 6, 7, 7, 7, 7].iter())
            .for_each(|(x, y)| {
                self.board[*x][*y] = Some(Piece::black_piece());
            });
    }
    fn is_black(&self, c: Coordinate) -> bool {
        if !c.on_board() {
            false
        } else {
            let Coordinate(x, y) = c;
            match self.board[x][y] {
                None => false,
                Some(piece) => piece.is_black(),
            }
        }
    }
    fn is_white(&self, c: Coordinate) -> bool {
        if !c.on_board() {
            false
        } else {
            let Coordinate(x, y) = c;
            match self.board[x][y] {
                None => false,
                Some(piece) => piece.is_white(),
            }
        }
    }
    fn is_empty(&self, c: Coordinate) -> bool {
        if !c.on_board() {
            false
        } else {
            let Coordinate(x, y) = c;
            match self.board[x][y] {
                None => true,
                _ => false,
            }
        }
    }
}

impl Default for Engine {
    fn default() -> Self {
        let mut engine = Self {
            board: [[None; 8]; 8],
            current_turn: PieceColor::Black,
            move_count: 0_usize,
        };
        engine.initialize_board();
        engine
    }
}

#[cfg(test)]
mod tests {
    use super::{Coordinate, Engine};
    #[test]
    fn initial_board() {
        let engine = Engine::new();
        // first row
        [0, 1, 2, 3, 4, 5, 6, 7]
            .iter()
            .zip([0, 0, 0, 0, 0, 0, 0].iter())
            .map(|(x, y)| Coordinate(*x, *y))
            .for_each(|c| {
                if c.0 % 2 == 1 {
                    assert!(engine.is_white(c));
                } else {
                    assert!(engine.is_empty(c));
                }
            });
        // second row
        [0, 1, 2, 3, 4, 5, 6, 7]
            .iter()
            .zip([1, 1, 1, 1, 1, 1, 1].iter())
            .map(|(x, y)| Coordinate(*x, *y))
            .for_each(|c| {
                if c.0 % 2 == 0 {
                    assert!(engine.is_white(c));
                } else {
                    assert!(engine.is_empty(c));
                }
            });
        // third row
        [0, 1, 2, 3, 4, 5, 6, 7]
            .iter()
            .zip([2, 2, 2, 2, 2, 2, 2].iter())
            .map(|(x, y)| Coordinate(*x, *y))
            .for_each(|c| {
                if c.0 % 2 == 1 {
                    assert!(engine.is_white(c));
                } else {
                    assert!(engine.is_empty(c));
                }
            });
        // forth and fifth rows
        [0, 1, 2, 3, 4, 5, 6, 7, 0, 1, 2, 3, 4, 5, 6, 7]
            .iter()
            .zip([3, 3, 3, 3, 3, 3, 3, 4, 4, 4, 4, 4, 4, 4].iter())
            .map(|(x, y)| Coordinate(*x, *y))
            .for_each(|c| assert!(engine.is_empty(c)));
        // sixth row
        [0, 1, 2, 3, 4, 5, 6, 7]
            .iter()
            .zip([5, 5, 5, 5, 5, 5, 5].iter())
            .map(|(x, y)| Coordinate(*x, *y))
            .for_each(|c| {
                if c.0 % 2 == 0 {
                    assert!(engine.is_black(c));
                } else {
                    assert!(engine.is_empty(c));
                }
            });
        // seventh row
        [0, 1, 2, 3, 4, 5, 6, 7]
            .iter()
            .zip([6, 6, 6, 6, 6, 6, 6].iter())
            .map(|(x, y)| Coordinate(*x, *y))
            .for_each(|c| {
                if c.0 % 2 == 1 {
                    assert!(engine.is_black(c));
                } else {
                    assert!(engine.is_empty(c));
                }
            });
        // eighth row
        [0, 1, 2, 3, 4, 5, 6, 7]
            .iter()
            .zip([7, 7, 7, 7, 7, 7, 7].iter())
            .map(|(x, y)| Coordinate(*x, *y))
            .for_each(|c| {
                if c.0 % 2 == 0 {
                    assert!(engine.is_black(c));
                } else {
                    assert!(engine.is_empty(c));
                }
            });
    }
}
