/*
 * Copyright (C) 2026 NexusCorps / Dennis W. Merritt. All Rights Reserved.
 *
 * Proprietary and Confidential.
 * Authorized for use solely under evaluation terms.
 */

#![no_std]
#![deny(unsafe_code)]
#![deny(clippy::pedantic)]
#![allow(clippy::unreadable_literal)]

use core_sec_field::FieldElement;

/// Polynomial wrapper containing exactly 256 coefficients of modular field elements.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Polynomial<const Q: u32>(pub [FieldElement<Q>; 256]);

impl<const Q: u32> Polynomial<Q> {
    /// Creates a new polynomial from coefficients.
    #[inline]
    #[must_use]
    pub const fn new(coeffs: [FieldElement<Q>; 256]) -> Self {
        Self(coeffs)
    }

    /// Creates a zero polynomial.
    #[inline]
    #[must_use]
    pub const fn zero() -> Self {
        Self([FieldElement::new(0); 256])
    }
}

/// Secure zeroization on drop for intermediate polynomial buffers.
impl<const Q: u32> Drop for Polynomial<Q> {
    #[inline]
    fn drop(&mut self) {
        let mut i = 0;
        while i < 256 {
            self.0[i] = FieldElement::new(0);
            core::hint::black_box(&mut self.0[i]);
            i += 1;
        }
    }
}

/// Precomputed bit-reversed twiddle factors for ML-KEM (q = 3329, n = 256, \omega = 17).
/// Zetas[i] = 17^BitRev7(i) mod 3329.
pub const KEM_ZETAS: [FieldElement<3329>; 128] = [
    FieldElement::new(1),
    FieldElement::new(1729),
    FieldElement::new(2580),
    FieldElement::new(3289),
    FieldElement::new(2642),
    FieldElement::new(630),
    FieldElement::new(1897),
    FieldElement::new(848),
    FieldElement::new(1062),
    FieldElement::new(1919),
    FieldElement::new(193),
    FieldElement::new(797),
    FieldElement::new(2786),
    FieldElement::new(3260),
    FieldElement::new(569),
    FieldElement::new(1746),
    FieldElement::new(296),
    FieldElement::new(2447),
    FieldElement::new(1339),
    FieldElement::new(1476),
    FieldElement::new(3046),
    FieldElement::new(56),
    FieldElement::new(2240),
    FieldElement::new(1333),
    FieldElement::new(1426),
    FieldElement::new(2094),
    FieldElement::new(535),
    FieldElement::new(2882),
    FieldElement::new(2393),
    FieldElement::new(2879),
    FieldElement::new(1974),
    FieldElement::new(821),
    FieldElement::new(289),
    FieldElement::new(331),
    FieldElement::new(3253),
    FieldElement::new(1756),
    FieldElement::new(1197),
    FieldElement::new(2304),
    FieldElement::new(2277),
    FieldElement::new(2055),
    FieldElement::new(650),
    FieldElement::new(1977),
    FieldElement::new(2513),
    FieldElement::new(632),
    FieldElement::new(2865),
    FieldElement::new(33),
    FieldElement::new(1320),
    FieldElement::new(1915),
    FieldElement::new(2319),
    FieldElement::new(1435),
    FieldElement::new(807),
    FieldElement::new(452),
    FieldElement::new(1438),
    FieldElement::new(2868),
    FieldElement::new(1534),
    FieldElement::new(2402),
    FieldElement::new(2647),
    FieldElement::new(2617),
    FieldElement::new(1481),
    FieldElement::new(648),
    FieldElement::new(2474),
    FieldElement::new(3110),
    FieldElement::new(1227),
    FieldElement::new(910),
    FieldElement::new(17),
    FieldElement::new(2761),
    FieldElement::new(583),
    FieldElement::new(2649),
    FieldElement::new(1637),
    FieldElement::new(723),
    FieldElement::new(2288),
    FieldElement::new(1100),
    FieldElement::new(1409),
    FieldElement::new(2662),
    FieldElement::new(3281),
    FieldElement::new(233),
    FieldElement::new(756),
    FieldElement::new(2156),
    FieldElement::new(3015),
    FieldElement::new(3050),
    FieldElement::new(1703),
    FieldElement::new(1651),
    FieldElement::new(2789),
    FieldElement::new(1789),
    FieldElement::new(1847),
    FieldElement::new(952),
    FieldElement::new(1461),
    FieldElement::new(2687),
    FieldElement::new(939),
    FieldElement::new(2308),
    FieldElement::new(2437),
    FieldElement::new(2388),
    FieldElement::new(733),
    FieldElement::new(2337),
    FieldElement::new(268),
    FieldElement::new(641),
    FieldElement::new(1584),
    FieldElement::new(2298),
    FieldElement::new(2037),
    FieldElement::new(3220),
    FieldElement::new(375),
    FieldElement::new(2549),
    FieldElement::new(2090),
    FieldElement::new(1645),
    FieldElement::new(1063),
    FieldElement::new(319),
    FieldElement::new(2773),
    FieldElement::new(757),
    FieldElement::new(2099),
    FieldElement::new(561),
    FieldElement::new(2466),
    FieldElement::new(2594),
    FieldElement::new(2804),
    FieldElement::new(1092),
    FieldElement::new(403),
    FieldElement::new(1026),
    FieldElement::new(1143),
    FieldElement::new(2150),
    FieldElement::new(2775),
    FieldElement::new(886),
    FieldElement::new(1722),
    FieldElement::new(1212),
    FieldElement::new(1874),
    FieldElement::new(1029),
    FieldElement::new(2110),
    FieldElement::new(2935),
    FieldElement::new(885),
    FieldElement::new(2154),
];

