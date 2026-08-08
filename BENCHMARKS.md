# Benchmark and Size Metrics - Core_Sec_NTT

This report documents the static size and stack footprint metrics of `Core_Sec_NTT` bare-metal embedded library for different target platforms.

## 1. Static Compiled Binary Sizes (`libcore_sec_emb.rlib`)

- **ARM Cortex-M4/M7 (`thumbv7em-none-eabi`)**:
  - Raw Release `.rlib` size: **22 KB** (including full debugging symbols and metadata)
  - Estimated stripped `.a` footprint: **< 3.0 KB** of code section (.text)

- **RISC-V (`riscv32imac-unknown-none-elf`)**:
  - Raw Release `.rlib` size: **22 KB**
  - Estimated stripped `.a` footprint: **< 3.0 KB**

## 2. Stack Footprint Analysis

To ensure bare-metal compatibility for hardware enclaves with restricted memory resources, our transformation kernels operate entirely in-place and are strictly bounded on stack frame sizes:

| Function | Parameter Size (Input) | Stack Footprint (Estimated) | Allocation Overhead |
| :--- | :--- | :--- | :--- |
| `safe_ntt_forward_kem` | `[u32; 256]` (1024 B) | **~1.1 KB** | **0 bytes** (O(1) Heap) |
| `safe_ntt_inverse_kem` | `[u32; 256]` (1024 B) | **~1.1 KB** | **0 bytes** (O(1) Heap) |
| `safe_ntt_forward_dsa` | `[u32; 256]` (1024 B) | **~1.1 KB** | **0 bytes** (O(1) Heap) |
| `safe_ntt_inverse_dsa` | `[u32; 256]` (1024 B) | **~1.1 KB** | **0 bytes** (O(1) Heap) |

All stack footprints are strictly `< 2.0 KB`, making them ideal for integration inside security enclaves and deep embedded enclaves.
