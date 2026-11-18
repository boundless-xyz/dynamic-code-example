> **Archived:** This repository is no longer maintained and is preserved for reference. For context on the Boundless ZK proving protocol pause and transition, see the [Boundless Foundation announcement](https://boundlessfdn.org/blog/zkc-the-ai-access-coin).

# RISC Zero Dynamic Loading Example

Builds a dynamic library from C code and loads and executes it at runtime in the ZKVM

> [!WARNING]  
> Loading unauthorized code is dangerous as by default it executes with kernel level permissions.
> Ensure loaded code is whitelisted and verified in some way otherwise it is easy to generate arbitrary valid proofs

## Overview

- [dylib/lib.c](./dylib/lib.c) This is the lib to be built dynamically loaded into the ZKVM. Currently just a simple function that adds two args
- [methods/guest/src/main.rs](./methods/guest/src/main.rs) The ZKVM guest code. This loads the dynlib then jumps into it using an unsafe function pointer
- [host/src/main.rs](./host/src/main.rs) The host code that runs locally. This boots up the ZKVM with the given guest and sends it the dylib via the input

## Prerequisites

- [Rust and Risc Zero toolchain](https://github.com/risc0/risc0/?tab=readme-ov-file#getting-started) 
- Make
- [cbindgen](https://github.com/mozilla/cbindgen)

Cross compilation toolchain will be installed in the repo directory by the makefile

## Build and run

```shell
make
```

This will:

- Download the RISCV cross-compilation toolchain for your platform
- Build the ZKVM platform library (dylib links this to get ZKVM specific symbols)
- Build the dylib C code
- Build the guest code 
- Run the Host binary which runs the guest inside the ZKVM and provides the dylib as input

By default this runs in dev mode (no proof generated). You can also generate a proof with

```shell
make prove
```

