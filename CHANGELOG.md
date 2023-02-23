# Changelog
All notable changes to this library will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.3.2] - 2023-02-23

### Fixed

- Spacing for units in `Display` trait for `Current`, `Voltage`, and `Resistance` structs

## [0.3.1] - 2023-02-23

### Added

- Implement `Display` trait for `Current`, `Voltage`, and `Resistance` structs

## [0.3.0] - 2023-02-22

### Added

- `Resistance` extension methods for all integer and floating point types via macros
- `Voltage` extension methods for all integer and floating point types via macros
- `Current` extension methods for all integer and floating point types via macros

### Changed

- `Resistance` values are now stored as `u64` internally (from `u32`)
- `Voltage` values are now stored as `i64` internally (from `i32`)
- `Current` values are now stored as `u64` internally (from `u32`)
- `Resistance` extension traits renamed to `ResistanceFromInteger` and `ResistanceFromFloat`
- `Voltage` extension traits renamed to `VoltageFromInteger` and `VoltageFromFloat`
- `Current` extension traits renamed to `CurrentFromInteger` and `CurrentFromFloat`

### Removed

- Unit tests for `Resistance`, `Voltage`, and `Current`

## [0.2.0] - 2023-02-17

### Added

- Ohm's Law calculations for `Voltage`, `Current`, and `Resistance` via operators
- Unit tests for Ohm's Law calculations

## [0.1.4] - 2023-02-16

### Added

- Unit tests for `Current` struct
- Unit tests for `Voltage` struct
- Unit tests for `Resistance` struct

### Fixed

- Updated docs to pass `cargo test`

## [0.1.3] - 2023-02-03

### Fixed

- Include extension traits explicitly in `prelude` module

## [0.1.2] - 2023-02-03

### Added

- Prelude module available via `ohms::prelude::*`

## [0.1.1] - 2023-02-03

### Added

- `micro_volts()` method for `Voltage` struct

## [0.1.0] - 2023-02-03

### Added

- `Current` struct for storing current values
- `Current` extension methods for `u32` and `f32` values
- `Voltage` struct for storing voltage values (positive and negative)
- `Voltage` extension methods for `i32` and `f32` values
- `Resistance` struct for storing resistance values
- `Resistance` extension methods for `u32` and `f32` values
