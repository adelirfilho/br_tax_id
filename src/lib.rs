//! # `br_tax_id`
//!
//! An ultra-fast, zero-allocation, and `#![no_std]` compatible Rust library for
//! validating and parsing Brazilian tax identification numbers (CPF and CNPJ).

#![no_std]

use core::fmt;
use core::str::FromStr;

/// Represents the type of Brazilian tax identification document.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TaxIdType {
    /// Cadastro de Pessoas Físicas (11 digits).
    Cpf,
    /// Cadastro Nacional da Pessoa Jurídica (14 digits).
    Cnpj,
}

/// Represents the possible validation or parsing errors.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ValidationError {
    /// DoS Mitigation: Input string exceeds 50 bytes.
    PayloadTooLarge,
    /// Fail-Secure: Input contains characters outside ASCII digits and standard delimiters.
    InvalidCharacters,
    /// Digit count mismatch: Input does not contain exactly 11 (CPF) or 14 (CNPJ) digits.
    InvalidLength,
    /// Algorithmic failure: Verification check digits failed or sequence consists of repeated numbers.
    InvalidChecksum,
}

impl fmt::Display for ValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ValidationError::PayloadTooLarge => {
                write!(f, "Input byte length exceeds maximum limit of 50 bytes")
            }
            ValidationError::InvalidCharacters => {
                write!(f, "Input contains invalid or unsafe characters")
            }
            ValidationError::InvalidLength => write!(
                f,
                "Input digit count is not valid for CPF (11) or CNPJ (14)"
            ),
            ValidationError::InvalidChecksum => write!(
                f,
                "Check digits validation failed or identical sequence detected"
            ),
        }
    }
}

// Implements core::error::Error for modern Rust (edition 2024 / core compatibility)
impl core::error::Error for ValidationError {}

impl FromStr for TaxIdType {
    type Err = ValidationError;

    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        validate_tax_id(s)
    }
}

/// Validates whether a given string is a valid Brazilian CPF or CNPJ.
///
/// Automatically filters out standard formatting characters (`.`, `-`, `/`, ` `).
/// Enforces an upper-bound input length constraint to prevent CPU-exhaustion Denial of Service (DoS) attacks.
///
/// # Errors
/// Returns a [`ValidationError`] describing why the input is invalid.
///
/// # Examples
/// ```rust
/// use br_tax_id::{validate_tax_id, TaxIdType, ValidationError};

/// assert_eq!(validate_tax_id("529.982.247-25"), Ok(TaxIdType::Cpf));
/// assert_eq!(validate_tax_id("11.222.333/0001-81"), Ok(TaxIdType::Cnpj));
/// assert_eq!(validate_tax_id("000.000.000-00"), Err(ValidationError::InvalidChecksum));
/// ```
pub fn validate_tax_id(tax_id: &str) -> Result<TaxIdType, ValidationError> {
    // DoS Mitigation: Fast-fail if string byte length exceeds 50 bytes.
    if tax_id.len() > 50 {
        return Err(ValidationError::PayloadTooLarge);
    }

    // Stack-allocated fixed array ensuring zero heap allocations.
    let mut digits = [0u8; 14];
    let mut count = 0;

    let bytes = tax_id.as_bytes();

    for &b in bytes {
        match b {
            b'0'..=b'9' => {
                if count >= 14 {
                    // Exceeds maximum allowed tax ID digits (CNPJ is max 14).
                    return Err(ValidationError::InvalidLength);
                }
                digits[count] = b - b'0';
                count += 1;
            }
            // Accepted delimiters (ignored without failing)
            b'.' | b'-' | b'/' | b' ' => continue,
            // Fail-secure: any other character immediately rejects the input
            _ => return Err(ValidationError::InvalidCharacters),
        }
    }

    match count {
        11 => {
            if validate_cpf(&digits[0..11]) {
                Ok(TaxIdType::Cpf)
            } else {
                Err(ValidationError::InvalidChecksum)
            }
        }
        14 => {
            if validate_cnpj(&digits) {
                Ok(TaxIdType::Cnpj)
            } else {
                Err(ValidationError::InvalidChecksum)
            }
        }
        _ => Err(ValidationError::InvalidLength),
    }
}

/// Internal logic for validating an 11-digit CPF array slice.
fn validate_cpf(digits: &[u8]) -> bool {
    // Reject sequences with all identical digits (e.g., "111.111.111-11").
    if digits.windows(2).all(|w| w[0] == w[1]) {
        return false;
    }

    let calc_digit = |slice: &[u8], mut weight: u32| -> u8 {
        let sum: u32 = slice
            .iter()
            .map(|&d| {
                let res = d as u32 * weight;
                weight -= 1;
                res
            })
            .sum();

        let rem = sum % 11;
        if rem < 2 {
            0
        } else {
            (11 - rem) as u8
        }
    };

    let d1 = calc_digit(&digits[0..9], 10);
    let d2 = calc_digit(&digits[0..10], 11);

    d1 == digits[9] && d2 == digits[10]
}

