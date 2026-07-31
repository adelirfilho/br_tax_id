# 🇧🇷 br_tax_id

[![Crates.io](https://img.shields.io/crates/v/br_tax_id.svg)](https://crates.io/crates/br_tax_id)
[![Documentation](https://docs.rs/br_tax_id/badge.svg)](https://docs.rs/br_tax_id)
[![License](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue.svg)](https://crates.io/crates/br_tax_id)

Extremely fast, zero-allocation, and `no_std`-compatible CPF and CNPJ validator for Rust.

## ✨ Features

- 🚀 **Blazing Fast**: Direct byte-slice parsing (`as_bytes()`) avoids UTF-8 decoding overhead. Check-digit algorithms use branchless ALU math for maximum CPU performance.
- 🛡️ **Zero Allocation**: Fully `#![no_std]` compatible. Operates entirely on stack memory using fixed-size byte buffers.
- 🔒 **Security First (Fail-Secure)**: 
  - **Strict Allowlisting**: Rejects any character outside valid ASCII digits (`0-9`) and expected delimiters (`.`, `-`, `/`, ` `) to prevent downstream injection vulnerabilities.
  - **DoS Mitigation**: Instantly drops input strings exceeding 50 bytes to eliminate CPU exhaustion vectors.
- 🔄 **Thread-Safe & Reentrant**: Completely stateless and free of global shared state, inherently thread-safe for multi-threading and `no_std` RTOS setups.
- 📦 **Zero Dependencies**: Clean dependency graph shielding your project from supply-chain risks.

## 📦 Installation

Add the following to your `Cargo.toml`:

```toml
[dependencies]
br_tax_id = "0.1.6"
```

Or simply run:

```bash
cargo add br_tax_id
```

## 🛠️ Usage

The library provides a simple and ergonomic API through the `validate_tax_id` function, returning an `Option<TaxIdType>`.

## 1. Quick Validation (Using .is_some() / .is_none())

Ideal when you only need to check if the document is valid, without caring whether it is a CPF or a CNPJ.

```rust
use br_tax_id as br;

fn main() {
    let document = "529.982.247-25";

    if br::validate_tax_id(document).is_some() {
        println!("Valid document!");
    } else {
        println!("Invalid document or unexpected payload.");
    }
}
```

## 2. Detailed Validation (Using match or if let)

Use this when you need to know exactly whether the document is a CPF or a CNPJ, or if it's invalid.

```rust
use br_tax_id::{validate_tax_id, TaxIdType};

fn main() {
    let document = "11.222.333/0001-81";

    match validate_tax_id(document) {
        Some(TaxIdType::Cpf) => println!("Processed a valid CPF."),
        Some(TaxIdType::Cnpj) => println!("Processed a valid CNPJ."),
        _ => println!("Invalid document or unexpected payload."),
    }
}
```

## ⚙️ Optimization & Performance

This crate is built for speed and minimal binary size. If you are building an application with this library, it's recommended to apply aggressive optimization profiles. The project natively supports:

- **LTO (Link Time Optimization)** to remove dead code.
- **`panic = "abort"`** to reduce binary size by skipping stack unwinding.
- **Stripped binaries** to eliminate debug symbols.

## 📝 License

This project is dual-licensed under either the [MIT License](https://opensource.org/licenses/MIT) or the [Apache License, Version 2.0](https://www.apache.org/licenses/LICENSE-2.0), at your option.