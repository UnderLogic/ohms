# ohms
![crates.io](https://img.shields.io/crates/v/ohms.svg)

Electrical units library for embedded Rust targets focusing on ease-of-use and performance.
Supports `no_std` environments.

Greatly inspired by the [fugit](https://github.com/korken89/fugit) crate.

All units are stored internally as `u64` or `i64` in their base unit.

## Supported Units

- [Current](src/current.rs) (μA, mA, A)
- [Resistance](src/resistance.rs) (mΩ, Ω, kΩ, MΩ)
- [Voltage](src/voltage.rs) (μV, mV, V, kV)
- [Power](src/power.rs) (μW, mW, W, kW)

## Extension Traits

Types implement their own `FromInteger` and `FromFloat` traits for convenience on the following types:

- `u8`
- `u16`
- `u32`
- `u64`
- `i8`
- `i16`
- `i32`
- `i64`
- `f32`
- `f64`

## Ohm's Law

The `Current`, `Resistance` and `Voltage` types follow the [Ohm's Law](https://en.wikipedia.org/wiki/Ohm%27s_law) rules.

This means that you can use the `/` and `*` operators to calculate the missing value.
For example, `Voltage / Current` will return a `Resistance` value.

## Power Calculations

The `Power` type supports calculating the power from multiplying `Voltage` and `Current` values.

## Installation

You can add this crate via [crates.io](https://crates.io/ohms):

```
$ cargo add ohms
```

## Usage

```rust
use ohms::prelude::*;

let voltage = 5.volts();
let current = 1.milli_amps();
let resistance = voltage / current;

let power = voltage * current;

assert_eq!(resistance.ohms(), 5000);
assert_eq!(power.milli_watts(), 5000);
```

## Documentation

You can find the documentation [here](https://docs.rs/ohms/latest/ohms/).
