#![warn(clippy ::all)]
#![warn(future_incompatible, rust_2018_idioms)]
use penrose::{core::bindings::KeyEventHandler, x11rb::RustConn};

//pub mod actions;
pub mod bar;
//pub mod bindings;
pub mod layouts;

pub type KeyHandler = Box<dyn KeyEventHandler<RustConn>>;

//UI Estilo para a barra de estado, mover depois para seu propio arquivo
pub const FONT: &str = "terminus nerd"; // checar nome correto
pub const BLACK: u32 = 0x282828ff;
pub const WHITE: u32 = 0xebdbb2ff;
pub const GREY: u32 = 0x3c3836ff;
pub const BLUE: u32 = 0x458588ff;
pub const RED: u32 = 0xa54242ff;

pub const MAX_MAIN: u32 = 1;
pub const RATIO: f32 = 0.5;
pub const RATIO_STEP: f32 = 0.1;
//const OUTER_PX: u32 = 5;
//const INNER_PX: u32 = 5;
pub const BAR_HEIGHT_PX: u32 = 22;
pub const MAX_ACTIVE_WINDOW_CHARS: usize = 50;
