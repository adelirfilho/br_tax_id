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
- 🎯 **Granular Observability**: Returns detailed `Result<TaxIdType, ValidationError>` variants, allowing consuming systems to distinguish between simple user typos and malicious injection/DoS attempts.
- 🔄 **Idiomatic Rust**: Implements `FromStr`, allowing native `.parse::<TaxIdType>()` calls.
- 📦 **Zero Dependencies**: Clean dependency graph shielding your project from supply-chain risks.

## 📦 Installation

Add the following to your `Cargo.toml`:

```toml
[dependencies]
br_tax_id = "0.2.3"
```

Or simply run:

```bash
cargo add br_tax_id
```

## 🛠️ Usage

## 1. Idiomatic Parsing (Using `.parse()`)

By implementing `FromStr`, you can parse strings directly into `TaxIdType`.

```rust
use br_tax_id::{TaxIdType, ValidationError};

fn main() -> Result<(), ValidationError> {
    let document = "529.982.247-25";
    let tax_id: TaxIdType = document.parse()?;

    match tax_id {
        TaxIdType::Cpf => println!("Valid CPF detected!"),
        TaxIdType::Cnpj => println!("Valid CNPJ detected!"),
    }

    Ok(())
}
```

## 2. Fast Validation Check (Using `.is_ok()`)

Ideal when you only need to verify validity without inspecting the exact error or document type.

```rust
use br_tax_id as br;

fn main() {
    let document = "11.222.333/0001-81";

    if br::validate_tax_id(document).is_ok() {
        println!("Valid document!");
    } else {
        println!("Invalid document.");
    }
}
```
## 3. Granular Error Handling & Security Telemetry

Distinguish between malicious payloads, character injection, and simple bad checksums.

```rust
use br_tax_id::{validate_tax_id, TaxIdType, ValidationError};

fn main() {
    let payload = "529.982.247-25<script>";

    match validate_tax_id(payload) {
        Ok(TaxIdType::Cpf) => println!("Processed CPF."),
        Ok(TaxIdType::Cnpj) => println!("Processed CNPJ."),
        Err(ValidationError::PayloadTooLarge) => {
            eprintln!("SECURITY ALERT: Possible DoS attempt (oversized payload).");
        }
        Err(ValidationError::InvalidCharacters) => {
            eprintln!("SECURITY ALERT: Invalid characters detected in input.");
        }
        Err(ValidationError::InvalidChecksum) => {
            println!("User typed an incorrect document number.");
        }
        Err(ValidationError::InvalidLength) => {
            println!("Document length is wrong.");
        }
    }
}
```
## ⚙️ Optimization & Performance

This crate is built for speed and minimal binary size. If you are building an application with this library, it's recommended to apply aggressive optimization profiles in your `Cargo.toml`:

```toml
[profile.release]
opt-level = 3
lto = true
codegen-units = 1
panic = "abort"
strip = true
```

## 🛡️ Security & Resilience Testing

This crate is subjected to continuous fuzz testing using `cargo-fuzz` and `libFuzzer` to ensure memory safety and robustness against unexpected inputs. 

The `fuzz/` directory contains our fuzzing targets, which continuously bombard the `validate_tax_id` function with mutated payloads. We verify that:
- No input combination triggers an unexpected `panic!`.
- Memory usage remains stable (stack-only).
- Fail-secure logic remains consistent across all mutations.

Our current testing infrastructure ensures that even highly obfuscated, malformed, or over-sized payloads are handled gracefully without compromising the host system.

## 📝 License

This project is dual-licensed under either the [MIT License](https://opensource.org/licenses/MIT) or the [Apache License, Version 2.0](https://www.apache.org/licenses/LICENSE-2.0), at your option.