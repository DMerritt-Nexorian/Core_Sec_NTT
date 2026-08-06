# AGENTS.md — System Instructions for Core_Sec_NTT

**Target Repository:** Core_Sec_NTT  
**Author / Entity:** Dennis W. Merritt / Nexorian Corporation  
**License:** CC0 1.0 Universal  
**Target Standard:** NIST FIPS 203 (ML-KEM) & FIPS 204 (ML-DSA)  

---

## 1. FIRST-PRINCIPLE DECONSTRUCTION & HARD BOUNDARIES

All code generated or modified within this repository MUST strictly abide by these three non-negotiable operational boundaries:

### Rule 1: Proof Before Trust
* **Zero Trust:** Never trust compiler auto-vectorization, implicit timing invariance, or unverified memory access.
* **Formal Proofs Required:** All NTT butterfly operations and modular polynomial arithmetic must be formally proven panic-free, overflow-free, and constant-time using symbolic model checking via Kani (`#[kani::proof]`).
* **Static Bounds:** Every input vector and intermediate state must undergo static, type-level bounds checking before execution in NTT compute stages.

### Rule 2: Rules Before Reasoning
* **Strict Constraints:** Absolute adherence to `#![no_std]`, `#![deny(unsafe_code)]`, and strict constant-time execution paths supersedes dynamic runtime optimizations or branching heuristics.
* **NIST Compliance:** Exact compliance with NIST FIPS 203 (ML-KEM, $q=3329$, $n=256$) and FIPS 204 (ML-DSA, $q=8380417$, $n=256$) mathematical specs is mandatory.
* **Branchless Reductions:** Montgomery and Barrett reduction invariants ($0 \le r < q$) must hold at every stage without conditional execution branches (`if/else`) dependent on secret data.

### Rule 3: Determinism Before Autonomy
* **Constant-Time Execution:** Execution time must be strictly $\mathcal{O}(N \log N)$ with 0-cycle variance across all secret coefficient inputs to eliminate side-channel timing attacks.
* **Cross-Compilation:** Enforce bit-level deterministic cross-compilation target validation for embedded security enclaves (`thumbv7em-none-eabi` and `riscv32imac-unknown-none-elf`).

---

## 2. REPOSITORY ARCHITECTURE & CRATES

Create and maintain a clean Rust workspace with three distinct crates:

1. **`core_sec_field`** (Modular Field Arithmetic)
   * Implements dual-prime Montgomery & Barrett reduction kernels for:
     * ML-KEM prime: $q = 3329$, $n = 256$, primitive 256-th root of unity $\omega = 17$.
     * ML-DSA prime: $q = 8380417$, $n = 256$, primitive 256-th root of unity $\omega = 1753$.
   * Enforces complete branchless routines (`sub_pick`, `montgomery_reduce`, `barrett_reduce`) utilizing bitwise masking and constant-time selection.
   * Exposes type-safe wrapper structs `FieldElement<const Q: u32>` enforcing static boundary bounds at compile time.

2. **`core_sec_ntt`** (NTT Transformation Kernel)
   * Implements Forward NTT (Cooley-Tukey butterfly) mapping polynomial $A(X) \in R_q$ to point-value representation in constant time.
   * Implements Inverse NTT (Gentleman-Sande butterfly) mapping back to polynomial representation with exact division by $n$ in $\mathbb{F}_q$.
   * Implements Pointwise Modular Multiplication engine: $C_i = (A_i \cdot B_i) \pmod q$ in $\mathcal{O}(N)$ constant time.
   * Pre-computes and stores bit-reversed twiddle factor lookup tables in static `.rodata` memory with zero dynamic allocation (`O(1)` heap footprint).

3. **`core_sec_emb`** (Embedded Hardware Abstraction & Verification)
   * Exposes bare-metal FFI/Rust bindings for ARM Cortex-M4/M7 and RISC-V targets.
   * Contains Kani formal verification harnesses and NIST ACVP Known Answer Test (KAT) suites.

---

## 3. COMPILATION & CONFIGURATION REQUIREMENTS

### Root `Cargo.toml` Setup
```toml
[workspace]
members = [
    "core_sec_field",
    "core_sec_ntt",
    "core_sec_emb",
]
resolver = "2"

[profile.release]
opt-level = 3
lto = true
codegen-units = 1
panic = "abort"
