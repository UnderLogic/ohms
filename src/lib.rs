//! Electrical unit types for embedded targets focusing on ease-of-use and performance
//!
//! Supported units:
//! - Current (μA, mA, A)
//! - Resistance (mΩ, Ω, kΩ, MΩ)
//! - Voltage (μV, mV, V, kV)
//! - Power (μW, mW, W, kW)
//!
//! Each unit type has a corresponding extension trait for creating values from integer and floating point values.
//!
//! Unit types can easily be converted to and from different denominations.
//!
//! Ohm's Law is implemented for `Voltage` and `Current` types, allowing you to easily calculate
//! between the three units using the `/` and `*` operators.
//!
//! ## Examples
//!
//! Determine the resistance of a 5V, 1A load:
//! ```rust
//! use ohms::prelude::*;
//!
//! let voltage = 5.volts();
//! let current = 1.amps();
//!
//! let resistance = voltage / current; // 5V / 1A = 5Ω
//! println!("Resistance: {} Ω", resistance.ohms());
//! ```
//!
//! Determine the current of a 5V, 220Ω load:
//! ```rust
//! use ohms::prelude::*;
//!
//! let voltage = 5.volts();
//! let resistance = 220.ohms();
//!
//! let current = voltage / resistance; // 5V / 220Ω = 22.72mA
//! println!("Current: {} mA", current.milli_amps());
//! ```
//!
//! Determine the voltage of a 0.5A, 4.7kΩ load:
//! ```rust
//! use ohms::prelude::*;
//!
//! let current = 0.5.amps();
//! let resistance = 4.7.kilo_ohms();
//!
//! let voltage = current * resistance; // 0.5A * 4.7kΩ = 2.35V
//! println!("Voltage: {} V", voltage.volts());
//! ```
#![no_std]

mod assert;
mod current;
mod law;
mod power;
pub mod prelude;
mod resistance;
mod voltage;

pub use current::{Current, FromFloat as CurrentFromFloat, FromInteger as CurrentFromInteger};
pub use law::*;
pub use power::{FromFloat as PowerFromFloat, FromInteger as PowerFromInteger, Power};
pub use resistance::{
    FromFloat as ResistanceFromFloat, FromInteger as ResistanceFromInteger, Resistance,
};
pub use voltage::{FromFloat as VoltageFromFloat, FromInteger as VoltageFromInteger, Voltage};
