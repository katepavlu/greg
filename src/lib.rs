pub mod mylexer;
pub mod io;
mod types;
pub use types::*;

pub mod parser;

pub const DATA_ADDRESS_OFFSET:u32 = 0x10000000;
pub const TEXT_ADDRESS_OFFSET:u32 = 0x00000000;
