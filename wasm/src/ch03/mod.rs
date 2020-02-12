//! Creating Rusty Checkers
pub mod board;
pub mod game;

use lazy_static::lazy_static;
use mut_static::MutStatic;

use board::{Coordinate, Move, Piece};
use game::Engine;

lazy_static! {
    pub static ref GAME_ENGINE: MutStatic<Engine> = { MutStatic::from(Engine::new()) };
}

extern "C" {
    fn notify_piecemoved(fromX: i32, fromY: i32, toX: i32, toY: i32);
    fn notify_piececrowned(x: i32, y: i32);
}

#[no_mangle]
pub extern "C" fn get_piece(x: i32, y: i32) -> i32 {
    let engine = GAME_ENGINE.read().unwrap();
    match engine.get_piece(Coordinate(x as usize, y as usize)) {
        Ok(Some(p)) => p.into(),
        Ok(None) => -1,
        Err(_) => -1,
    }
}

#[no_mangle]
pub extern "C" fn get_current_turn() -> i32 {
    let engine = GAME_ENGINE.read().unwrap();
    Piece::new(engine.current_turn()).into()
}

#[no_mangle]
pub extern "C" fn move_piece(fx: i32, fy: i32, tx: i32, ty: i32) -> i32 {
    let mut engine = GAME_ENGINE.write().unwrap();
    let mv = Move::new((fx as usize, fy as usize), (tx as usize, ty as usize));
    match engine.move_piece(&mv) {
        Ok(ret) => {
            unsafe {
                notify_piecemoved(fx, fy, tx, ty);
            }
            if ret.crowned {
                unsafe {
                    notify_piececrowned(tx, ty);
                }
            }
            1
        }
        Err(_) => 0,
    }
}
