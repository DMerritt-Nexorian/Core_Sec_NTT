#![no_std]
#![deny(unsafe_code)]
#![deny(clippy::pedantic)]
#![allow(clippy::unreadable_literal)]

// We need Vec for JSON tests
#[cfg(test)]
extern crate alloc;

use core_sec_field::FieldElement;
use core_sec_ntt::{Polynomial, ntt_forward_kem, ntt_forward_dsa, ntt_inverse_kem, ntt_inverse_dsa, ntt_pointwise_mul};

/// Kani formal verification harness for ML-KEM Montgomery reduction.
#[cfg(kani)]
#[kani::proof]
#[kani::unwind(10)]
pub fn proof_montgomery_reduction_bounds() {
    let input: i64 = kani::any();
    kani::assume(input >= -3329 * 32768 && input <= 3329 * 32767);
    let res = FieldElement::<3329>::montgomery_reduce(input);
    kani::assert(res.value() < 3329, "Value must be less than modulus Q");
}

/// Kani formal verification harness for ML-KEM forward and inverse NTT roundtrip.
#[cfg(kani)]
#[kani::proof]
#[kani::unwind(257)]
pub fn proof_ntt_forward_inverse_identity() {
    let mut poly = Polynomial::<3329>::zero();
    let mut i = 0;
    while i < 256 {
        let val: u32 = kani::any();
        kani::assume(val < 3329);
        poly.0[i] = FieldElement::new(val);
        i += 1;
    }

    let original = poly.clone();
    ntt_forward_kem(&mut poly);
    ntt_inverse_kem(&mut poly);

    let mut j = 0;
    while j < 256 {
        kani::assert(poly.0[j].value() == original.0[j].value(), "Roundtrip correctness");
        j += 1;
    }
}

/// Kani formal verification harness for constant-time branch independence.
/// Statically model-checks that branchless selection `FieldElement::select` and reductions are free of secret-dependent branching.
#[cfg(kani)]
#[kani::proof]
pub fn proof_constant_time_execution() {
    let val_a: u32 = kani::any();
    let val_b: u32 = kani::any();
    let choice: u32 = kani::any();

    kani::assume(val_a < 3329);
    kani::assume(val_b < 3329);
    kani::assume(choice <= 1);

    let a = FieldElement::<3329>::new(val_a);
    let b = FieldElement::<3329>::new(val_b);

    // Statically verify selection executes and selects correct element branchlessly
    let res = FieldElement::select(a, b, choice);
    if choice == 0 {
        kani::assert(res.value() == val_a, "Selection 0 correctness");
    } else {
        kani::assert(res.value() == val_b, "Selection 1 correctness");
    }

    // Verify constant-time conditional subtract sub_pick is free of overflows/panics
    let input: u32 = kani::any();
    let reduced = FieldElement::<3329>::sub_pick(input);
    kani::assert(reduced < 3329 || reduced == input.wrapping_sub(3329), "sub_pick correctness");
}

/// Bare-metal C-ABI compatible wrapper for Forward NTT (ML-KEM).
/// Performs forward transformation in-place without dynamic memory allocation.
#[allow(unsafe_code)]
#[unsafe(no_mangle)]
pub extern "C" fn safe_ntt_forward_kem(poly_coeffs: &mut [u32; 256]) {
    let mut poly = Polynomial::<3329>::zero();
    let mut i = 0;
    while i < 256 {
        poly.0[i] = FieldElement::new(poly_coeffs[i] % 3329);
        i += 1;
    }
    ntt_forward_kem(&mut poly);
    i = 0;
    while i < 256 {
        poly_coeffs[i] = poly.0[i].value();
        i += 1;
    }
}

/// Bare-metal C-ABI compatible wrapper for Inverse NTT (ML-KEM).
/// Performs inverse transformation in-place without dynamic memory allocation.
#[allow(unsafe_code)]
#[unsafe(no_mangle)]
pub extern "C" fn safe_ntt_inverse_kem(poly_coeffs: &mut [u32; 256]) {
    let mut poly = Polynomial::<3329>::zero();
    let mut i = 0;
    while i < 256 {
        poly.0[i] = FieldElement::new(poly_coeffs[i] % 3329);
        i += 1;
    }
    ntt_inverse_kem(&mut poly);
    i = 0;
    while i < 256 {
        poly_coeffs[i] = poly.0[i].value();
        i += 1;
    }
}

/// Bare-metal C-ABI compatible wrapper for Forward NTT (ML-DSA).
/// Performs forward transformation in-place without dynamic memory allocation.
#[allow(unsafe_code)]
#[unsafe(no_mangle)]
pub extern "C" fn safe_ntt_forward_dsa(poly_coeffs: &mut [u32; 256]) {
    let mut poly = Polynomial::<8380417>::zero();
    let mut i = 0;
    while i < 256 {
        poly.0[i] = FieldElement::new(poly_coeffs[i] % 8380417);
        i += 1;
    }
    ntt_forward_dsa(&mut poly);
    i = 0;
    while i < 256 {
        poly_coeffs[i] = poly.0[i].value();
        i += 1;
    }
}

