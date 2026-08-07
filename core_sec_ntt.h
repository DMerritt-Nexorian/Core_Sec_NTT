/*
 * Core_Sec_NTT - High-Security, Constant-Time FIPS 203 / FIPS 204 NTT Transformation Kernel.
 * This header declares the safe C-ABI compatible interface wrappers of the Core_Sec_NTT static library.
 * It enables C/C++ build pipelines to integrate and link our compiled static library without requiring raw Rust tools.
 */

#ifndef CORE_SEC_NTT_H
#define CORE_SEC_NTT_H

#ifdef __cplusplus
extern "C" {
#endif

#include <stdint.h>

/**
 * @brief Performs Forward NTT for ML-KEM (Q = 3329) on exactly 256 coefficients in-place.
 *
 * @param poly_coeffs Array of exactly 256 u32 coefficients to be transformed in-place.
 */
void safe_ntt_forward_kem(uint32_t poly_coeffs[256]);

/**
 * @brief Performs Inverse NTT for ML-KEM (Q = 3329) on exactly 256 coefficients in-place.
 *
 * @param poly_coeffs Array of exactly 256 u32 coefficients to be transformed in-place.
 */
void safe_ntt_inverse_kem(uint32_t poly_coeffs[256]);

/**
 * @brief Performs Forward NTT for ML-DSA (Q = 8380417) on exactly 256 coefficients in-place.
 *
 * @param poly_coeffs Array of exactly 256 u32 coefficients to be transformed in-place.
 */
void safe_ntt_forward_dsa(uint32_t poly_coeffs[256]);

/**
 * @brief Performs Inverse NTT for ML-DSA (Q = 8380417) on exactly 256 coefficients in-place.
 *
 * @param poly_coeffs Array of exactly 256 u32 coefficients to be transformed in-place.
 */
void safe_ntt_inverse_dsa(uint32_t poly_coeffs[256]);

/**
 * @brief Pointwise modular multiplication of two polynomials in the NTT representation for ML-KEM.
 *
 * @param a_coeffs Left-hand side polynomial coefficients array (exactly 256 elements).
 * @param b_coeffs Right-hand side polynomial coefficients array (exactly 256 elements).
 * @param res_coeffs Output polynomial coefficients array (exactly 256 elements).
 */
void safe_pointwise_mul_kem(const uint32_t a_coeffs[256], const uint32_t b_coeffs[256], uint32_t res_coeffs[256]);

/**
 * @brief Pointwise modular multiplication of two polynomials in the NTT representation for ML-DSA.
 *
 * @param a_coeffs Left-hand side polynomial coefficients array (exactly 256 elements).
 * @param b_coeffs Right-hand side polynomial coefficients array (exactly 256 elements).
 * @param res_coeffs Output polynomial coefficients array (exactly 256 elements).
 */
void safe_pointwise_mul_dsa(const uint32_t a_coeffs[256], const uint32_t b_coeffs[256], uint32_t res_coeffs[256]);

#ifdef __cplusplus
}
#endif

#endif /* CORE_SEC_NTT_H */
