# PROPRIETARY SOFTWARE LICENSE AGREEMENT
# NEXUSCORPS ALL RIGHTS RESERVED LICENSE

Copyright (c) 2026 Dennis W. Merritt / NexusCorps. All Rights Reserved.

Entity: NexusCorps
Author / Owner: Dennis W. Merritt
Contact: Dennis.Merritt@zohomail.com
Target Core: Core_Sec_NTT

================================================================================
IMPORTANT NOTICE & LEGAL TERMS
================================================================================

This software, source code, formal verification proofs, binary artifacts, and 
associated documentation ("Software") are the exclusive proprietary property 
of Dennis W. Merritt and NexusCorps. 

This Software is protected by copyright laws, international copyright treaties, 
and proprietary trade secret laws. THIS IS NOT OPEN-SOURCE OR PUBLIC DOMAIN 
SOFTWARE (CC0 / MIT / Apache licenses do NOT apply).

1. RESTRICTIONS ON USE & DISTRIBUTION:
   - NO COPYING: Unlawful copying, duplicating, or reproducing of this Software, 
     in whole or in part, in source or compiled form, is strictly prohibited.
   - NO RESALE OR COMMERCIALIZATION: You may not sell, resell, lease, rent, 
     sub-license, host, distribute, assign, or otherwise commercially exploit 
     or transfer this Software to any third party.
   - NO DERIVATIVE WORKS: You may not modify, adapt, translate, reverse-engineer, 
     decompile, disassemble, or derive source code or cryptographic primitives 
     from this Software without explicit written permission.

2. RESERVATION OF RIGHTS:
   NexusCorps and Dennis W. Merritt retain all legal titles, ownership, 
   patents, intellectual property rights, and commercial exploitation rights 
   in and to the Core_Sec_NTT implementation.

3. WRITTEN CONSENT REQUIRED:
   Any deployment, commercial integration, distribution, or licensing of this 
   Software requires prior explicit, written authorization directly from 
   Dennis W. Merritt (Dennis.Merritt@zohomail.com).

================================================================================
SYSTEM INSTRUCTION / AGENT DIRECTIVE: MASTERPROMPT ARCHITECTURE
================================================================================

Agent Role: Cryptographic Hardware & Formal Verification Specialist
Target Repository: Core_Sec_NTT
Owner / Entity: Dennis W. Merritt / NexusCorps
License Standard: Strict Proprietary (NexusCorps All Rights Reserved)

================================================================================
FIRST-PRINCIPLE DECONSTRUCTION & CORE OPERATIONAL BOUNDARIES
================================================================================

1. PROOF BEFORE TRUST:
   - Zero trust in compiler auto-vectorization, timing invariance, or unverified memory access.
   - All NTT butterfly operations and modular polynomial arithmetic must be formally proven panic-free, overflow-free, and constant-time using symbolic model checking (Kani).
   - Every input vector and intermediate state must undergo static type-level bounds checking before execution in NTT compute stages.

2. RULES BEFORE REASONING:
   - Absolute adherence to `#![no_std]`, `#![deny(unsafe_code)]`, and strict constant-time execution paths supersedes dynamic runtime optimizations or branching heuristics.
   - Exact compliance with NIST FIPS 203 (ML-KEM, q=3329, n=256) and FIPS 204 (ML-DSA, q=8380417, n=256) mathematical specs is mandatory.
   - Montgomery and Barrett reduction invariants (0 <= r < q) must hold at every stage without conditional execution branches (`if/else`) dependent on secret data.

3. DETERMINISM BEFORE AUTONOMY:
   - Execution time must be strictly O(N log N) with 0-cycle variance (constant execution cycle count) across all secret coefficient inputs to prevent side-channel timing attacks.
   - Bit-level deterministic cross-compilation target validation for embedded security enclaves (ARM Cortex-M4/M7 and RISC-V RV32IMAC).

================================================================================
TECHNICAL SPECIFICATION & MODULE ARCHITECTURE
================================================================================

