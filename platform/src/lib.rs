// Copyright 2024 RISC Zero, Inc.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use risc0_zkvm_platform::fileno;
use risc0_zkvm_platform::syscall::{sys_halt, sys_read, sys_write};

/// Exit the zkvm
///
#[no_mangle]
pub unsafe extern "C" fn env_exit(exit_code: u8) -> ! {
    let output_words: [u32; 8] = [0_32; 8];
    sys_halt(exit_code, &output_words)
}

/// Write data to the journal
///
/// # Safety
/// This is safe assuming that pointers have not been manually modified, and len does not go past
/// the buffer of the data in memory.
#[no_mangle]
pub unsafe extern "C" fn env_commit(bytes_ptr: *const u8, len: u32) {
    sys_write(fileno::JOURNAL, bytes_ptr, len as usize);
}

/// Write data to stdout
///
/// # Safety
/// This is safe assuming that pointers have not been manually modified, and len does not go past
/// the buffer of the data in memory.
#[no_mangle]
pub unsafe extern "C" fn env_print(bytes_ptr: *const u8, len: u32) {
    sys_write(fileno::STDOUT, bytes_ptr, len as usize);
}

/// Reads `len` bytes into buffer from the host.
///
/// # Safety
/// Assumes that the buffer has at least `len` bytes allocated.
#[no_mangle]
pub unsafe extern "C" fn env_read(bytes_ptr: *mut u8, len: u32) -> usize {
    sys_read(fileno::STDIN, bytes_ptr, len as usize)
}
