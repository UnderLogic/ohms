//! Electrical unit types for embedded targets focusing on ease-of-use and performance
//!
//! Supported units:
//! - Current (μA, mA, A)
//! - Resistance (mΩ, Ω, kΩ, MΩ)
//! - Voltage (μV, mV, V)
//!
//! Each unit type has a corresponding extension trait for creating values from `i32`/`u32` and `f32`
//! values. The extension traits are named `ExtI32`, `ExtU32`, and `ExtF32` respectively.
//!
//! Unit types can easily be converted to and from different denominations.
//!
#![no_std]

mod assert;
mod current;
mod helpers;
pub mod prelude;
mod resistance;
mod voltage;

pub use current::{Current, ExtF32 as CurrentExtF32, ExtU32 as CurrentExtU32};
pub use resistance::{ExtF32 as ResistanceExtF32, ExtU32 as ResistanceExtU32, Resistance};
pub use voltage::{ExtF32 as VoltageExtF32, ExtI32 as VoltageExtI32, Voltage};
