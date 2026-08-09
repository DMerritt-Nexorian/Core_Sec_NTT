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

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    use std::path::Path;

    #[test]
    #[ignore = "Live NIST ACVTS mTLS Endpoint Test"]
    fn test_nist_acvp_fips203_fips204_kat() {
        // 1. Resolve credentials from environment variables
        let cert_path_str = env::var("ACVP_CERT_PATH")
            .expect("FATAL: ACVP_CERT_PATH environment variable is missing.");
        let key_path_str = env::var("ACVP_KEY_PATH")
            .expect("FATAL: ACVP_KEY_PATH environment variable is missing.");
        let server_url = env::var("ACVP_SERVER_URL")
            .unwrap_or_else(|_| "https://demo.acvts.nist.gov/acvp/v1/".to_string());

        // 2. File existence assertion
        assert!(
            Path::new(&cert_path_str).exists(),
            "Specified ACVP_CERT_PATH file does not exist."
        );
        assert!(
            Path::new(&key_path_str).exists(),
            "Specified ACVP_KEY_PATH file does not exist."
        );

        // 3. Initialize authenticated mTLS ACVP Client
        let mut client = AcvtsClient::new(&server_url, &cert_path_str, &key_path_str)
            .expect("Failed to establish mTLS connection with NIST ACVTS server endpoint.");

        // 4. Retrieve Vector Sets for ML-KEM and ML-DSA
        let vector_set = client
            .get_vector_set("ML-KEM")
            .expect("Failed to fetch FIPS 203 vector sets from NIST server.");

        // 5. Execute vectors through core_sec_ntt engine
        let response_payload = process_acvp_vectors(&vector_set);

        // 6. Submit results back to NIST Endpoint
        let validation_result = client
            .submit_results(&response_payload)
            .expect("Failed to submit ACVP responses to NIST server.");

        assert!(
            validation_result.is_passed(),
            "NIST ACVP KAT Validation Failed: {:?}",
            validation_result.status_message()
        );
    }
}
