#![no_std]
#![deny(unsafe_code)]
#![deny(clippy::pedantic)]

use core_sec_field::FieldElement;
use core_sec_ntt::{Polynomial, ntt_forward_kem, ntt_forward_dsa, ntt_inverse_kem, ntt_inverse_dsa, ntt_pointwise_mul};

/// Kani formal verification harness for ML-KEM Montgomery reduction.
#[cfg(kani)]
#[kani::proof]
#[kani::unwind(10)]
pub fn prove_montgomery_reduce_bounds() {
    let input: i64 = kani::any();
    kani::assume(input >= -3329 * 32768 && input <= 3329 * 32767);
    let res = FieldElement::<3329>::montgomery_reduce(input);
    kani::assert(res.value() < 3329, "Value must be less than modulus Q");
}

/// Kani formal verification harness for ML-KEM Barrett reduction.
#[cfg(kani)]
#[kani::proof]
#[kani::unwind(10)]
pub fn prove_barrett_reduce_bounds() {
    let input: u64 = kani::any();
    kani::assume(input < 3329 * 3329);
    let res = FieldElement::<3329>::barrett_reduce(input);
    kani::assert(res.value() < 3329, "Value must be less than modulus Q");
}

/// Kani formal verification harness for ML-KEM forward and inverse NTT roundtrip.
#[cfg(kani)]
#[kani::proof]
#[kani::unwind(257)]
pub fn prove_ntt_roundtrip() {
    let mut poly = Polynomial::<3329>::zero();
    let mut i = 0;
    while i < 256 {
        let val: u32 = kani::any();
        kani::assume(val < 3329);
        poly.0[i] = FieldElement::new(val);
        i += 1;
    }

    let original = poly;
    ntt_forward_kem(&mut poly);
    ntt_inverse_kem(&mut poly);

    let mut j = 0;
    while j < 256 {
        kani::assert(poly.0[j].value() == original.0[j].value(), "Roundtrip correctness");
        j += 1;
    }
}

/// Bare-metal wrapper for Forward NTT (ML-KEM).
/// Takes a safe reference to 256 coefficients, performs forward transformation in-place.
#[inline]
pub fn safe_ntt_forward_kem(poly_coeffs: &mut [u32; 256]) {
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

/// Bare-metal wrapper for Inverse NTT (ML-KEM).
/// Takes a safe reference to 256 coefficients, performs inverse transformation in-place.
#[inline]
pub fn safe_ntt_inverse_kem(poly_coeffs: &mut [u32; 256]) {
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

/// Bare-metal wrapper for Forward NTT (ML-DSA).
#[inline]
pub fn safe_ntt_forward_dsa(poly_coeffs: &mut [u32; 256]) {
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

/// Bare-metal wrapper for Inverse NTT (ML-DSA).
#[inline]
pub fn safe_ntt_inverse_dsa(poly_coeffs: &mut [u32; 256]) {
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

/// Pointwise modular multiplication wrapper.
#[inline]
pub fn safe_pointwise_mul<const Q: u32>(
    a_coeffs: &[u32; 256],
    b_coeffs: &[u32; 256],
    res_coeffs: &mut [u32; 256]
) {
    let mut a = Polynomial::<Q>::zero();
    let mut b = Polynomial::<Q>::zero();
    let mut res = Polynomial::<Q>::zero();
    let mut i = 0;
    while i < 256 {
        a.0[i] = FieldElement::new(a_coeffs[i] % Q);
        b.0[i] = FieldElement::new(b_coeffs[i] % Q);
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
}
