#![allow(clippy::all)]
pub struct ParserError;

include!(concat!(env!("OUT_DIR"), "/parser.rs"));
