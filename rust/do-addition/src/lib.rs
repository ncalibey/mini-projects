// `no_mangle` tells the compiler we want the name of the function to be `add` in the
// final binary.
#[no_mangle]
// `extern "C"` means we want the function to use the right calling conventions that
// are understood by WASM.
pub extern "C" fn add(a: u32, b: u32) -> u32 {
    a + b
}