/// Precomputed bit-reversed twiddle factors for ML-DSA (q = 8380417, n = 256, \omega = 1753).
pub const DSA_ZETAS: [FieldElement<8380417>; 256] = [
    FieldElement::new(1),
    FieldElement::new(4808194),
    FieldElement::new(3765607),
    FieldElement::new(3761513),
    FieldElement::new(5178923),
    FieldElement::new(5496691),
    FieldElement::new(5234739),
    FieldElement::new(5178987),
    FieldElement::new(7778734),
    FieldElement::new(3542485),
    FieldElement::new(2682288),
    FieldElement::new(2129892),
    FieldElement::new(3764867),
    FieldElement::new(7375178),
    FieldElement::new(557458),
    FieldElement::new(7159240),
    FieldElement::new(5010068),
    FieldElement::new(4317364),
    FieldElement::new(2663378),
    FieldElement::new(6705802),
    FieldElement::new(4855975),
    FieldElement::new(7946292),
    FieldElement::new(676590),
    FieldElement::new(7044481),
    FieldElement::new(5152541),
    FieldElement::new(1714295),
    FieldElement::new(2453983),
    FieldElement::new(1460718),
    FieldElement::new(7737789),
    FieldElement::new(4795319),
    FieldElement::new(2815639),
    FieldElement::new(2283733),
    FieldElement::new(3602218),
    FieldElement::new(3182878),
    FieldElement::new(2740543),
    FieldElement::new(4793971),
    FieldElement::new(5269599),
    FieldElement::new(2101410),
    FieldElement::new(3704823),
    FieldElement::new(1159875),
    FieldElement::new(394148),
    FieldElement::new(928749),
    FieldElement::new(1095468),
    FieldElement::new(4874037),
    FieldElement::new(2071829),
    FieldElement::new(4361428),
    FieldElement::new(3241972),
    FieldElement::new(2156050),
    FieldElement::new(3415069),
    FieldElement::new(1759347),
    FieldElement::new(7562881),
    FieldElement::new(4805951),
    FieldElement::new(3756790),
    FieldElement::new(6444618),
    FieldElement::new(6663429),
    FieldElement::new(4430364),
    FieldElement::new(5483103),
    FieldElement::new(3192354),
    FieldElement::new(556856),
    FieldElement::new(3870317),
    FieldElement::new(2917338),
    FieldElement::new(1853806),
    FieldElement::new(3345963),
    FieldElement::new(1858416),
    FieldElement::new(3073009),
    FieldElement::new(1277625),
    FieldElement::new(5744944),
    FieldElement::new(3852015),
    FieldElement::new(4183372),
    FieldElement::new(5157610),
    FieldElement::new(5258977),
    FieldElement::new(8106357),
    FieldElement::new(2508980),
    FieldElement::new(2028118),
    FieldElement::new(1937570),
    FieldElement::new(4564692),
    FieldElement::new(2811291),
    FieldElement::new(5396636),
    FieldElement::new(7270901),
    FieldElement::new(4158088),
    FieldElement::new(1528066),
    FieldElement::new(482649),
    FieldElement::new(1148858),
    FieldElement::new(5418153),
    FieldElement::new(7814814),
    FieldElement::new(169688),
    FieldElement::new(2462444),
    FieldElement::new(5046034),
    FieldElement::new(4213992),
    FieldElement::new(4892034),
    FieldElement::new(1987814),
    FieldElement::new(5183169),
    FieldElement::new(1736313),
    FieldElement::new(235407),
    FieldElement::new(5130263),
    FieldElement::new(3258457),
    FieldElement::new(5801164),
    FieldElement::new(1787943),
    FieldElement::new(5989328),
    FieldElement::new(6125690),
    FieldElement::new(3482206),
    FieldElement::new(4197502),
    FieldElement::new(7080401),
    FieldElement::new(6018354),
    FieldElement::new(7062739),
    FieldElement::new(2461387),
    FieldElement::new(3035980),
    FieldElement::new(621164),
    FieldElement::new(3901472),
    FieldElement::new(7153756),
    FieldElement::new(2925816),
    FieldElement::new(3374250),
    FieldElement::new(1356448),
    FieldElement::new(5604662),
    FieldElement::new(2683270),
    FieldElement::new(5601629),
    FieldElement::new(4912752),
    FieldElement::new(2312838),
    FieldElement::new(7727142),
    FieldElement::new(7921254),
    FieldElement::new(348812),
    FieldElement::new(8052569),
    FieldElement::new(1011223),
    FieldElement::new(6026202),
    FieldElement::new(4561790),
    FieldElement::new(6458164),
    FieldElement::new(6143691),
    FieldElement::new(1744507),
    FieldElement::new(1753),
    FieldElement::new(6444997),
    FieldElement::new(5720892),
    FieldElement::new(6924527),
    FieldElement::new(2660408),
    FieldElement::new(6600190),
    FieldElement::new(8321269),
    FieldElement::new(2772600),
    FieldElement::new(1182243),
    FieldElement::new(87208),
    FieldElement::new(636927),
    FieldElement::new(4415111),
    FieldElement::new(4423672),
    FieldElement::new(6084020),
    FieldElement::new(5095502),
    FieldElement::new(4663471),
    FieldElement::new(8352605),
    FieldElement::new(822541),
    FieldElement::new(1009365),
    FieldElement::new(5926272),
    FieldElement::new(6400920),
    FieldElement::new(1596822),
    FieldElement::new(4423473),
    FieldElement::new(4620952),
    FieldElement::new(6695264),
    FieldElement::new(4969849),
    FieldElement::new(2678278),
    FieldElement::new(4611469),
    FieldElement::new(4829411),
    FieldElement::new(635956),
    FieldElement::new(8129971),
    FieldElement::new(5925040),
    FieldElement::new(4234153),
    FieldElement::new(6607829),
    FieldElement::new(2192938),
    FieldElement::new(6653329),
    FieldElement::new(2387513),
    FieldElement::new(4768667),
    FieldElement::new(8111961),
    FieldElement::new(5199961),
    FieldElement::new(3747250),
    FieldElement::new(2296099),
    FieldElement::new(1239911),
    FieldElement::new(4541938),
    FieldElement::new(3195676),
    FieldElement::new(2642980),
    FieldElement::new(1254190),
    FieldElement::new(8368000),
    FieldElement::new(2998219),
    FieldElement::new(141835),
    FieldElement::new(8291116),
    FieldElement::new(2513018),
    FieldElement::new(7025525),
    FieldElement::new(613238),
    FieldElement::new(7070156),
    FieldElement::new(6161950),
    FieldElement::new(7921677),
    FieldElement::new(6458423),
    FieldElement::new(4040196),
    FieldElement::new(4908348),
    FieldElement::new(2039144),
    FieldElement::new(6500539),
    FieldElement::new(7561656),
    FieldElement::new(6201452),
    FieldElement::new(6757063),
    FieldElement::new(2105286),
    FieldElement::new(6006015),
    FieldElement::new(6346610),
    FieldElement::new(586241),
    FieldElement::new(7200804),
    FieldElement::new(527981),
    FieldElement::new(5637006),
    FieldElement::new(6903432),
    FieldElement::new(1994046),
    FieldElement::new(2491325),
    FieldElement::new(6987258),
    FieldElement::new(507927),
    FieldElement::new(7192532),
    FieldElement::new(7655613),
    FieldElement::new(6545891),
    FieldElement::new(5346675),
    FieldElement::new(8041997),
    FieldElement::new(2647994),
    FieldElement::new(3009748),
    FieldElement::new(5767564),
    FieldElement::new(4148469),
    FieldElement::new(749577),
    FieldElement::new(4357667),
    FieldElement::new(3980599),
    FieldElement::new(2569011),
    FieldElement::new(6764887),
    FieldElement::new(1723229),
    FieldElement::new(1665318),
    FieldElement::new(2028038),
    FieldElement::new(1163598),
    FieldElement::new(5011144),
    FieldElement::new(3994671),
    FieldElement::new(8368538),
    FieldElement::new(7009900),
    FieldElement::new(3020393),
    FieldElement::new(3363542),
    FieldElement::new(214880),
    FieldElement::new(545376),
    FieldElement::new(7609976),
    FieldElement::new(3105558),
    FieldElement::new(7277073),
    FieldElement::new(508145),
    FieldElement::new(7826699),
    FieldElement::new(860144),
    FieldElement::new(3430436),
    FieldElement::new(140244),
    FieldElement::new(6866265),
    FieldElement::new(6195333),
    FieldElement::new(3123762),
    FieldElement::new(2358373),
    FieldElement::new(6187330),
    FieldElement::new(5365997),
    FieldElement::new(6663603),
    FieldElement::new(2926054),
    FieldElement::new(7987710),
    FieldElement::new(8077412),
    FieldElement::new(3531229),
    FieldElement::new(4405932),
    FieldElement::new(4606686),
    FieldElement::new(1900052),
    FieldElement::new(7598542),
    FieldElement::new(1054478),
    FieldElement::new(7648983),
];

