mod args;
mod chunk;
mod chunk_type;
mod commands;
mod png;

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

use chunk_type::ChunkType;
use std::str::FromStr;

fn main() {
 let chunk_type: ChunkType = ChunkType::from_str("RuSt").unwrap();
    println!("{}", chunk_type);
}