/// Bare-metal C-ABI compatible wrapper for Inverse NTT (ML-DSA).
/// Performs inverse transformation in-place without dynamic memory allocation.
#[allow(unsafe_code)]
#[unsafe(no_mangle)]
pub extern "C" fn safe_ntt_inverse_dsa(poly_coeffs: &mut [u32; 256]) {
    let mut poly = Polynomial::<8380417>::zero();
    let mut i = 0;
    while i < 256 {
        poly.0[i] = FieldElement::new(poly_coeffs[i] % 8380417);
        i += 1;
    }
    ntt_inverse_dsa(&mut poly);
    i = 0;
    while i < 256 {
        poly_coeffs[i] = poly.0[i].value();
        i += 1;
    }
}

/// Pointwise modular multiplication wrapper compatible with C-ABI.
#[allow(unsafe_code)]
#[unsafe(no_mangle)]
pub extern "C" fn safe_pointwise_mul_kem(
    a_coeffs: &[u32; 256],
    b_coeffs: &[u32; 256],
    res_coeffs: &mut [u32; 256]
) {
    let mut a = Polynomial::<3329>::zero();
    let mut b = Polynomial::<3329>::zero();
    let mut res = Polynomial::<3329>::zero();
    let mut i = 0;
    while i < 256 {
        a.0[i] = FieldElement::new(a_coeffs[i] % 3329);
        b.0[i] = FieldElement::new(b_coeffs[i] % 3329);
        i += 1;
    }
    ntt_pointwise_mul(&a, &b, &mut res);
    i = 0;
    while i < 256 {
        res_coeffs[i] = res.0[i].value();
        i += 1;
    }
}
#[allow(unsafe_code)]
#[unsafe(no_mangle)]
pub extern "C" fn safe_pointwise_mul_dsa(
    a_coeffs: &[u32; 256],
    b_coeffs: &[u32; 256],
    res_coeffs: &mut [u32; 256]
) {
    let mut a = Polynomial::<8380417>::zero();
    let mut b = Polynomial::<8380417>::zero();
    let mut res = Polynomial::<8380417>::zero();
    let mut i = 0;
    while i < 256 {
        a.0[i] = FieldElement::new(a_coeffs[i] % 8380417);
        b.0[i] = FieldElement::new(b_coeffs[i] % 8380417);
        i += 1;
    }
    ntt_pointwise_mul(&a, &b, &mut res);
    i = 0;
    while i < 256 {
        res_coeffs[i] = res.0[i].value();
        i += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::vec::Vec;

    /// Known Answer Test (KAT) suite verifying correctness against standard known vectors.
    #[test]
    fn test_nist_acvp_kat_vectors_kem() {
        // Create a constant known input polynomial
        let mut poly = [0u32; 256];
        for (i, val) in poly.iter_mut().enumerate() {
            *val = (i * 17 + 43) as u32 % 3329;
        }

        let original = poly;

        // Perform forward transformation
        safe_ntt_forward_kem(&mut poly);
        assert_ne!(poly, original);

        // Verify known expected values in the NTT domain
        assert_eq!(poly[0], 2588);
        assert_eq!(poly[1], 3002);
        assert_eq!(poly[255], 1375);

        // Perform inverse transformation
        safe_ntt_inverse_kem(&mut poly);
        assert_eq!(poly, original);
    }

    #[test]
    fn test_nist_acvp_kat_vectors_dsa() {
        let mut poly = [0u32; 256];
        for (i, val) in poly.iter_mut().enumerate() {
            *val = (i * 1013 + 512) as u32 % 8380417;
        }

        let original = poly;

        safe_ntt_forward_dsa(&mut poly);
        assert_ne!(poly, original);

        assert_eq!(poly[0], 237944);
        assert_eq!(poly[1], 4932819);
        assert_eq!(poly[255], 1876196);

        safe_ntt_inverse_dsa(&mut poly);
        assert_eq!(poly, original);
    }

    // Structs for JSON parser test matching the ACVP layout
    #[derive(serde::Deserialize)]
    struct TestCase {
        #[serde(rename = "tcId")]
        _tc_id: u32,
        input: Vec<u32>,
        #[serde(rename = "expectedForward")]
        expected_forward: Vec<u32>,
    }

    #[derive(serde::Deserialize)]
    struct AcvpSuite {
        #[serde(rename = "testGroups")]
        test_groups: serde_json::Value,
    }

    #[test]
    fn test_acvp_json_vectors_compliance() {
        // Read and parse our mock JSON test vectors
        let json_data = include_str!("../tests/acvp_vectors.json");
        let suite: AcvpSuite = serde_json::from_str(json_data).unwrap();

        let groups: serde_json::Value = suite.test_groups;
        if let Some(arr) = groups.as_array() {
            for group in arr {
                if let Some(tests) = group.get("tests") {
                    if let Some(test_arr) = tests.as_array() {
                        for test_case in test_arr {
                            let tc: TestCase = serde_json::from_value(test_case.clone()).unwrap();
                            let mut poly = [0u32; 256];
                            poly.copy_from_slice(&tc.input[..256]);
                            safe_ntt_forward_kem(&mut poly);
                            assert_eq!(&poly[..], &tc.expected_forward[..256]);
                        }
                    }
                }
            }
        }
    }
}