/// In-place Cooley-Tukey Forward NTT transformation kernel for ML-KEM.
#[inline]
pub fn ntt_forward_kem(poly: &mut Polynomial<3329>) {
    let mut i = 1;
    let mut length = 128;
    while length >= 2 {
        let mut start = 0;
        while start < 256 {
            let zeta = KEM_ZETAS[i];
            i += 1;
            let mut j = start;
            let end_j = start + length;
            while j < end_j {
                let t = zeta.mul_std(poly.0[j + length]);
                poly.0[j + length] = poly.0[j].sub(t);
                poly.0[j] = poly.0[j].add(t);
                j += 1;
            }
            start += 2 * length;
        }
        length /= 2;
    }
}

/// In-place Cooley-Tukey Forward NTT transformation kernel for ML-DSA.
#[inline]
pub fn ntt_forward_dsa(poly: &mut Polynomial<8380417>) {
    let mut i = 1;
    let mut length = 128;
    while length >= 1 {
        let mut start = 0;
        while start < 256 {
            let zeta = DSA_ZETAS[i];
            i += 1;
            let mut j = start;
            let end_j = start + length;
            while j < end_j {
                let t = zeta.mul_std(poly.0[j + length]);
                poly.0[j + length] = poly.0[j].sub(t);
                poly.0[j] = poly.0[j].add(t);
                j += 1;
            }
            start += 2 * length;
        }
        length /= 2;
    }
}

