# Core_Sec_NTT

> **Formally Verified, Branchless Number Theoretic Transform (NTT) Engine for NIST FIPS 203 (ML-KEM) & FIPS 204 (ML-DSA) Post-Quantum Cryptography**

[![Language: Rust](https://img.shields.io/badge/Language-Rust-orange.svg)](https://www.rust-lang.org/)
[![License: CC0 1.0](https://img.shields.io/badge/License-CC0_1.0-blue.svg)](https://creativecommons.org/publicdomain/zero/1.0/)
[![no_std](https://img.shields.io/badge/no__std-compatible-brightgreen.svg)](#architecture)
[![Verification: Kani](https://img.shields.io/badge/Formal_Verification-Kani-purple.svg)](https://model-checking.github.io/kani/)

---

## Overview

**`Core_Sec_NTT`** is a high-assurance, bare-metal (`#![no_std]`) cryptographic arithmetic engine implementing constant-time Number Theoretic Transform (NTT) polynomial operations. Tailored specifically for **NIST FIPS 203 (ML-KEM)** and **FIPS 204 (ML-DSA)**, `Core_Sec_NTT` eliminates timing side-channel risks through strictly branchless Barrett and Montgomery reduction kernels.

The workspace is engineered with zero dynamic memory allocation ($\mathcal{O}(1)$ heap footprint) and evaluated under the **Kani Symbolic Model Checker** to formally prove arithmetic bounds, panic freedom, and timing invariance across embedded enclaves (ARM Cortex-M and RISC-V).

---

## Workspace Architecture

The repository is structured as a modular Rust workspace:

```text
Core_Sec_NTT/
├── core_sec_field/    # Branchless prime field arithmetic (q=3329, q=8380417)
├── core_sec_ntt/      # CT Cooley-Tukey & GS INTT butterfly kernels & static .rodata Zetas
└── core_sec_emb/      # Safe bare-metal wrappers, ACVP KAT tests, and Kani harnesses
