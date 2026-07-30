# 🇧🇷 br_tax_id

[![Crates.io](https://img.shields.io/crates/v/br_tax_id.svg)](https://crates.io/crates/br_tax_id)
[![Documentation](https://docs.rs/br_tax_id/badge.svg)](https://docs.rs/br_tax_id)
[![License](https://img.shields.io/crates/l/br_tax_id.svg)](https://crates.io/crates/br_tax_id)

Extremely fast, zero-allocation, and `no_std`-compatible CPF and CNPJ validator for Rust.

## ✨ Features

- 🚀 **Blazing Fast**: Designed to execute with minimal CPU cycles.
- 🛡️ **Zero Allocation**: Fully `#![no_std]` compatible. Uses fixed stack-allocated arrays ensuring no heap memory is dynamically allocated.
- 🔒 **Security First**: Built-in DoS (Denial of Service) mitigation prevents CPU exhaustion by instantly rejecting input strings exceeding 50 bytes.
- 🧹 **Flexible Input**: Automatically ignores standard formatting characters like `.`, `-`, and `/`, focusing only on numeric ASCII characters.
- 📦 **Zero Dependencies**: Keeps your dependency tree completely clean, shielding your project from third-party vulnerabilities.

## 📦 Installation

Add the following to your `Cargo.toml`:

```toml
[dependencies]
br_tax_id = "0.1.0"
```

Or simply run:

```bash
cargo add br_tax_id
```

## 🛠️ Usage

The library provides a simple and ergonomic API through the `validate_tax_id` function, returning an `Option<TaxIdType>`.

```rust
use br_tax_id::{validate_tax_id, TaxIdType};

fn main() {
    // Validating a formatted CPF
    assert_eq!(validate_tax_id("529.982.247-25"), Some(TaxIdType::Cpf));

    // Validating an unformatted CNPJ
    assert_eq!(validate_tax_id("11222333000181"), Some(TaxIdType::Cnpj));

    // Invalid sequences or wrong verification digits return None
    assert_eq!(validate_tax_id("000.000.000-00"), None);
}
```

## ⚙️ Optimization & Performance

This crate is built for speed and minimal binary size. If you are building an application with this library, it's recommended to apply aggressive optimization profiles. The project natively supports:

- **LTO (Link Time Optimization)** to remove dead code.
- **`panic = "abort"`** to reduce binary size by skipping stack unwinding.
- **Stripped binaries** to eliminate debug symbols.

## 📝 License

This project is dual-licensed under either the [MIT License](https://opensource.org/licenses/MIT) or the [Apache License, Version 2.0](https://www.apache.org/licenses/LICENSE-2.0), at your option.