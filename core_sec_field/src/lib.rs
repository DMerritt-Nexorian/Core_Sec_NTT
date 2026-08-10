#![no_std]
#![deny(unsafe_code)]
#![deny(clippy::pedantic)]
#![allow(unexpected_cfgs)]

#[cfg(kani)]
extern crate kani;

/// Compile-time checked field elements for prime moduli.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct FieldElement<const Q: u32> {
    val: u32,
}

impl<const Q: u32> FieldElement<Q> {
    /// Creates a new field element. Enforces val < Q.
    ///
    /// # Panics
    ///
    /// Panics if `val` is greater than or equal to `Q`.
    #[inline]
    #[must_use]
    pub const fn new(val: u32) -> Self {
        assert!(val < Q, "Value must be less than modulus Q");
        Self { val }
    }

    /// Access the raw inner value.
    #[inline]
    #[must_use]
    pub const fn value(self) -> u32 {
        self.val
    }

    /// Returns the modulus Q.
    #[inline]
    #[must_use]
    pub const fn modulus() -> u32 {
        Q
    }

    /// Constant-time select: returns `a` if `choice == 0`, and `b` if `choice == 1`.
    /// `choice` must be 0 or 1.
    #[inline]
    #[must_use]
    pub const fn select(a: Self, b: Self, choice: u32) -> Self {
        let mask = 0u32.wrapping_sub(choice & 1);
        let val = (a.val & !mask) | (b.val & mask);
        Self { val }
    }

    /// Constant-time conditional subtraction: if `a >= Q`, return `a - Q`, otherwise `a`.
    #[inline]
    #[must_use]
    pub const fn sub_pick(a: u32) -> u32 {
        let bit = (a >= Q) as u32;
        let mask = 0u32.wrapping_sub(bit);
        a.wrapping_sub(mask & Q)
    }

    /// Constant-time addition: `(self + other) mod Q`.
    #[inline]
    #[must_use]
    pub const fn add(self, other: Self) -> Self {
        let sum = self.val.wrapping_add(other.val);
        Self {
            val: Self::sub_pick(sum),
        }
    }

    /// Constant-time subtraction: `(self - other) mod Q`.
    #[inline]
    #[must_use]
    pub const fn sub(self, other: Self) -> Self {
        let diff = self.val.wrapping_add(Q).wrapping_sub(other.val);
        Self {
            val: Self::sub_pick(diff),
        }
    }

