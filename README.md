# ohms
![crates.io](https://img.shields.io/crates/v/ohms.svg)

Electrical units library for embedded Rust targets focusing on ease-of-use and performance.
Supports `no_std` environments.

Greatly inspired by the [fugit](https://github.com/korken89/fugit) crate.

## Supported Units

- [Current](src/current.rs) (μA, mA, A)
- [Resistance](src/resistance.rs) (mΩ, Ω, kΩ, MΩ)
- [Voltage](src/voltage.rs) (μV, mV, V, kV)

## Ohm's Law

The `Current`, `Resistance` and `Voltage` types follow the [Ohm's Law](https://en.wikipedia.org/wiki/Ohm%27s_law) rules.

This means that you can use the `/` and `*` operators to calculate the missing value.
For example, `Voltage / Current` will return a `Resistance` value.

```

## Installation

You can add this crate via [crates.io](https://crates.io/ohms):

```
$ cargo add ohms
```

## Documentation

You can find the documentation [here](https://docs.rs/ohms/latest/ohms/).
