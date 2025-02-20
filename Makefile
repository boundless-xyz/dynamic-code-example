ROOT_DIR:=$(strip $(dir $(realpath $(lastword $(MAKEFILE_LIST)))))

ifeq ($(OS),Windows_NT)
$(error Windows not supported)
else
    UNAME_S := $(shell uname -s)
    ifeq ($(UNAME_S),Linux)
        MACHINE = linux
    endif
    ifeq ($(UNAME_S),Darwin)
        MACHINE = osx
    endif

    UNAME_P := $(shell uname -p)
    ifeq ($(UNAME_P),x86_64)
        MACHINE := $(MACHINE)-x86_64
    endif
    ifneq ($(filter arm%,$(UNAME_P)),)
        MACHINE := $(MACHINE)-arm64
    endif
endif

default: execute

.PHONY: platform
platform:
	cargo +risc0 rustc -p zkvm-platform --target riscv32im-risc0-zkvm-elf --lib --crate-type staticlib --release --target-dir ./dylib/out/platform

# Note: this is currently broken on main, as there is a FFI incompatible type
# Regenerate the header file. Assumes cbindgen installed locally.
# .PHONY: headers
# headers:
# 	cbindgen --crate zkvm-platform -c ./platform/cbindgen.toml -o ./dylib/platform.h

.PHONY: gcc
gcc:
	@if ! [ -d "./riscv32im-${MACHINE}" ]; then \
        curl -L https://github.com/risc0/toolchain/releases/download/2024.01.05/riscv32im-${MACHINE}.tar.xz | tar xvJ -C ./; \
	else \
		echo "riscv32 toolchain already exists, skipping step"; \
    fi
	 
.PHONY: dylib
dylib: gcc platform
	${ROOT_DIR}riscv32im-${MACHINE}/bin/riscv32-unknown-elf-gcc -fPIC -c ./dylib/lib.c -o ./dylib/out/lib.o -L${ROOT_DIR}dylib/out/platform/riscv32im-risc0-zkvm-elf/release -lzkvm_platform

.PHONY: execute
execute: dylib
	RISC0_DEV_MODE=true cargo run -p host

.PHONY: prove
prove: dylib
	cargo run -p host