    /// Constant-time Montgomery reduction.
    /// Maps a 32-bit signed or 64-bit signed product/value to a representative modulo `Q`.
    /// For `Q = 3329` (ML-KEM): `R = 2^16`, `q_inv` = -3329^-1 mod 2^16 = 3327 (as signed 16-bit: -3327).
    /// For `Q = 8_380_417` (ML-DSA): `R = 2^32`, `q_inv` = -8_380_417^-1 mod 2^32 = 4236238847.
    #[inline]
    #[must_use]
    #[allow(
        clippy::cast_possible_truncation,
        clippy::cast_sign_loss,
        clippy::cast_possible_wrap
    )]
    pub const fn montgomery_reduce(a: i64) -> Self {
        if Q == 3329 {
            // ML-KEM Montgomery reduction
            let a_32 = a as i32;
            let q_inv = -3327i16; // -3329^-1 mod 2^16 as signed 16-bit
            let t = (a_32 as i16).wrapping_mul(q_inv);
            let sub_term = (t as i32).wrapping_mul(3329);
            let u = a_32.wrapping_sub(sub_term) >> 16;
            let u_shifted = u + 3329; // now in [0, 2q-1]
            let val = Self::sub_pick(u_shifted as u32);
            Self { val }
        } else if Q == 8_380_417 {
            // ML-DSA Montgomery reduction
            let q_neg_inverse = 4_236_238_847u64;
            let a_low = (a as u64) & 0xffff_ffff;
            let t = a_low.wrapping_mul(q_neg_inverse) & 0xffff_ffff;
            let b = a.wrapping_add((t as i64).wrapping_mul(8_380_417));
            let c = b >> 32;
            let c_shifted = c + 8_380_417; // now in [0, 2q-1]
            let val = Self::sub_pick(c_shifted as u32);
            Self { val }
        } else {
            let rem = (a % (Q as i64) + (Q as i64)) % (Q as i64);
            Self { val: rem as u32 }
        }
    }

    /// Constant-time Barrett reduction.
    /// Maps a 32-bit/64-bit value to a representative modulo `Q`.
    /// For `Q = 3329` (ML-KEM): precomputed `v` = floor(2^26 / q) = 20158.
    /// For `Q = 8_380_417` (ML-DSA): precomputed `v` = floor(2^46 / q) = `8_396_807`.
    #[inline]
    #[must_use]
    #[allow(clippy::cast_possible_truncation)]
    pub const fn barrett_reduce(a: u64) -> Self {
        if Q == 3329 {
            let v = 20_158u64;
            let t = (a.wrapping_mul(v)) >> 26;
            let r = a.wrapping_sub(t.wrapping_mul(3329));
            let r1 = Self::sub_pick(r as u32);
            Self {
                val: Self::sub_pick(r1),
            }
        } else if Q == 8_380_417 {
            let v = 8_396_807u128;
            let t = ((a as u128).wrapping_mul(v)) >> 46;
            let r = (a as u128).wrapping_sub(t.wrapping_mul(8_380_417));
            let r1 = Self::sub_pick(r as u32);
            Self {
                val: Self::sub_pick(r1),
            }
        } else {
            Self {
                val: (a % (Q as u64)) as u32,
            }
        }
    }

    /// Multiply two field elements using Montgomery reduction on the product.
    #[inline]
    #[must_use]
    #[allow(clippy::cast_possible_truncation)]
    pub const fn mul(self, other: Self) -> Self {
        if Q == 3329 || Q == 8_380_417 {
            let prod = (self.val as i64).wrapping_mul(other.val as i64);
            Self::montgomery_reduce(prod)
        } else {
            Self {
                val: ((self.val as u64).wrapping_mul(other.val as u64) % (Q as u64)) as u32,
            }
        }
    }

    /// Multiply two field elements using standard modular multiplication (with Barrett reduction).
    #[inline]
    #[must_use]
    pub const fn mul_std(self, other: Self) -> Self {
        let prod = (self.val as u64).wrapping_mul(other.val as u64);
        Self::barrett_reduce(prod)
    }
}

#[cfg(test)]
#[allow(
    clippy::all,
    clippy::pedantic,
    clippy::cast_lossless,
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    clippy::unreadable_literal,
    clippy::many_single_char_names
)]
mod tests {
    use super::*;

    #[test]
    fn test_field_element_basics() {
        let a = FieldElement::<3329>::new(2000);
        let b = FieldElement::<3329>::new(1500);
        let c = a.add(b);
        assert_eq!(c.value(), 171);

        let d = a.sub(b);
        assert_eq!(d.value(), 500);

        let e = b.sub(a);
        assert_eq!(e.value(), 2829);
    }

    #[test]
    fn test_select() {
        let a = FieldElement::<3329>::new(100);
        let b = FieldElement::<3329>::new(200);
        assert_eq!(FieldElement::select(a, b, 0), a);
        assert_eq!(FieldElement::select(a, b, 1), b);
    }

    #[test]
    fn test_barrett_reduction() {
        for i in 0..10_000 {
            let val = i as u64;
            let r = FieldElement::<3329>::barrett_reduce(val);
            assert_eq!(r.value(), (val % 3329) as u32);
        }

        for i in 0..10_000 {
            let val = i as u64;
            let r = FieldElement::<8_380_417>::barrett_reduce(val);
            assert_eq!(r.value(), (val % 8_380_417) as u32);
        }
    }

    #[test]
    fn test_montgomery_reduction() {
        let r_inv_3329 = 169;
        for i in -5000..5000 {
            let val = i as i64;
            let res = FieldElement::<3329>::montgomery_reduce(val);
            let expected = (val * r_inv_3329).rem_euclid(3329) as u32;
            assert_eq!(res.value(), expected);
        }

        let r_inv_8380417_val = 8265825;
        for i in -5000..5000 {
            let val = i as i64;
            let res = FieldElement::<8_380_417>::montgomery_reduce(val);
            let expected = (val * r_inv_8380417_val).rem_euclid(8_380_417) as u32;
            assert_eq!(res.value(), expected);
        }
    }
}
