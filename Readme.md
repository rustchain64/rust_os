# Refernece
os.phil-opp.com/freestanding-rust-binary/

# To build without error have no underlying os
rustup target add thumbv7em-none-eabihf
cargo build --target thumbv7em-none-eabihf

# custom target
## Linux
cargo rustc -- -C link-arg=-nostartfiles
## Mac OS
cargo rustc -- -C link-arg="-e __start -static -nostartfiles"

## rust_os.json
    "data-layout": "e-m:e-i64:64-f80:128-n8:16:32:64-S128",

## features
The mmx and sse features determine support for Single Instruction Multiple Data (SIMD) instructions, which can often speed up programs significantly. However, using the large SIMD registers in OS kernels leads to performance problems. The reason is that the kernel needs to restore all registers to their original state before continuing an interrupted program. 

This means that the kernel has to save the complete SIMD state to main memory on each system call or hardware interrupt. Since the SIMD state is very large (512–1600 bytes) and interrupts can occur very often, these additional save/restore operations considerably harm performance. To avoid this, we disable SIMD for our kernel (not for applications running on top!).

A problem with disabling SIMD is that floating point operations on x86_64 require SIMD registers by default. To solve this problem, we add the soft-float feature, which emulates all floating point operations through software functions based on normal integers.

     "features": "-mmx,-sse,+soft-float"
