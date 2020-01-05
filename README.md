# ruuvi-sensor-protocol-rs [![Crates.io](https://img.shields.io/crates/v/ruuvi-sensor-protocol.svg)](https://crates.io/crates/ruuvi-sensor-protocol) [![Docs.rs](https://docs.rs/ruuvi-sensor-protocol/badge.svg)](https://docs.rs/ruuvi-sensor-protocol) [![Crates.io](https://img.shields.io/crates/l/ruuvi-sensor-protocol.svg)](https://crates.io/crates/ruuvi-sensor-protocol)

Ruuvi sensor protocol parser implementation

## Requirements
- Rust `>= 1.34`
  - This crate can be compiled without `std` by disabling default features

## Crate Features
- `std` (default) enables features that depend on `std` crate

## Documentation
Docs are available online at
[docs.rs](https://docs.rs/ruuvi-sensor-protocol). They can be built
from source with `cargo doc`. Examples are included in the docs.

## Changes

### `0.4.1`
- Corrected `ZERO_CELCIUS_IN_MILLIKELVINS` constant

### `0.4.0`
- Requires Rust `>= 1.34`
- Add support for [data format 5 (RAWv2)](https://github.com/ruuvi/ruuvi-sensor-protocols/blob/master/dataformat_05.md).

### `0.3.0`
- Requires 2018 Edition (Rust `>= 1.31`)
- Adds option to compile without `std` create

## License
This project is licensed under [MIT license](LICENSE).
