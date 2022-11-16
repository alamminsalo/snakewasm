pub mod game;
#[cfg(feature = "py")]
mod py;
pub mod snake;
mod util;

pub use self::game::Game;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