/// In-place Gentleman-Sande Inverse NTT transformation kernel for ML-KEM.
#[inline]
pub fn ntt_inverse_kem(poly: &mut Polynomial<3329>) {
    let mut i = 127;
    let mut length = 2;
    while length <= 128 {
        let mut start = 0;
        while start < 256 {
            let zeta = KEM_ZETAS[i];
            i -= 1;
            let mut j = start;
            let end_j = start + length;
            while j < end_j {
                let u = poly.0[j];
                let v = poly.0[j + length];
                poly.0[j] = u.add(v);
                poly.0[j + length] = v.sub(u).mul_std(zeta);
                j += 1;
            }
            start += 2 * length;
        }
        length *= 2;
    }

    // Final normalization scale factor: 3303 (128^-1 mod 3329)
    let scale_factor = FieldElement::<3329>::new(3303);
    let mut j = 0;
    while j < 256 {
        poly.0[j] = poly.0[j].mul_std(scale_factor);
        j += 1;
    }
}

/// In-place Gentleman-Sande Inverse NTT transformation kernel for ML-DSA.
#[inline]
pub fn ntt_inverse_dsa(poly: &mut Polynomial<8380417>) {
    let mut i = 255;
    let mut length = 1;
    while length <= 128 {
        let mut start = 0;
        while start < 256 {
            let zeta = DSA_ZETAS[i];
            i -= 1;
            let mut j = start;
            let end_j = start + length;
            while j < end_j {
                let u = poly.0[j];
                let v = poly.0[j + length];
                poly.0[j] = u.add(v);
                poly.0[j + length] = v.sub(u).mul_std(zeta);
                j += 1;
            }
            start += 2 * length;
        }
        length *= 2;
    }

    // Final normalization scale factor: 8347681 (256^-1 mod 8380417)
    let scale_factor = FieldElement::<8380417>::new(8347681);
    let mut j = 0;
    while j < 256 {
        poly.0[j] = poly.0[j].mul_std(scale_factor);
        j += 1;
    }
}

