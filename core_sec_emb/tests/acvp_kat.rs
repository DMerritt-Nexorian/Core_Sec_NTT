/*
 * Copyright (C) 2026 NexusCorps / Dennis W. Merritt. All Rights Reserved.
 *
 * Proprietary and Confidential.
 * Authorized for use solely under evaluation terms.
 */

use core_sec_field::FieldElement;
use core_sec_ntt::{Polynomial, ntt_forward_kem, ntt_inverse_kem};

#[test]
fn test_acvp_kat_vectors_fips_203() {
    // Read and parse the ACVP test vectors JSON data
    let json_data = include_str!("acvp_vectors.json");
    let suite: serde_json::Value = serde_json::from_str(json_data).unwrap();

    if let Some(groups) = suite.get("testGroups") {
        if let Some(arr) = groups.as_array() {
            for group in arr {
                if let Some(tests) = group.get("tests") {
                    if let Some(test_arr) = tests.as_array() {
                        for test_case in test_arr {
                            // Extract input and expectedForward arrays of 256 elements
                            let input_arr: Vec<u32> = test_case
                                .get("input")
                                .unwrap()
                                .as_array()
                                .unwrap()
                                .iter()
                                .map(|v| v.as_u64().unwrap() as u32)
                                .collect();

                            let expected_arr: Vec<u32> = test_case
                                .get("expectedForward")
                                .unwrap()
                                .as_array()
                                .unwrap()
                                .iter()
                                .map(|v| v.as_u64().unwrap() as u32)
                                .collect();

                            let mut poly = Polynomial::<3329>::zero();
                            for i in 0..256 {
                                poly.0[i] = FieldElement::new(input_arr[i] % 3329);
                            }

                            // Perform NTT Forward transformation
                            ntt_forward_kem(&mut poly);

                            // Assert bit-exact expected values matching NIST ACVP vectors
                            for i in 0..256 {
                                assert_eq!(poly.0[i].value(), expected_arr[i] % 3329);
                            }

                            // Perform NTT Inverse transformation
                            ntt_inverse_kem(&mut poly);

                            // Verify exact roundtrip
                            for i in 0..256 {
                                assert_eq!(poly.0[i].value(), input_arr[i] % 3329);
                            }
                        }
                    }
                }
            }
        }
    }
}
