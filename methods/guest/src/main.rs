use risc0_zkvm::guest::env;

type EntryFnType = unsafe extern "C" fn(i32, i32) -> i32;

fn main() {
    let dylib_text = env::read_frame();

    // Assuming we know the offset to my_function in the object file
    // Note: According to Austin we could also use a custom linker script to fix this ahead of time
    let offset_to_my_function: usize = 0x00000000; // Example offset, I observed it using objdump

    let func_ptr: EntryFnType =
        unsafe { std::mem::transmute(dylib_text.as_ptr().add(offset_to_my_function)) };

    // Call the function
    let res = unsafe {
        func_ptr(5, 9) // Call the function at the resolved address
    };

    env::write(&format!("Dylib result: {}\n", res).as_bytes());

    env::commit(&res.to_be_bytes());
}