/// Pointwise modular multiplication of two polynomials in the NTT representation:
/// `res_i = (a_i * b_i) mod Q` in O(N) constant time.
#[inline]
pub fn ntt_pointwise_mul<const Q: u32>(
    a: &Polynomial<Q>,
    b: &Polynomial<Q>,
    res: &mut Polynomial<Q>,
) {
    let mut i = 0;
    while i < 256 {
        res.0[i] = a.0[i].mul_std(b.0[i]);
        i += 1;
    }
}

#[cfg(test)]
#[allow(clippy::all, clippy::pedantic, clippy::cast_possible_truncation)]
mod tests {
    use super::*;

    #[test]
    fn test_ntt_roundtrip_kem() {
        let mut original = Polynomial::<3329>::zero();
        for i in 0..256 {
            original.0[i] = FieldElement::new((i * 13 + 5) as u32 % 3329);
        }

        let mut transformed = original.clone();
        ntt_forward_kem(&mut transformed);
        assert_ne!(original, transformed);

        ntt_inverse_kem(&mut transformed);
        assert_eq!(original, transformed);
    }

    #[test]
    fn test_ntt_roundtrip_dsa() {
        let mut original = Polynomial::<8380417>::zero();
        for i in 0..256 {
            original.0[i] = FieldElement::new((i * 101 + 17) as u32 % 8380417);
        }

        let mut transformed = original.clone();
        ntt_forward_dsa(&mut transformed);
        assert_ne!(original, transformed);

        ntt_inverse_dsa(&mut transformed);
        assert_eq!(original, transformed);
    }

    #[test]
    fn test_pointwise_multiplication() {
        let mut a = Polynomial::<3329>::zero();
        let mut b = Polynomial::<3329>::zero();
        for i in 0..256 {
            a.0[i] = FieldElement::new(2);
            b.0[i] = FieldElement::new(3);
        }

        let mut res = Polynomial::<3329>::zero();
        ntt_pointwise_mul(&a, &b, &mut res);

        for i in 0..256 {
            assert_eq!(res.0[i].value(), 6);
        }
    }
}
