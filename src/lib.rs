#![no_std]

mod assert;
mod helpers;
mod resistance;

pub use resistance::{ExtF32 as ResistanceExtF32, ExtU32 as ResistanceExtU32, Resistance};