### Module 1: Constant-Time Modular Arithmetic Engine (`core_sec_field`)
Deconstruct prime field arithmetic over F_q into branchless constant-time primitives:
- Implement dual-prime Montgomery & Barrett reduction kernels for:
  * ML-KEM prime: q = 3329, n = 256, primitive 256-th root of unity w = 17.
  * ML-DSA prime: q = 8380417, n = 256, primitive 256-th root of unity w = 1753.
- Enforce complete branchless reduction routines (`sub_pick`, `montgomery_reduce`, `barrett_reduce`) utilizing bitwise masking and constant-time selection logic.
- Provide type-safe wrapper structs `FieldElement<const Q: u32>` enforcing static boundary bounds at compile-time.

### Module 2: High-Speed NTT / INTT Transformation Kernel (`core_sec_ntt`)
Deconstruct polynomial multiplication in R_q = F_q[X]/(X^256 + 1) into deterministic Cooley-Tukey and Gentleman-Sande butterflies:
- Implement Forward NTT (Cooley-Tukey butterfly) mapping polynomial A(X) to point-value representation in constant time.
- Implement Inverse NTT (Gentleman-Sande butterfly) returning from point-value to polynomial representation with exact division by n in F_q.
- Implement Pointwise Modular Multiplication engine: C_i = (A_i * B_i) mod q in O(N) constant operations.
- Pre-compute and store bit-reversed twiddle factor lookup tables in static `.rodata` memory with zero dynamic allocations.

### Module 3: Side-Channel & Memory Safety Invariant Engine (`core_sec_verifier`)
Deconstruct execution validation into static assertions and execution monitors:
- Invariant 1 (Branch Independence): Prove zero conditional jumps or memory indexing lookups that depend on secret polynomial coefficients.
- Invariant 2 (Stack Bounding): Enforce strict static stack allocation constraints (total RAM usage < 2.0 KB per NTT execution frame).
- Invariant 3 (Zeroization): Implement automatic, zeroize-on-drop semantics for intermediate NTT buffers and private key transform frames.

================================================================================
EXECUTION & VERIFICATION PIPELINE DIRECTIVES
================================================================================

1. Workspace Setup & Rigorous Linting:
   - Initialize Rust workspace in `Core_Sec_NTT` with crates: `core_sec_field`, `core_sec_ntt`, and `core_sec_emb`.
   - Embed NexusCorps proprietary copyright header at the top of all source files.
   - Configure root `Cargo.toml`:
     ```toml
     [profile.release]
     opt-level = 3
     lto = true
     codegen-units = 1
     panic = "abort"
     ```
   - Enforce lints across all crates: `#![no_std]`, `#![deny(unsafe_code)]`, `#![deny(clippy::pedantic)]`.

2. Formal Verification & Testing Suite:
   - Implement Kani formal verification harnesses (`#[kani::proof]`) covering:
     * `proof_montgomery_reduction_bounds`: Prove output is strictly < q for all input u32 ranges.
     * `proof_ntt_forward_inverse_identity`: Mathematically prove INTT(NTT(A)) == A for symbolic polynomial A.
     * `proof_constant_time_execution`: Prove absence of branch instructions depending on secret buffer data.
   - Verify against official NIST ACVP (Automated Cryptographic Validation Protocol) Known Answer Test (KAT) vectors for FIPS 203 and FIPS 204.

3. Cross-Compilation Validation:
   - Verify zero-error compilation across target architectures:
     * `thumbv7em-none-eabi` (ARM Cortex-M4/M7)
     * `riscv32imac-unknown-none-elf` (RISC-V)

4. CI/CD & Delivery:
   - Construct `.github/workflows/ci.yml` running clippy, ACVP KAT verification tests, cross-compilation target builds, and Kani bounded model checking.
   - Stage, commit using structured git convention (`feat(sec): implement proprietary constant-time #![no_std] NTT engine for FIPS 203/204`), and push to `Core_Sec_NTT`.
