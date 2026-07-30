//! # `br_tax_id`
//!
//! An ultra-fast, zero-allocation, and `#![no_std]` compatible Rust library for
//! validating Brazilian tax identification numbers (CPF and CNPJ).

#![no_std]

/// Represents the type of Brazilian tax identification document.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TaxIdType {
    /// Cadastro de Pessoas Físicas (11 digits).
    Cpf,
    /// Cadastro Nacional da Pessoa Jurídica (14 digits).
    Cnpj,
}

/// Validates whether a given string is a valid Brazilian CPF or CNPJ.
///
/// Automatically filters out standard formatting characters (`.`, `-`, `/`).
/// Enforces an upper-bound input length constraint to prevent CPU-exhaustion Denial of Service (DoS) attacks.
///
/// # Returns
/// - `Some(TaxIdType::Cpf)` if the input is a valid 11-digit CPF.
/// - `Some(TaxIdType::Cnpj)` if the input is a valid 14-digit CNPJ.
/// - `None` if check digits fail, formatting is invalid, or length bounds are violated.
///
/// # Examples
/// ```rust
/// use br_tax_id::{validate_tax_id, TaxIdType};
///
/// assert_eq!(validate_tax_id("529.982.247-25"), Some(TaxIdType::Cpf));
/// assert_eq!(validate_tax_id("11.222.333/0001-81"), Some(TaxIdType::Cnpj));
/// assert_eq!(validate_tax_id("000.000.000-00"), None);
/// ```
pub fn validate_tax_id(tax_id: &str) -> Option<TaxIdType> {
    // DoS Mitigation: Fast-fail if string byte length exceeds 50 bytes.
    // Prevents attackers from sending massive strings to exhaust CPU processing time.
    if tax_id.len() > 50 {
        return None;
    }

    // Stack-allocated fixed array ensuring zero heap allocations.
    let mut digits = [0u8; 14];
    let mut count = 0;

    // Filter numeric ASCII characters, skipping non-digit characters.
    for ch in tax_id.chars() {
        if let Some(digit) = ch.to_digit(10) {
            if count >= 14 {
                // Exceeds maximum allowed tax ID digits (CNPJ is max 14).
                return None;
            }
            digits[count] = digit as u8;
            count += 1;
        }
    }

    match count {
        11 if validate_cpf(&digits[0..11]) => Some(TaxIdType::Cpf),
        14 if validate_cnpj(&digits) => Some(TaxIdType::Cnpj),
        _ => None,
    }
}

/// Internal logic for validating an 11-digit CPF array slice.
fn validate_cpf(digits: &[u8]) -> bool {
    // Reject sequences with all identical digits (e.g., "111.111.111-11").
    if digits.windows(2).all(|w| w[0] == w[1]) {
        return false;
    }

    // Closure to calculate CPF check digits.
    let calc_digit = |slice: &[u8], mut weight: u32| -> u8 {
        let sum: u32 = slice
            .iter()
            .map(|&d| {
                let res = d as u32 * weight;
                weight -= 1;
                res
            })
            .sum();

        let rem = (sum * 10) % 11;
        if rem == 10 { 0 } else { rem as u8 }
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

    // Closure to calculate CNPJ check digits.
    let calc_digit = |slice: &[u8], weights: &[u32]| -> u8 {
        let sum: u32 = slice
            .iter()
            .zip(weights.iter())
            .map(|(&d, &w)| d as u32 * w)
            .sum();

        let rem = sum % 11;
        if rem < 2 { 0 } else { (11 - rem) as u8 }
    };

    // Constant weighting factors for CNPJ calculation algorithms.
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

    // Re-enable std for testing allocations (e.g., String repeat for DoS test)
    extern crate std;
    use std::string::String;

    #[test]
    fn test_valid_cpf() {
        // Valid unformatted CPF
        assert_eq!(validate_tax_id("52998224725"), Some(TaxIdType::Cpf));
        // Valid formatted CPF
        assert_eq!(validate_tax_id("529.982.247-25"), Some(TaxIdType::Cpf));
    }

    #[test]
    fn test_invalid_cpf() {
        // Wrong verification digits
        assert_eq!(validate_tax_id("52998224724"), None);
        // All identical digits (must fail despite mathematically passing check digit)
        assert_eq!(validate_tax_id("111.111.111-11"), None);
        assert_eq!(validate_tax_id("00000000000"), None);
    }

    #[test]
    fn test_valid_cnpj() {
        // Valid unformatted CNPJ
        assert_eq!(validate_tax_id("11222333000181"), Some(TaxIdType::Cnpj));
        // Valid formatted CNPJ
        assert_eq!(validate_tax_id("11.222.333/0001-81"), Some(TaxIdType::Cnpj));
    }

    #[test]
    fn test_invalid_cnpj() {
        // Wrong verification digits
        assert_eq!(validate_tax_id("11.222.333/0001-00"), None);
        // All identical digits
        assert_eq!(validate_tax_id("00.000.000/0000-00"), None);
        assert_eq!(validate_tax_id("11111111111111"), None);
    }

    #[test]
    fn test_dos_mitigation_oversized_string() {
        // Build a 51-byte non-digit string to test memory/CPU bound guardrails
        let oversized_payload = "a".repeat(51);
        assert_eq!(validate_tax_id(&oversized_payload), None);
    }

    #[test]
    fn test_overflow_digit_count() {
        // 15 digits (valid CNPJ sequence plus an extra digit)
        assert_eq!(validate_tax_id("11.222.333/0001-819"), None);
    }

    #[test]
    fn test_malformed_and_empty_inputs() {
        assert_eq!(validate_tax_id("invalid_input_payload"), None);
        assert_eq!(validate_tax_id(""), None);
        assert_eq!(validate_tax_id("123.abc"), None);
    }
}