/// Internal logic for validating a 14-digit CNPJ array slice.
fn validate_cnpj(digits: &[u8]) -> bool {
    // Reject sequences with all identical digits (e.g., "00.000.000/0000-00").
    if digits.windows(2).all(|w| w[0] == w[1]) {
        return false;
    }

    let calc_digit = |slice: &[u8], weights: &[u32]| -> u8 {
        let sum: u32 = slice
            .iter()
            .zip(weights.iter())
            .map(|(&d, &w)| d as u32 * w)
            .sum();

        let rem = sum % 11;
        if rem < 2 {
            0
        } else {
            (11 - rem) as u8
        }
    };

    let w1 = [5, 4, 3, 2, 9, 8, 7, 6, 5, 4, 3, 2];
    let w2 = [6, 5, 4, 3, 2, 9, 8, 7, 6, 5, 4, 3, 2];

    let d1 = calc_digit(&digits[0..12], &w1);
    let d2 = calc_digit(&digits[0..13], &w2);

    d1 == digits[12] && d2 == digits[13]
}

// ============================================================================
// Unit Tests Suite
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    extern crate std;
    use std::string::String;

    #[test]
    fn test_valid_cpf() {
        assert_eq!(validate_tax_id("52998224725"), Ok(TaxIdType::Cpf));
        assert_eq!(validate_tax_id("529.982.247-25"), Ok(TaxIdType::Cpf));
    }

    #[test]
    fn test_invalid_cpf_checksum() {
        assert_eq!(
            validate_tax_id("52998224724"),
            Err(ValidationError::InvalidChecksum)
        );
        assert_eq!(
            validate_tax_id("111.111.111-11"),
            Err(ValidationError::InvalidChecksum)
        );
        assert_eq!(
            validate_tax_id("00000000000"),
            Err(ValidationError::InvalidChecksum)
        );
    }

    #[test]
    fn test_valid_cnpj() {
        assert_eq!(validate_tax_id("11222333000181"), Ok(TaxIdType::Cnpj));
        assert_eq!(validate_tax_id("11.222.333/0001-81"), Ok(TaxIdType::Cnpj));
    }

    #[test]
    fn test_invalid_cnpj_checksum() {
        assert_eq!(
            validate_tax_id("11.222.333/0001-00"),
            Err(ValidationError::InvalidChecksum)
        );
        assert_eq!(
            validate_tax_id("00.000.000/0000-00"),
            Err(ValidationError::InvalidChecksum)
        );
        assert_eq!(
            validate_tax_id("11111111111111"),
            Err(ValidationError::InvalidChecksum)
        );
    }

    #[test]
    fn test_dos_mitigation_oversized_string() {
        let oversized_payload = "a".repeat(51);
        assert_eq!(
            validate_tax_id(&oversized_payload),
            Err(ValidationError::PayloadTooLarge)
        );
    }

    #[test]
    fn test_invalid_length() {
        // 15 digits (overflow)
        assert_eq!(
            validate_tax_id("11.222.333/0001-819"),
            Err(ValidationError::InvalidLength)
        );
        // Too short
        assert_eq!(
            validate_tax_id("12345"),
            Err(ValidationError::InvalidLength)
        );
        assert_eq!(validate_tax_id(""), Err(ValidationError::InvalidLength));
    }

    #[test]
    fn test_strict_validation_rejects_junk() {
        assert_eq!(
            validate_tax_id("529.982.247-25<script>"),
            Err(ValidationError::InvalidCharacters)
        );
        assert_eq!(
            validate_tax_id("529a982b247c25"),
            Err(ValidationError::InvalidCharacters)
        );
        assert_eq!(
            validate_tax_id("11.222.333/0001-81\0"),
            Err(ValidationError::InvalidCharacters)
        );
    }

    #[test]
    fn test_from_str_trait_parsing() {
        // Test parsing directly using .parse()
        let cpf_res: Result<TaxIdType, ValidationError> = "529.982.247-25".parse();
        assert_eq!(cpf_res, Ok(TaxIdType::Cpf));

        let cnpj_res: Result<TaxIdType, ValidationError> = "11.222.333/0001-81".parse();
        assert_eq!(cnpj_res, Ok(TaxIdType::Cnpj));

        let err_res: Result<TaxIdType, ValidationError> = "invalid_payload".parse();
        assert_eq!(err_res, Err(ValidationError::InvalidCharacters));
    }
}
