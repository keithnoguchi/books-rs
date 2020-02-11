//! Creating Rusty Checkers
pub mod board;
pub mod game;

use game::Engine;

use lazy_static::lazy_static;
use mut_static::MutStatic;

lazy_static! {
    pub static ref GAME_ENGINE: MutStatic<Engine> =
    { MutStatic::from(Engine::new()) };
}
