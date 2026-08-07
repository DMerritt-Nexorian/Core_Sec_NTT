Core_Sec_NTT
> Proprietary Post-Quantum Cryptographic Hardware & Acceleration Engine
> NIST FIPS 203 (ML-KEM) & FIPS 204 (ML-DSA) NTT Core
> 
Proprietary Software License Agreement
NEXUSCORPS ALL RIGHTS RESERVED LICENSE
Copyright (c) 2026 Dennis W. Merritt / NexusCorps. All Rights Reserved.
 * Entity: NexusCorps
 * Author / Owner: Dennis W. Merritt
 * Contact: Dennis.Merritt@zohomail.com
 * Target Core: Core_Sec_NTT
Important Notice & Legal Terms
This software, source code, formal verification proofs, binary artifacts, and associated documentation ("Software") are the exclusive proprietary property of Dennis W. Merritt and NexusCorps.
This Software is protected by copyright laws, international copyright treaties, and proprietary trade secret laws. THIS IS NOT OPEN-SOURCE OR PUBLIC DOMAIN SOFTWARE (CC0, MIT, Apache, or similar open-source licenses do NOT apply).
Restrictions on Use & Distribution
 * No Copying: Unlawful copying, duplicating, or reproducing of this Software, in whole or in part, in source or compiled form, is strictly prohibited.
 * No Resale or Commercialization: You may not sell, resell, lease, rent, sub-license, host, distribute, assign, or otherwise commercially exploit or transfer this Software to any third party.
 * No Derivative Works: You may not modify, adapt, translate, reverse-engineer, decompile, disassemble, or derive source code or cryptographic primitives from this Software without explicit written permission.
 * Reservation of Rights: NexusCorps and Dennis W. Merritt retain all legal titles, ownership, patents, intellectual property rights, and commercial exploitation rights in and to the Core_Sec_NTT implementation.
> Written Consent Required: Any deployment, commercial integration, distribution, or licensing of this Software requires prior explicit, written authorization directly from Dennis W. Merritt (Dennis.Merritt@zohomail.com).
> 
Technical Overview & Architectural Guarantees
Core_Sec_NTT is a high-performance, zero-allocation Number Theoretic Transform (NTT) execution engine engineered for post-quantum cryptographic primitives.
Core Architectural Invariants
 * Proof Before Trust: Zero trust in compiler auto-vectorization, timing invariance, or unverified memory access. All NTT butterfly operations and modular polynomial arithmetic are formally proven panic-free, overflow-free, and constant-time using Kani bounded model checking. Static type-level bounds checks are applied prior to execution.
 * Strict #![no_std] & #![deny(unsafe_code)] Execution: Built with 100% safe Rust primitives operating without standard library dependencies, heap allocations, or dynamic branching dependent on secret data.
 * NIST FIPS 203 & 204 Compliance: Exact mathematical compliance with ML-KEM (q=3329, n=256) and ML-DSA (q=8380417, n=256) specifications. Montgomery and Barrett reduction invariants (0 \le r < q) hold strictly across all execution stages.
 * Side-Channel Hardening: Execution complexity is strictly \mathcal{O}(N \log N) with 0-cycle timing variance across secret coefficient inputs, eliminating side-channel leakage.
Crate Architecture & Module Layout
The workspace is organized into modular core crates targeting constrained embedded enclaves (ARM Cortex-M4/M7, RISC-V RV32IMAC):
 * core_sec_field — Constant-Time Modular Arithmetic Engine
   * Implements branchless reduction routines (sub_pick, montgomery_reduce, barrett_reduce) using bitwise masking and constant-time selection logic.
   * Field arithmetic defined for ML-KEM (q = 3329, n = 256, \omega = 17) and ML-DSA (q = 8380417, n = 256, \omega = 1753).
   * Exposes compile-time bounded wrapper types (FieldElement<const Q: u32>).
 * core_sec_ntt — High-Speed NTT / INTT Transformation Kernel
   * Forward NTT via Cooley-Tukey butterfly mapping A(X) \in R_q to point-value representation.
   * Inverse NTT via Gentleman-Sande butterfly returning to polynomial representation with exact division by n in \mathbb{F}_q.
   * Pointwise modular multiplication (C_i = (A_i \times B_i) \bmod q) in \mathcal{O}(N) constant time.
   * Pre-computed bit-reversed twiddle factor lookup tables embedded directly in static .rodata memory.
 * core_sec_verifier — Side-Channel & Memory Safety Invariant Engine
   * Verifies zero conditional jumps or memory indexing operations tied to secret inputs.
   * Enforces static stack bounds (< 2.0\text{ KB} RAM limit per NTT execution frame).
   * Automatically applies zeroize-on-drop semantics for intermediate buffers and key frame state.
Configuration & Quality Assurance
Optimization Profile (Cargo.toml)
[profile.release]
opt-level = 3
lto = true
codegen-units = 1
panic = "abort"

Mandated Top-Level Lints
#![no_std]
#![deny(unsafe_code)]
#![deny(clippy::pedantic)]

Verification & CI/CD Pipeline
 * Formal Verification: Kani proofs (proof_montgomery_reduction_bounds, proof_ntt_forward_inverse_identity, proof_constant_time_execution) ensure strict mathematical safety and timing invariance.
 * NIST Validation: Tested against official NIST ACVP Known Answer Test (KAT) vectors for FIPS 203 and FIPS 204.
 * Cross-Compilation: Validated target builds for thumbv7em-none-eabi and riscv32imac-unknown-none-elf.
